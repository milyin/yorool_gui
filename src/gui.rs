pub mod button;
pub mod checkbox;
pub mod panel;
pub mod radio_group;
pub mod ribbon;
pub mod window_manager;

use ggez::event::EventHandler;
use ggez::graphics::Rect;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type Handler<'a, T> = Rc<dyn Fn(Rc<RefCell<T>>) + 'a>;

pub type HandlerId = u64;

pub fn handler_id<'a, T: ?Sized>(h: Handler<'a, T>) -> HandlerId {
    h.as_ref() as *const _ as *const () as HandlerId
}

pub trait TRcSelf {
    fn create() -> Rc<RefCell<Self>>;
    fn wrcself(&self) -> Weak<RefCell<Self>>;
    fn rcself(&self) -> Rc<RefCell<Self>> {
        self.wrcself().upgrade().unwrap()
    }
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

pub trait IActions<'a> {
    fn collect_fired(&mut self) -> Vec<Rc<dyn Fn() + 'a>>;
}

pub trait ILayout {
    fn set_rect(&mut self, rect: Rect);
    fn get_rect(&self) -> Rect;
}
pub trait Widget<'a>: EventHandler + ILayout + IActions<'a> {}

impl<'a, W> Widget<'a> for W where W: EventHandler + ILayout + IActions<'a> {}

pub trait ICheckbox<'a> {
    fn get_state(&self) -> bool;
    fn set_state(&mut self, state: bool);
    fn on_changed_rc(&mut self, handler: Rc<dyn Fn(Rc<RefCell<dyn ICheckbox<'a> + 'a>>) + 'a>);
    fn remove_handler(&mut self, handler: Rc<dyn Fn(Rc<RefCell<dyn ICheckbox<'a> + 'a>>) + 'a>);
}

pub trait ILabel<'a> {
    fn get_label(&self) -> String;
    fn set_label(&mut self, label: String);
}

pub type ButtonBuilder<'a> =
    button::Builder<'a, button::Backend<'a>, button::Frontend<'a, button::Backend<'a>>>;

pub fn button<'a>() -> ButtonBuilder<'a> {
    ButtonBuilder::new()
}

pub type RibbonBuilder<'a> = ribbon::Builder<'a>;

pub fn ribbon<'a>() -> RibbonBuilder<'a> {
    RibbonBuilder::new()
}

pub fn row<'a>() -> RibbonBuilder<'a> {
    RibbonBuilder::new().set_horizontal(true)
}

pub fn column<'a>() -> RibbonBuilder<'a> {
    RibbonBuilder::new().set_horizontal(false)
}

pub type PanelBuilder<'a> = panel::Builder<'a>;

pub fn panel<'a>() -> PanelBuilder<'a> {
    PanelBuilder::new()
}

// Accordingly to discussions below there is still no api to compare only
// data part of fat pointers. So using own api for now
// https://github.com/rust-lang/rust/issues/63021
// https://github.com/rust-lang/rust/pull/48814
// https://github.com/rust-lang/rust/issues/48795
pub fn is_same<A: ?Sized, B: ?Sized>(a: &Rc<A>, b: &Rc<B>) -> bool {
    let pda = a.as_ref() as *const _ as *const ();
    let pdb = b.as_ref() as *const _ as *const ();
    return pda == pdb;
}
