pub mod button;
pub mod checkbox;
pub mod panel;
pub mod ribbon;

use crate::request::MessageProcessor;
use ggez::event::EventHandler;

pub trait Layoutable {
    fn set_rect(&mut self, x: f32, y: f32, w: f32, h: f32);
}

pub trait Widget<MSG>: MessageProcessor<MSG> + EventHandler + Layoutable {
    fn as_message_handler(&mut self) -> &mut dyn MessageProcessor<MSG>;
}

impl<W, MSG> Widget<MSG> for W
where
    W: MessageProcessor<MSG> + EventHandler + Layoutable,
{
    fn as_message_handler(&mut self) -> &mut dyn MessageProcessor<MSG> {
        self
    }
}
