pub mod button;
pub mod checkbox;
pub mod panel;
pub mod radio_group;
pub mod ribbon;
pub mod window_manager;

use ggez::event::EventHandler;
use ggez::graphics::Rect;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Executable<'a> {
    fn take_to_execute(&mut self) -> Vec<Rc<dyn Fn() + 'a>>;
}

pub trait Layoutable {
    fn set_rect(&mut self, rect: Rect);
    fn get_rect(&self) -> Rect;
}
pub trait Widget<'a>: EventHandler + Layoutable + Executable<'a> {}

impl<'a, W> Widget<'a> for W where W: EventHandler + Layoutable + Executable<'a> {}

pub trait ICheckbox<'a> {
    fn get_state(&self) -> bool;
    fn set_state(&mut self, state: bool);
    fn on_changed_rc(&mut self, handler: Rc<dyn Fn(Rc<RefCell<dyn ICheckbox<'a> + 'a>>) + 'a>);
    fn remove_handler(&mut self, handler: Rc<dyn Fn(Rc<RefCell<dyn ICheckbox<'a> + 'a>>) + 'a>);
}

// Accordincly to discussions below there is still no api to compare only
// data part of fat pointers. So using own api for now
// https://github.com/rust-lang/rust/issues/63021
// https://github.com/rust-lang/rust/pull/48814
// https://github.com/rust-lang/rust/issues/48795
pub fn is_same<A: ?Sized, B: ?Sized>(a: &Rc<A>, b: &Rc<B>) -> bool {
    let pda = a.as_ref() as *const _ as *const ();
    let pdb = b.as_ref() as *const _ as *const ();
    return pda == pdb;
}
