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

pub fn get_response<MSG, POOL, EVT, R>(
    pool: &mut POOL,
    ctrlid: CtrlId<MSG, EVT>,
    evtisr: fn(&EVT) -> bool,
    evttor: fn(EVT) -> R,
) -> Option<R>
where
    EVT: Default,
    MSG: Unpack<EVT>,
    POOL: MessagePool<MSG>,
{
    // Extract matching responses
    let mut ctrl_evts = pool.drain(&|m| match m.peek(ctrlid) {
        Some(evt) => evtisr(evt),
        None => false,
    });
    // Return last response if found
    if let Some(msg) = ctrl_evts.pop() {
        Some(evttor(msg.unpack(ctrlid).ok().unwrap())) // it contains EVT - by filter in drain
    } else {
        None
    }
}

struct MessageRouterFuture<MSG, POOL, EVT, R>
where
    POOL: MessagePool<MSG>,
    MSG: Unpack<EVT>,
    EVT: Default,
{
    pool: Rc<RefCell<POOL>>,
    ctrlid: CtrlId<MSG, EVT>,
    evtisr: fn(&EVT) -> bool,
    evttor: fn(EVT) -> R,
}

impl<MSG, POOL, EVT, R> MessageRouterFuture<MSG, POOL, EVT, R>
where
    POOL: MessagePool<MSG>,
    MSG: Unpack<EVT>,
    EVT: Default,
{
}

impl<MSG, POOL, EVT, R> std::future::Future for MessageRouterFuture<MSG, POOL, EVT, R>
where
    POOL: MessagePool<MSG>,
    MSG: Unpack<EVT>,
    EVT: Default,
{
    type Output = R;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut pool = self.pool.borrow_mut();
        if let Some(r) = get_response(&mut *pool, self.ctrlid, self.evtisr, self.evttor) {
            Poll::Ready(r)
        } else {
            Poll::Pending
        }
    }
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

    pub async fn request<EVT, Q, R>(
        &mut self,
        ctrlid: CtrlId<MSG, EVT>,
        evtisr: fn(&EVT) -> bool,
        evttor: fn(EVT) -> R,
        evtq: EVT,
    ) -> R
    where
        EVT: Default,
        MSG: Unpack<EVT>,
    {
        {
            let mut pool = self.pool.borrow_mut();
            if let Some(r) = get_response(&mut *pool, ctrlid, evtisr, evttor) {
                return r;
            }
            pool.push(ctrlid(evtq));
        }
        MessageRouterFuture {
            pool: self.pool.clone(),
            ctrlid,
            evtisr,
            evttor,
        }
        .await
    }
}
