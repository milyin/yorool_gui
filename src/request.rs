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
