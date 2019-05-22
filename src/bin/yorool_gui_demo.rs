extern crate yorool_gui;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color};
use ggez::conf::{WindowSetup, WindowMode};

use yorool_gui::gui::{button, Layoutable};
use yorool_gui::gui::button::Button;
use yorool_gui::gui::ribbon::Ribbon;
use yorool_gui::request::{IMessageHandler, Unpack};

#[allow(dead_code)]
enum GuiDemoMsg {
    ButtonA(button::Event),
    ButtonB(button::Event),
    ButtonC(button::Event)
}

impl Unpack<button::Event> for GuiDemoMsg {
    fn unpack(self, f: fn(button::Event) -> GuiDemoMsg ) -> Result<button::Event,GuiDemoMsg> {
        let test = f(button::Event::default());
        match (self, test) {
            (GuiDemoMsg::ButtonA(e), GuiDemoMsg::ButtonA(_)) => Ok(e),
            (GuiDemoMsg::ButtonB(e), GuiDemoMsg::ButtonB(_)) => Ok(e),
            (GuiDemoMsg::ButtonC(e), GuiDemoMsg::ButtonC(_)) => Ok(e),
            (m,_) => Err(m)
        }
    }
}

struct GuiDemoState<'a> {
    grid: Ribbon<'a,GuiDemoMsg>
}

impl GuiDemoState<'_> {
    fn new() -> GameResult<Self> {
        let grid = Ribbon::new(false)
            .add_widget(
                Button::new(GuiDemoMsg::ButtonA)
            )
            .add_widget(Ribbon::new(true)
                .add_widget(Button::new(GuiDemoMsg::ButtonB))
                .add_widget(Button::new(GuiDemoMsg::ButtonC))
            );
        Ok(Self{grid})
    }
}

impl EventHandler for GuiDemoState<'_> {

    fn update(&mut self,ctx: &mut Context) -> GameResult {
        let msgs = self.grid.collect();
        // TODO: process messages
        self.grid.handle(msgs);
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
