use crate::gui::Widget;
use crate::request::Handler;
use crate::gui::Layoutable;
use ggez::event::{EventHandler, MouseButton};
use ggez::{Context, GameResult};
use ggez::graphics::Rect;

pub struct Ribbon<'a,MSG> {
    widgets: Vec<Box<Widget<MSG> + 'a>>,
    rect: Rect,
    horizontal: bool
}

impl<'a,MSG> Ribbon<'a,MSG> {
    pub fn new(horizontal: bool) -> Self {
        Self {
            widgets: Vec::new(),
            rect: Rect::zero(),
            horizontal
        }
    }

    pub fn add_widget<W>(mut self, widget: W) -> Self
        where W: Widget<MSG> + 'a
    {
        self.widgets.push(Box::new(widget));
        self
    }

    fn for_all_res<F: FnMut(&mut Box<Widget<MSG> + 'a>) -> GameResult>(&mut self, mut f: F) -> GameResult {
        for w in &mut self.widgets {
            f(w)?
        }
        Ok(())
    }

    fn for_all<F: FnMut(&mut Box<Widget<MSG> + 'a>)>(&mut self, mut f: F) {
        for w in &mut self.widgets {
            f(w)
        }
    }
}

impl<MSG> Handler<MSG> for Ribbon<'_,MSG> {

    fn collect(&mut self) -> Vec<MSG> {
        let mut messages = Vec::new();
        for w in &mut self.widgets {
            messages.append(&mut w.collect());
        }
        messages
    }
    fn handle(&mut self, msgs: Vec<MSG>) -> Vec<MSG> {
        let mut msgs = msgs;
        for w in &mut self.widgets {
            msgs = w.handle(msgs);
            if msgs.is_empty() { break }
        }
        msgs
    }
}

impl<'a,MSG> EventHandler for Ribbon<'a,MSG> {
    fn update(&mut self,ctx: &mut Context) -> GameResult {
        self.for_all_res(|w| w.update(ctx))
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.for_all_res(|w| w.draw(ctx))
    }

    fn mouse_button_down_event(
        &mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32,
    ) {
        self.for_all( |w|
            w.mouse_button_down_event(ctx,button,x,y)
        )
    }

    fn mouse_button_up_event(
        &mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32,
    ) {
        self.for_all( |w|
            w.mouse_button_up_event(ctx,button,x,y)
        )
    }
}

impl<MSG> Layoutable for Ribbon<'_,MSG> {
    fn set_rect(&mut self, x:f32, y:f32, w:f32, h:f32) {
        self.rect.x = x;
        self.rect.y = y;
        self.rect.w = w;
        self.rect.h = h;
        if self.horizontal {
            let dw = w / self.widgets.len() as f32;
            let mut x = x;
            self.for_all(|wgt| {
                wgt.set_rect(x, y, dw, h);
                x += dw;
            });
        } else {
            let dh = h / self.widgets.len() as f32;
            let mut y = y;
            self.for_all(|wgt| {
                wgt.set_rect(x, y, w, dh);
                y += dh;
            });
        }
    }
}