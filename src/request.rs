use core::borrow::Borrow;
use std::cell::{RefCell, RefMut};
use std::pin::Pin;
use std::rc::Rc;
use std::task::Poll;
use std::task::RawWakerVTable;
use std::task::{Context, RawWaker, Waker};

#[derive(Debug)]
pub enum QR<Q, R> {
    Query(Q),
    Response(R),
}

pub type CtrlId<MSG, EVT> = fn(EVT) -> MSG;

pub trait Unpack<EVT: Default> {
    fn unpack(self, ctrl: CtrlId<Self, EVT>) -> Result<EVT, Self>
    where
        Self: Sized;
    fn peek(&self, ctrl: CtrlId<Self, EVT>) -> Option<&EVT>;
}

pub trait MessagePoolIn<MSG> {
    fn drain(&mut self, f: &Fn(&MSG) -> bool) -> Vec<MSG>;
    fn query(&self, f: &Fn(&MSG) -> bool) -> Vec<&MSG>;
    fn is_empty(&self) -> bool;
    fn clear(&mut self);
}

pub trait MessagePoolOut<MSG> {
    fn push(&mut self, m: MSG);
}

impl<MSG> MessagePoolIn<MSG> for Vec<MSG> {
    fn drain(&mut self, f: &Fn(&MSG) -> bool) -> Vec<MSG> {
        self.drain_filter(|m| f(m)).collect()
    }
    fn query(&self, f: &Fn(&MSG) -> bool) -> Vec<&MSG> {
        self.iter().filter(|m| f(m)).collect()
    }
    fn is_empty(&self) -> bool {
        (self as &Vec<MSG>).is_empty()
    }
    fn clear(&mut self) {
        (self as &mut Vec<MSG>).clear()
    }
}

impl<MSG> MessagePoolOut<MSG> for Vec<MSG> {
    fn push(&mut self, m: MSG) {
        self.push(m)
    }
}

pub trait MessagePool<MSG>: MessagePoolIn<MSG> + MessagePoolOut<MSG> {}
impl<T, MSG> MessagePool<MSG> for T where T: MessagePoolIn<MSG> + MessagePoolOut<MSG> {}

pub trait MessageHandler<MSG> {
    fn handle(&mut self, from: &mut MessagePoolIn<MSG>, to: &mut MessagePoolOut<MSG>);
}

pub fn query_by_ctrlid<EVT: Default, MSG: Unpack<EVT>>(
    pool: &mut MessagePoolIn<MSG>,
    ctrl: CtrlId<MSG, EVT>,
) -> Vec<EVT> {
    let msgs = pool.drain(&|msg: &MSG| msg.peek(ctrl).is_some());
    msgs.into_iter()
        .filter_map(|msg| msg.unpack(ctrl).ok())
        .collect()
}

pub fn message_loop<MSG, POOL: MessagePool<MSG> + Default>(
    handler: &mut MessageHandler<MSG>,
    mut src: POOL,
) {
    loop {
        let mut dst = POOL::default();
        handler.handle(&mut src, &mut dst);
        if dst.is_empty() {
            break;
        }
        std::mem::swap(&mut src, &mut dst);
    }
}

const DUMMY_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
    dummy_waker_clone,
    dummy_waker_wake,
    dummy_waker_wake_by_ref,
    dummy_waker_drop,
);

unsafe fn dummy_waker_clone(data: *const ()) -> RawWaker {
    RawWaker::new(data, &DUMMY_WAKER_VTABLE)
}

unsafe fn dummy_waker_wake(data: *const ()) {}

unsafe fn dummy_waker_wake_by_ref(data: *const ()) {}

unsafe fn dummy_waker_drop(_data: *const ()) {}

fn new_dummy_waker() -> Waker {
    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &DUMMY_WAKER_VTABLE)) }
}

/*

struct MessagePoolFuture<'a, MSG, POOL, F, T>
where
    POOL: MessagePool<MSG> + Default,
    F: Fn(&mut MessagePool<MSG>) -> Option<T>
{
    shared_state: Rc<SharedState<'a, MSG, POOL>>,
    f: F
}


impl<'a,MSG,POOL,F,T> MessagePoolFuture<'a, MSG, POOl,F,T>
    where
        POOL: MessagePool<MSG> + Default,
        F: Fn(&mut MessagePool<MSG>) -> Option<T>
{
   pub fn new(f: F) -> Self {

   }
}

//impl<MSG,F,T> Unpin for MessagePoolFuture<'_,MSG,F,T> {}

impl<MSG, POOL> std::future::Future for MessagePoolFuture<'_, MSG, POOL>
where
    POOL: MessagePool<MSG> + Default
{
    type Output = T;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(shared_state) = Rc::get_mut(&mut self.shared_state) {

        } else {
            Poll::Pending
        }
    }
}

struct SharedState<'a, MSG, POOL>
where
    POOL: MessagePool<MSG> + Default,
{
    handler: &'a mut MessageHandler<MSG>,
    pool: POOL,
}
*/

struct MessageRouterFuture<MSG, POOL: MessagePool<MSG> + Default> {
    pool: Rc<RefCell<POOL>>,
    phantom: std::marker::PhantomData<MSG>,
}

struct MessageRouterAsync<'a, MSG, POOL: MessagePool<MSG> + Default> {
    pool: Rc<RefCell<POOL>>,
    handler: &'a mut MessageHandler<MSG>,
}

impl<'a, MSG, POOL> MessageRouterAsync<'a, MSG, POOL>
where
    POOL: MessagePool<MSG> + Default,
{
    pub fn new(handler: &'a mut MessageHandler<MSG>) -> MessageRouterAsync<'a, MSG, POOL> {
        Self {
            pool: Rc::new(RefCell::new(POOL::default())),
            handler,
        }
    }

    pub fn request_sync<EVT, Q, R>(
        &mut self,
        ctrlid: CtrlId<MSG, EVT>,
        evtisr: fn(&EVT) -> bool,
        evttor: fn(EVT) -> R,
        evtq: EVT,
    ) -> Option<R>
    where
        EVT: Default,
        MSG: Unpack<EVT>,
    {
        // Filter only responses
        let mut pool: RefMut<POOL> = self.pool.borrow_mut();
        let ctrl_evts = pool.drain(&|m| match m.peek(ctrlid) {
            Some(evt) => evtisr(evt),
            None => false,
        });
        // Either return response or push query to message pool
        if let Some(evt) = ctrl_evts.pop() {
            Ok(evttor(evt))
        } else {
            pool.push(ctrlid(evtq));
            None
        }
    }
    /*
    pub fn request<EVT, MSG: Unpack<EVT>, Q, R>(
        &mut self,
        ctrlid: CtrlId<MSG, EVT>,
        evtisr: fn(&EVT) -> bool,
        evttor: fn(EVT) -> R,
        evtq: EVT,
    ) -> R {
        if let Some(r) = request_sync(self, ctrlid, evtisr, evttor, evtq) {
            r
        } else {

        }
    }
    */
}
