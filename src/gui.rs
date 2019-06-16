pub mod button;
pub mod panel;
pub mod ribbon;

use crate::request::MessageHandler;
use ggez::event::EventHandler;

pub trait Layoutable {
    fn set_rect(&mut self, x: f32, y: f32, w: f32, h: f32);
}

pub trait Widget<MSG>: MessageHandler<MSG> + EventHandler + Layoutable {
    fn as_message_handler(&mut self) -> &mut MessageHandler<MSG>;
}

impl<W, MSG> Widget<MSG> for W
where
    W: MessageHandler<MSG> + EventHandler + Layoutable,
{
    fn as_message_handler(&mut self) -> &mut MessageHandler<MSG> {
        self
    }
}

//async fn query_widget<MSG,Q,R>(ctllid: CtrlId<MSG, >)
