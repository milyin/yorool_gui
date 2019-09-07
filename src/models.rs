use crate::gui::{Executable, ILabel, Layoutable};
use std::cell::RefCell;
use std::rc::Rc;

pub type Handler<'a, T> = Rc<dyn Fn(Rc<RefCell<T>>) + 'a>;

pub type HandlerId = u64;

pub fn handler_id<'a, T: ?Sized>(h: Handler<'a, T>) -> HandlerId {
    h.as_ref() as *const _ as *const () as HandlerId
}

pub trait TRcSelf {
    fn create() -> Rc<RefCell<Self>>;
    fn rcself(&self) -> Rc<RefCell<Self>>;
}

pub trait THandlers<'a>: TRcSelf {
    fn collect_fired_handlers(&mut self) -> Vec<Handler<'a, Self>>;
    fn remove_handler(&mut self, hid: HandlerId);
}

pub fn collect_fired_actions<'a, T: THandlers<'a> + 'a>(widget: &mut T) -> Vec<Rc<dyn Fn() + 'a>> {
    widget
        .collect_fired_handlers()
        .into_iter()
        .map(|h| {
            let rcself = widget.rcself().clone();
            let rh = h.clone();
            let f = move || rh(rcself.clone());
            Rc::new(f) as Rc<dyn Fn() + 'a>
        })
        .collect()
}

pub trait TButtonBackend<'a>: THandlers<'a> + ILabel<'a> {
    fn set_touched(&mut self, state: bool);
    fn is_touched(&self) -> bool;
    fn click(&mut self);
    fn on_click(&mut self, hid: HandlerId, h: Handler<'a, Self>);
}

pub trait TButtonFrontend<'a, BE: TButtonBackend<'a>>:
    TRcSelf + Layoutable + Executable<'a>
{
    fn backend(&self) -> Rc<RefCell<BE>>;
}

pub trait IButton<'a> {
    fn set_touched(&mut self, state: bool);
    fn is_touched(&self) -> bool;
    fn click(&mut self);
    fn on_click(&mut self, h: Handler<'a, dyn IButton<'a> + 'a>);
}

impl<'a, W> IButton<'a> for W
where
    W: TButtonBackend<'a> + 'a,
{
    fn set_touched(&mut self, state: bool) {
        self.set_touched(state)
    }
    fn is_touched(&self) -> bool {
        self.is_touched()
    }
    fn click(&mut self) {
        self.click()
    }
    fn on_click(&mut self, h: Handler<'a, dyn IButton<'a> + 'a>) {
        self.on_click(handler_id(h.clone()), {
            let rh = h.clone();
            Rc::new(move |w| rh(w.clone()))
        });
    }
}
