extern crate yorool_gui;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color};
use ggez::conf::{WindowSetup, WindowMode};

use yorool_gui::gui::{button, Layoutable};
use yorool_gui::gui::button::Button;
use yorool_gui::gui::ribbon::Ribbon;
use yorool_gui::request::Handler;

#[allow(dead_code)]
enum GuiDemoQ {
    ButtonA(button::Query),
    ButtonB(button::Query),
    ButtonC(button::Query)
}

#[allow(dead_code)]
enum GuiDemoR {
    ButtonA(button::Response),
    ButtonB(button::Response),
    ButtonC(button::Response)
}

struct GuiDemoState<'a> {
    grid: Ribbon<'a,GuiDemoQ,GuiDemoR>
}

impl GuiDemoState<'_> {
    fn new() -> GameResult<Self> {
        fn to_button_a(q: GuiDemoQ) -> Result<button::Query, GuiDemoQ>
            { if let GuiDemoQ::ButtonA(bq) = q { Ok(bq ) } else { Err(q) } }
        fn to_button_b(q: GuiDemoQ) -> Result<button::Query, GuiDemoQ>
            { if let GuiDemoQ::ButtonB(bq) = q { Ok(bq ) } else { Err(q) } }
        fn to_button_c(q: GuiDemoQ) -> Result<button::Query, GuiDemoQ>
        { if let GuiDemoQ::ButtonC(bq) = q { Ok(bq ) } else { Err(q) } }
        let mut grid = Ribbon::new(false)
            .add_widget(Button::new(to_button_a, GuiDemoR::ButtonA))
            .add_widget(Ribbon::new(true)
                .add_widget(Button::new(to_button_b, GuiDemoR::ButtonB))
                .add_widget(Button::new(to_button_c, GuiDemoR::ButtonC))
            );
        let _ = grid.handle(GuiDemoQ::ButtonA(button::Query::SetState(true)));
        let _ = grid.handle(GuiDemoQ::ButtonC(button::Query::SetState(true)));
        Ok(Self{grid})
    }
}

impl EventHandler for GuiDemoState<'_> {

    fn update(&mut self,ctx: &mut Context) -> GameResult {
        let (w, h) = graphics::drawable_size(ctx);
        self.grid.set_rect(0.,0.,w,h);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::new(0.,0.,0.,0.));
        self.grid.draw(ctx)?;
        graphics::present(ctx)
    }

    fn mouse_button_down_event(
        &mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32,
    ) {
        self.grid.mouse_button_down_event(ctx,button,x,y)
    }

    fn mouse_button_up_event(
        &mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32,
    ) {
        self.grid.mouse_button_up_event(ctx,button,x,y)
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
