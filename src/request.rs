use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::Poll;
use std::task::RawWakerVTable;
use std::task::{Context, RawWaker, Waker};

#[derive(Debug, Clone)]
pub enum QR<Q, R>
where
    Q: Clone,
    R: Clone,
{
    Query(Q),
    Response(R),
}

impl<Q, R> Default for QR<Q, R>
where
    Q: Clone,
    R: Default + Clone,
{
    fn default() -> QR<Q, R> {
        QR::Response(R::default())
    }
}

pub type EvtId<EVT, Q, R> = fn(QR<Q, R>) -> EVT;

pub trait EvtUnpack<Q: Clone, R: Default + Clone> {
    fn make_query(evt: EvtId<Self, Q, R>, q: Q) -> Self
    where
        Self: Sized,
    {
        evt(QR::Query(q))
    }
    fn unpack_response(self, ctrl: EvtId<Self, Q, R>) -> Result<R, Self>
    where
        Self: Sized;
    fn peek_response(&self, ctrl: EvtId<Self, Q, R>) -> Option<&R>;
}

pub type CtrlId<MSG, EVT> = fn(EVT) -> MSG;

pub trait Unpack<EVT: Default> {
    fn unpack(self, ctrl: CtrlId<Self, EVT>) -> Result<EVT, Self>
    where
        Self: Sized;
    fn peek(&self, ctrl: CtrlId<Self, EVT>) -> Option<&EVT>;
}

pub trait MessagePoolIn<MSG> {
    fn drain(&mut self) -> Vec<MSG>;
    fn drain_filter(&mut self, f: &dyn Fn(&MSG) -> bool) -> Vec<MSG>;
    fn query(&self, f: &dyn Fn(&MSG) -> bool) -> Vec<&MSG>;
    fn is_empty(&self) -> bool;
    fn clear(&mut self);
}

pub trait MessagePoolOut<MSG> {
    fn push(&mut self, m: MSG);
    fn append(&mut self, other: &mut dyn MessagePoolIn<MSG>);
}

impl<MSG> MessagePoolIn<MSG> for Vec<MSG> {
    fn drain(&mut self) -> Vec<MSG> {
        self.split_off(0)
    }
    fn drain_filter(&mut self, f: &dyn Fn(&MSG) -> bool) -> Vec<MSG> {
        self.drain_filter(|m| f(m)).collect()
    }
    fn query(&self, f: &dyn Fn(&MSG) -> bool) -> Vec<&MSG> {
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
    fn append(&mut self, other: &mut dyn MessagePoolIn<MSG>) {
        self.append(&mut other.drain())
    }
}

pub trait MessagePool<MSG>: MessagePoolIn<MSG> + MessagePoolOut<MSG> {}
impl<T, MSG> MessagePool<MSG> for T where T: MessagePoolIn<MSG> + MessagePoolOut<MSG> {}

pub trait MessageHandler<MSG> {
    fn handle(&mut self, from: &mut dyn MessagePoolIn<MSG>, to: &mut dyn MessagePoolOut<MSG>);
}

pub trait MessageHandlerExecutor<MSG> {
    fn execute(&mut self, handler: &mut dyn MessageHandler<MSG>, seed: &mut dyn MessagePoolIn<MSG>);
}

pub fn query_by_ctrlid<EVT: Default, MSG: Unpack<EVT>>(
    pool: &mut dyn MessagePoolIn<MSG>,
    ctrl: CtrlId<MSG, EVT>,
) -> Vec<EVT> {
    let msgs = pool.drain_filter(&|msg: &MSG| msg.peek(ctrl).is_some());
    msgs.into_iter()
        .filter_map(|msg| msg.unpack(ctrl).ok())
        .collect()
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

unsafe fn dummy_waker_wake(_data: *const ()) {}

unsafe fn dummy_waker_wake_by_ref(_data: *const ()) {}

unsafe fn dummy_waker_drop(_data: *const ()) {}

fn new_dummy_waker() -> Waker {
    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &DUMMY_WAKER_VTABLE)) }
}

pub fn get_response<MSG, POOL, EVT, Q, R>(
    pool: &mut POOL,
    ctrlid: CtrlId<MSG, EVT>,
    evtid: EvtId<EVT, Q, R>,
) -> Option<R>
where
    EVT: Default,
    Q: Clone,
    R: Default + Clone,
    EVT: EvtUnpack<Q, R>,
    MSG: Unpack<EVT>,
    POOL: MessagePool<MSG>,
{
    let evtisr = |evt: &EVT| evt.peek_response(evtid).is_some();
    let evttor = |evt: EVT| evt.unpack_response(evtid).ok().unwrap();
    // Extract matching responses
    let mut ctrl_evts = pool.drain_filter(&|m| match m.peek(ctrlid) {
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

struct MessageRouterFuture<MSG, POOL, EVT, Q, R>
where
    Q: Clone,
    R: Default + Clone,
    EVT: Default,
    POOL: MessagePool<MSG>,
    MSG: Unpack<EVT>,
    EVT: EvtUnpack<Q, R>,
{
    pool: Rc<RefCell<POOL>>,
    ctrlid: CtrlId<MSG, EVT>,
    evtid: EvtId<EVT, Q, R>,
}

impl<MSG, POOL, EVT, Q, R> std::future::Future for MessageRouterFuture<MSG, POOL, EVT, Q, R>
where
    Q: Clone,
    R: Default + Clone,
    EVT: Default,
    POOL: MessagePool<MSG>,
    MSG: Unpack<EVT>,
    EVT: EvtUnpack<Q, R>,
{
    type Output = R;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut pool = self.pool.borrow_mut();
        if let Some(r) = get_response(&mut *pool, self.ctrlid, self.evtid) {
            Poll::Ready(r)
        } else {
            Poll::Pending
        }
    }
}

impl<MSG, POOL, EVT, Q, R> Unpin for MessageRouterFuture<MSG, POOL, EVT, Q, R>
where
    Q: Clone,
    R: Default + Clone,
    EVT: Default,
    POOL: MessagePool<MSG>,
    MSG: Unpack<EVT>,
    EVT: EvtUnpack<Q, R>,
{
}

pub struct MessageRouterAsync<MSG, POOL: MessagePool<MSG> + Default = Vec<MSG>> {
    pool: Rc<RefCell<POOL>>,
    phantom: std::marker::PhantomData<MSG>,
}

impl<MSG, POOL> MessageRouterAsync<MSG, POOL>
where
    POOL: MessagePool<MSG> + Default,
{
    pub fn new(pool: POOL) -> MessageRouterAsync<MSG, POOL> {
        Self {
            pool: Rc::new(RefCell::new(pool)),
            phantom: std::marker::PhantomData,
        }
    }

    pub async fn query<EVT, Q, R>(
        &self,
        ctrlid: CtrlId<MSG, EVT>,
        evtid: EvtId<EVT, Q, R>,
        param: Q,
    ) -> R
    where
        Q: Clone,
        R: Default + Clone,
        EVT: Default,
        MSG: Unpack<EVT>,
        EVT: EvtUnpack<Q, R>,
    {
        {
            let mut pool = self.pool.borrow_mut();
            if let Some(r) = get_response(&mut *pool, ctrlid, evtid) {
                return r;
            }
            pool.push(ctrlid(evtid(QR::Query(param))));
        }
        MessageRouterFuture {
            pool: self.pool.clone(),
            ctrlid,
            evtid,
        }
        .await
    }

    pub fn run<F: Future>(&self, handler: &mut dyn MessageHandler<MSG>, f: F) -> F::Output {
        //pub fn run<EVT, R>(&self, f: MessageRouterFuture<MSG, POOL, EVT, R>)
        //where
        //    MSG: Unpack<EVT>,
        //    EVT: Default,
        //{
        let mut future = Box::pin(f);
        loop {
            match Pin::new(&mut future).poll(&mut Context::from_waker(&new_dummy_waker())) {
                Poll::Ready(v) => return v,
                Poll::Pending => {
                    let mut src = self.pool.borrow_mut();
                    let mut dst = POOL::default();
                    handler.handle(&mut *src, &mut dst);
                    src.clear();
                    src.append(&mut dst);
                }
            }
        }
    }
}
