pub mod button;
pub mod checkbox;
pub mod panel;
pub mod ribbon;
pub mod window_manager;

use ggez::event::EventHandler;
use std::rc::Rc;

pub trait Executable<'a> {
    fn to_execute(&mut self) -> Vec<Rc<dyn Fn() + 'a>>;
}

pub trait Layoutable {
    fn set_rect(&mut self, x: f32, y: f32, w: f32, h: f32);
}
pub trait Widget<'a>: EventHandler + Layoutable + Executable<'a> {}

impl<'a, W> Widget<'a> for W where W: EventHandler + Layoutable + Executable<'a> {}
