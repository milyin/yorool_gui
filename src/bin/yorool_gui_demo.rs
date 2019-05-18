extern crate yorool_gui;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::conf::{WindowSetup, WindowMode};

struct GuiDemoState {

}

impl GuiDemoState {
    fn new() -> GameResult<Self> {
        Ok(Self{})
    }
}

impl EventHandler for GuiDemoState {

    fn update(&mut self,_ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::new(0.,0.,0.,0.));
        graphics::present(ctx)
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let new_rect = graphics::Rect::new(0.,0., width, height);
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
    }

}

fn main() -> Result<(),Box<std::error::Error>> {
    let cb = ContextBuilder::new("yorool_gui_demo", "milyin")
        .window_setup(WindowSetup::default().title("Yorool GUI demo"))
        .window_mode(WindowMode::default().resizable(true));
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut GuiDemoState::new()?;
    event::run(ctx, event_loop, state)?;
    Ok(())
}
