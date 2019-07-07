use crate::gui::{Layoutable, Widget};
use crate::request::{MessagePoolIn, MessagePoolOut, MessageProcessor, MessageReactor};
use ggez::event::{EventHandler, MouseButton};
use ggez::{Context, GameResult};

pub type Event = ();

pub struct Panel<'a, MSG> {
    widget: Box<dyn Widget<MSG> + 'a>,
    handlers: Vec<Box<dyn MessageReactor<MSG> + 'a>>,
    //    phantom: std::marker::PhantomData<MSG>,
}

impl<'a, MSG> Panel<'a, MSG>
where
    MSG: Clone,
{
    pub fn new<W: Widget<MSG> + 'a>(w: W) -> Self {
        Self {
            widget: box w,
            handlers: Vec::new(),
            //           phantom: std::marker::PhantomData,
        }
    }
    pub fn add_handler<H: MessageReactor<MSG> + 'a>(mut self, handler: H) -> Self {
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

impl<'a, MSG> MessageProcessor<MSG> for Panel<'a, MSG> {}

impl<MSG> EventHandler for Panel<'_, MSG>
where
    MSG: Clone,
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

impl<MSG> Layoutable for Panel<'_, MSG> {
    fn set_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.widget.set_rect(x, y, w, h)
    }
}
