pub mod button;
pub mod checkbox;
pub mod panel;
pub mod ribbon;

use crate::request::MessageSender;
use ggez::event::EventHandler;
use std::future::Future;

pub trait Layoutable {
    fn set_rect(&mut self, x: f32, y: f32, w: f32, h: f32);
}
pub trait Widget<MSG>: MessageSender<MSG> + EventHandler + Layoutable {
    fn as_message_sender(&mut self) -> &mut dyn MessageSender<MSG>;
}

impl<W, MSG> Widget<MSG> for W
where
    W: MessageSender<MSG> + EventHandler + Layoutable,
{
    fn as_message_sender(&mut self) -> &mut dyn MessageSender<MSG> {
        self
    }
}
