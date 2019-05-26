pub mod button;
pub mod ribbon;
pub mod panel;

use crate::request::MessageHandler;
use ggez::event::EventHandler;

pub trait Layoutable {
    fn set_rect(&mut self, x:f32, y:f32, w:f32, h:f32);
}

pub trait Widget<MSG> : MessageHandler<MSG> + EventHandler + Layoutable {}
impl<W,MSG> Widget<MSG> for W where W: MessageHandler<MSG> + EventHandler + Layoutable {}

