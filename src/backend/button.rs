use crate::backend::{Handlers, IButton, IHandlers, ILabel};
use crate::gui::{Handler, HandlerId, TRcSelf};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub trait IBackend<'a>: IHandlers<'a> + IButton<'a> + ILabel<'a> {}

impl<'a, T> IBackend<'a> for T where T: IHandlers<'a> + IButton<'a> + ILabel<'a> {}

pub struct Backend<'a> {
    label: String,
    touched: bool,
    on_click_handlers: Vec<HandlerId>,
    handlers: Handlers<'a>,
    rcself: Option<Weak<RefCell<Self>>>,
}

impl TRcSelf for Backend<'_> {
    fn create() -> Rc<RefCell<Self>> {
        let v = Rc::new(RefCell::new(Self {
            label: String::new(),
            touched: false,
            on_click_handlers: Vec::new(),
            handlers: Handlers::default(),
            rcself: None,
        }));
        v.borrow_mut().rcself = Some(Rc::downgrade(&v.clone()));
        v
    }
    fn wrcself(&self) -> Weak<RefCell<Self>> {
        self.rcself.as_ref().unwrap().clone()
    }
}

impl<'a> IHandlers<'a> for Backend<'a> {
    fn remove_handler(&mut self, hid: HandlerId) {
        self.handlers.remove_handler(hid)
    }
    fn collect_fired_handlers(&mut self) -> Vec<Rc<dyn Fn() + 'a>> {
        self.handlers.collect_fired_handlers()
    }
}

impl<'a> IButton<'a> for Backend<'a> {
    fn set_touched(&mut self, state: bool) {
        self.touched = state
    }
    fn is_touched(&self) -> bool {
        self.touched
    }
    fn click(&mut self) {
        self.handlers.fire_handlers(&self.on_click_handlers)
    }
    fn on_click(&mut self, hid: HandlerId, h: Handler<'a, dyn IButton<'a> + 'a>) {
        self.on_click_handlers.push(hid);
        self.handlers.add_handler(hid, h, self.wrcself());
    }
}

impl<'a> ILabel<'a> for Backend<'a> {
    fn get_label(&self) -> String {
        self.label.clone()
    }
    fn set_label(&mut self, label: String) {
        self.label = label;
    }
}
