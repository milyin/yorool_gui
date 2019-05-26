use crate::gui::{Layoutable, Widget};
use crate::request::{message_loop, MessageHandler, MessagePoolIn, MessagePoolOut};
use ggez::event::{EventHandler, MouseButton};
use ggez::{Context, GameResult};

pub type Event = ();

pub struct Panel<'a, EXTMSG, INTMSG> {
    widget: Box<dyn Widget<INTMSG> + 'a>,
    phantom: std::marker::PhantomData<EXTMSG>,
}

impl<'a, EXTMSG, INTMSG> Panel<'a, EXTMSG, INTMSG> {
    pub fn new<W: Widget<INTMSG> + 'a>(w: W) -> Self {
        Self {
            widget: box w,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, EXTMSG, INTMSG> MessageHandler<EXTMSG> for Panel<'a, EXTMSG, INTMSG> {
    fn handle(&mut self, _src: &mut MessagePoolIn<EXTMSG>, _dst: &mut MessagePoolOut<EXTMSG>) {
        let intpool = Vec::new();
        message_loop(self.widget.as_message_handler(), intpool);
    }
}

impl<EXTMSG, INTMSG> EventHandler for Panel<'_, EXTMSG, INTMSG> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.widget.update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.widget.draw(ctx)
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.widget.mouse_button_down_event(ctx, button, x, y)
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.widget.mouse_button_up_event(ctx, button, x, y)
    }
}

impl<EXTMSG, INTMSG> Layoutable for Panel<'_, EXTMSG, INTMSG> {
    fn set_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.widget.set_rect(x, y, w, h)
    }
}
