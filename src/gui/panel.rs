use crate::gui::{Layoutable, Widget};
use crate::request::{MessagePoolIn, MessagePoolOut, MessageProcessor, MessageReactor};
use ggez::event::{EventHandler, MouseButton};
use ggez::{Context, GameResult};

pub type Event = ();

pub struct Panel<'a, EXTMSG, INTMSG> {
    widget: Box<dyn Widget<INTMSG> + 'a>,
    handlers: Vec<Box<dyn MessageReactor<INTMSG> + 'a>>,
    phantom: std::marker::PhantomData<EXTMSG>,
}

impl<'a, EXTMSG, INTMSG> Panel<'a, EXTMSG, INTMSG>
where
    INTMSG: Clone,
{
    pub fn new<W: Widget<INTMSG> + 'a>(w: W) -> Self {
        Self {
            widget: box w,
            handlers: Vec::new(),
            phantom: std::marker::PhantomData,
        }
    }
    pub fn add_handler<H: MessageReactor<INTMSG> + 'a>(mut self, handler: H) -> Self {
        self.handlers.push(box handler);
        self
    }
    pub fn run_handlers(&mut self) {
        // collect notifications
        let mut notifications = Vec::new();
        self.widget.process(&mut Vec::new(), &mut notifications);
        // handle notifications by registered handlers
        for h in &mut self.handlers {
            h.react(self.widget.as_message_handler(), &mut notifications.clone());
        }
    }
}

impl<'a, EXTMSG, INTMSG> MessageProcessor<EXTMSG> for Panel<'a, EXTMSG, INTMSG>
where
    INTMSG: Clone,
{
    fn process(
        &mut self,
        _src: &mut dyn MessagePoolIn<EXTMSG>,
        _dst: &mut dyn MessagePoolOut<EXTMSG>,
    ) {
    }
}

impl<EXTMSG, INTMSG> EventHandler for Panel<'_, EXTMSG, INTMSG>
where
    INTMSG: Clone,
{
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.run_handlers();
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
