pub mod button;
pub mod grid;

use crate::request::Handler;
use ggez::event::EventHandler;

pub trait Layoutable {
    fn set_rect(&mut self, x:f32, y:f32, w:f32, h:f32);
}

pub trait Widget<Q,R> : Handler<Q,R> + EventHandler + Layoutable {}
impl<W,Q,R> Widget<Q,R> for W where W: Handler<Q,R> + EventHandler + Layoutable {}
