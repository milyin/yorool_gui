use crate::request::Handler;
use ggez::event::EventHandler;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Rect, MeshBuilder, DrawMode, DrawParam};
use crate::gui::Layoutable;

#[derive(Debug)]
pub enum Query {
    SetState(bool),
    GetState
}

#[derive(Debug)]
pub enum Response {
    Ok,
    State(bool)
}

type ToQ<Q> = fn(Q) -> Result<Query,Q>;
type FromR<R> = fn(Response) -> R;

pub struct Button<Q,R> {
    pressed: bool,
    rect: Rect,
    fq: ToQ<Q>,
    fr: FromR<R>
}

impl<Q,R> Button<Q,R>
{
    pub fn new(fq: ToQ<Q>, fr: FromR<R>) -> Self
    {
        Self {
            pressed: false,
            rect: Rect::zero(),
            fq, fr
        }
    }

    fn handle_query(&mut self, q: Query) -> Response {
        match q {
            Query::GetState => Response::State(self.pressed),
            Query::SetState(v) => { self.pressed = v; Response::Ok }
        }
    }
}

impl<Q,R> Handler<Q,R> for Button<Q,R>
{
    fn handle(&mut self, req: Q) -> Result<R,Q> {
        match (self.fq)(req) {
            Ok(q) => Ok((self.fr)(self.handle_query(q))),
            Err(q) => Err(q)
        }
    }
}

impl<Q,R> EventHandler for Button<Q,R>
{
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut rect = self.rect;
        rect.x += 5.;
        rect.y += 5.;
        rect.w -= 10.;
        rect.h -= 10.;
        let mesh = MeshBuilder::new()
            .rectangle(if self.pressed { DrawMode::fill() } else {DrawMode::stroke(1.)},
                       rect, graphics::WHITE)
            .build(ctx)?;
        graphics::draw(
            ctx,
            &mesh,
            DrawParam::default()
        )
    }
}

impl<Q,R> Layoutable for Button<Q,R> {
    fn set_rect(&mut self, x:f32, y:f32, w:f32, h:f32) {
        self.rect.x = x;
        self.rect.y = y;
        self.rect.w = w;
        self.rect.h = h;
    }
}
