use crate::gui::Widget;
use crate::request::Handler;
use ggez::event::EventHandler;
use ggez::{Context, GameResult};

pub struct Grid<'a,Q,R> {
    widgets: Vec<Box<Widget<Q,R> + 'a>>
}

impl<'a,Q,R> Grid<'a,Q,R> {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new()
        }
    }

    pub fn add_widget<W>(&mut self, widget: W) -> &mut Self
        where W: Widget<Q,R> + 'a
    {
        self.widgets.push(Box::new(widget));
        self
    }

    fn for_all<F: FnMut(&mut Box<Widget<Q,R> + 'a>) -> GameResult>(&mut self, mut f: F) -> GameResult {
        for w in &mut self.widgets {
            f(w)?
        }
        Ok(())
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
        self.for_all(|w| w.update(ctx))
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.for_all(|w| w.draw(ctx))
    }
}

