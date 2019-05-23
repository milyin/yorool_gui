use crate::gui::{Widget, Layoutable};
use crate::request::MessageHandler;
use ggez::event::{EventHandler, MouseButton};
use ggez::{Context, GameResult};
use std::marker::PhantomData;

pub struct Panel<MSG,W: Widget<MSG>> {
    widget: W,
    phantom: PhantomData<MSG>
}

impl<MSG,W:Widget<MSG>> Panel<MSG,W> {
    pub fn new(widget: W) -> Self {
        Self { widget, phantom:std::marker::PhantomData }
    }
}

impl<MSG,W:Widget<MSG>> MessageHandler<MSG> for Panel<MSG,W> {
    type T = ();
    type S = ();
    fn collect_impl(&mut self) -> Vec<MSG> {
        self.widget.collect()
    }
    fn handle_impl(&mut self, msgs: Vec<MSG>) -> Vec<MSG> {
        self.widget.handle(msgs)
    }
}

impl<MSG,W:Widget<MSG>> EventHandler for Panel<MSG,W> {
    fn update(&mut self,ctx: &mut Context) -> GameResult {
        self.widget.update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.widget.draw(ctx)
    }

    fn mouse_button_down_event(
        &mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32,
    ) {
        self.widget.mouse_button_down_event(ctx,button,x,y)
    }

    fn mouse_button_up_event(
        &mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32,
    ) {
        self.widget.mouse_button_up_event(ctx,button,x,y)
    }
}

impl<MSG,W:Widget<MSG>> Layoutable for Panel<MSG,W> {
    fn set_rect(&mut self, x:f32, y:f32, w:f32, h:f32) {
        self.widget.set_rect(x,y,w,h)
    }
}
