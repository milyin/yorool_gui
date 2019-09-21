use crate::gui::{Handler, HandlerId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub mod button;

pub trait IHandlers<'a> {
    fn remove_handler(&mut self, hid: HandlerId);
    fn collect_fired_handlers(&mut self) -> Vec<Rc<dyn Fn() + 'a>>;
}

pub trait IButton<'a> {
    fn set_touched(&mut self, state: bool);
    fn is_touched(&self) -> bool;
    fn click(&mut self);
    fn on_click(&mut self, hid: HandlerId, h: Handler<'a, dyn IButton<'a> + 'a>);
}

pub trait ILabel<'a> {
    fn get_label(&self) -> String;
    fn set_label(&mut self, label: String);
}

#[derive(Default)]
pub struct Handlers<'a> {
    stored: HashMap<HandlerId, Rc<dyn Fn() + 'a>>,
    pending: Vec<Rc<dyn Fn() + 'a>>,
}

impl<'a> Handlers<'a> {
    fn fire_handler(&mut self, hid: HandlerId) {
        let h = self.stored.get(&hid).unwrap().clone();
        self.pending.push(Rc::new(move || h()))
    }
    fn fire_handlers<HIDS>(&mut self, hids: &HIDS)
    where
        for<'b> &'b HIDS: IntoIterator<Item = &'b HandlerId>,
    {
        for hid in hids.into_iter() {
            self.fire_handler(*hid)
        }
    }
    fn add_handler<T: 'a + ?Sized>(
        &mut self,
        hid: HandlerId,
        h: Handler<'a, T>,
        wt: Weak<RefCell<T>>,
    ) {
        self.stored.insert(
            hid,
            Rc::new(move || {
                if let Some(rt) = wt.upgrade() {
                    h(rt)
                }
            }),
        );
    }
}

impl<'a> IHandlers<'a> for Handlers<'a> {
    fn remove_handler(&mut self, hid: HandlerId) {
        self.stored.remove(&hid);
    }
    fn collect_fired_handlers(&mut self) -> Vec<Rc<dyn Fn() + 'a>> {
        self.pending.drain(..).collect()
    }
}
