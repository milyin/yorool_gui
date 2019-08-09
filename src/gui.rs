pub mod button;
pub mod checkbox;
pub mod panel;
pub mod ribbon;

use crate::request::MessageSender;
use ggez::event::EventHandler;
use std::rc::Rc;

pub trait Executable<'a> {
    fn to_execute(&mut self) -> Vec<Rc<dyn Fn() + 'a>>;
}

pub trait Layoutable {
    fn set_rect(&mut self, x: f32, y: f32, w: f32, h: f32);
}
pub trait Widget<'a, MSG>: MessageSender<MSG> + EventHandler + Layoutable + Executable<'a> {
    fn as_message_sender(&mut self) -> &mut dyn MessageSender<MSG>;
}

impl<'a, W, MSG> Widget<'a, MSG> for W
where
    W: MessageSender<MSG> + EventHandler + Layoutable + Executable<'a>,
{
    fn as_message_sender(&mut self) -> &mut dyn MessageSender<MSG> {
        self
    }
}
