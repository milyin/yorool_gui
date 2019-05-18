use crate::gui::Widget;
use crate::request::Handler;
use crate::gui::Layoutable;
use ggez::event::EventHandler;
use ggez::{Context, GameResult};
use ggez::graphics::Rect;

pub struct Grid<'a,Q,R> {
    widgets: Vec<Box<Widget<Q,R> + 'a>>,
    rect: Rect
}

impl<'a,Q,R> Grid<'a,Q,R> {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
            rect: Rect::zero()
        }
    }

    pub fn add_widget<W>(&mut self, widget: W) -> &mut Self
        where W: Widget<Q,R> + 'a
    {
        self.widgets.push(Box::new(widget));
        self
    }

    fn for_all_res<F: FnMut(&mut Box<Widget<Q,R> + 'a>) -> GameResult>(&mut self, mut f: F) -> GameResult {
        for w in &mut self.widgets {
            f(w)?
        }
        Ok(())
    }

    fn for_all<F: FnMut(&mut Box<Widget<Q,R> + 'a>)>(&mut self, mut f: F) {
        for w in &mut self.widgets {
            f(w)
        }
    }
}

impl<'a,Q,R> Handler<Q,R> for Grid<'a,Q,R> {
    fn handle(&mut self, req: Q) -> Result<R,Q> {
        let mut req = req;
        for w in &mut self.widgets {
            match w.handle(req) {
                Ok(r) => return Ok(r),
                Err(q) => req = q
            }
        }
        Err(req)
    }
}

impl<'a,Q,R> EventHandler for Grid<'a,Q,R> {
    fn update(&mut self,ctx: &mut Context) -> GameResult {
        self.for_all_res(|w| w.update(ctx))
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.for_all_res(|w| w.draw(ctx))
    }
}

impl<Q,R> Layoutable for Grid<'_,Q,R> {
    fn set_rect(&mut self, x:f32, y:f32, w:f32, h:f32) {
        self.rect.x = x;
        self.rect.y = y;
        self.rect.w = w;
        self.rect.h = h;
        let dh = h / self.widgets.len() as f32;
        let mut y = y;
        self.for_all(|wgt| {
            wgt.set_rect(x, y, w, dh);
            y += dh;
        });
    }
}