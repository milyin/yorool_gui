use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::Poll;
use std::task::RawWakerVTable;
use std::task::{Context, RawWaker, Waker};

pub struct CtrlId<MSG, EVT>(fn(EVT) -> MSG);

impl<MSG, EVT> Copy for CtrlId<MSG, EVT> {}

impl<MSG, EVT> Clone for CtrlId<MSG, EVT> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<MSG, EVT> From<fn(EVT) -> MSG> for CtrlId<MSG, EVT> {
    fn from(f: fn(EVT) -> MSG) -> Self {
        Self(f)
    }
}

impl<MSG, EVT> CtrlId<MSG, EVT> {
    pub fn tomsg(&self, evt: EVT) -> MSG {
        (self.0)(evt)
    }
}

impl<MSG, EVT> PartialEq for CtrlId<MSG, EVT>
where
    MSG: Unpack<EVT>,
    EVT: Default,
{
    fn eq(&self, other: &Self) -> bool {
        return self.tomsg(EVT::default()).peek(*other).is_some();
    }
}

pub trait Unpack<EVT: Default> {
    fn unpack(self, ctrlid: CtrlId<Self, EVT>) -> Result<EVT, Self>
    where
        Self: Sized;
    fn peek(&self, ctrlid: CtrlId<Self, EVT>) -> Option<&EVT>
    where
        Self: Sized;
}

pub trait MessageSender<MSG> {
    fn get_message(&mut self) -> Option<MSG> {
        None
    }
}
