pub mod button;
pub mod ribbon;

use crate::request::Handler;
use ggez::event::EventHandler;

pub trait Layoutable {
    fn set_rect(&mut self, x:f32, y:f32, w:f32, h:f32);
}

pub trait Widget<MSG,CMD> : Handler<MSG,CMD> + EventHandler + Layoutable where CMD: Clone {}
impl<W,MSG,CMD> Widget<MSG,CMD> for W where W: Handler<MSG,CMD> + EventHandler + Layoutable, CMD: Clone {}
