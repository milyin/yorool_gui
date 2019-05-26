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

pub trait MessagePool<MSG> {
    fn query(&mut self, f: &Fn(&MSG) -> bool) -> Vec<MSG>;
    fn push(&mut self, m: MSG);
}

impl<MSG> MessagePool<MSG> for Vec<MSG> {
    fn query(&mut self, f: &Fn(&MSG) -> bool) -> Vec<MSG> {
        self.drain_filter(|m| f(m)).collect()
    }
    fn push(&mut self, m: MSG) {
        self.push(m)
    }
}

pub fn query_by_ctrlid<EVT: Default, MSG: Unpack<EVT>>(
    pool: &mut MessagePool<MSG>,
    ctrl: CtrlId<MSG, EVT>,
) -> Vec<EVT> {
    let msgs = pool.query(&|msg: &MSG| msg.peek(ctrl).is_some());
    msgs.into_iter()
        .filter_map(|msg| msg.unpack(ctrl).ok())
        .collect()
}

pub trait MessageHandler<MSG> {
    fn handle(&mut self, pool: &mut MessagePool<MSG>);
}
