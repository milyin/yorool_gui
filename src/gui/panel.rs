use crate::gui::{Layoutable, Widget};
use crate::request::MessageSender;
use ggez::event::{EventHandler, MouseButton};
use ggez::{Context, GameResult};

pub type Event = ();

pub struct Panel<'a, MSG> {
    widget: Box<dyn Widget<MSG> + 'a>,
    //    phantom: std::marker::PhantomData<MSG>,
}

impl<'a, MSG> Panel<'a, MSG>
where
    MSG: Clone,
{
    pub fn new<W: Widget<MSG> + 'a>(w: W) -> Self {
        Self {
            widget: box w,
            //           phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, MSG> MessageSender<MSG> for Panel<'a, MSG> {
    fn get_message(&mut self) -> Option<MSG> {
        self.widget.get_message()
    }
}

impl<MSG> EventHandler for Panel<'_, MSG>
where
    MSG: Clone,
{
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

impl<MSG> Layoutable for Panel<'_, MSG> {
    fn set_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.widget.set_rect(x, y, w, h)
    }
}
