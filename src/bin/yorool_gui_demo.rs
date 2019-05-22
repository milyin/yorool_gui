extern crate yorool_gui;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color};
use ggez::conf::{WindowSetup, WindowMode};

use yorool_gui::gui::{button, Layoutable};
use yorool_gui::gui::button::Button;
use yorool_gui::gui::ribbon::Ribbon;
use yorool_gui::request::{IMessageHandler, Unpack, MessageProcessor, Event, is_changed, get_state};

#[derive(Debug)]
enum GuiDemoMsg {
    ButtonA(button::Event),
    ButtonB(button::Event),
    ButtonC(button::Event)
}

//
// TODO: autogenerate it with macro when stabilized
//
impl Unpack<button::Event> for GuiDemoMsg {
    fn peek(&self, f: fn(button::Event) -> Self) -> Option<&button::Event> {
        let test = f(button::Event::default());
        match (self, test) {
            (GuiDemoMsg::ButtonA(ref e), GuiDemoMsg::ButtonA(_)) => Some(&e),
            (GuiDemoMsg::ButtonB(ref e), GuiDemoMsg::ButtonB(_)) => Some(&e),
            (GuiDemoMsg::ButtonC(ref e), GuiDemoMsg::ButtonC(_)) => Some(&e),
            _ => None
        }
    }

    fn unpack(self, f: fn(button::Event) -> Self ) -> Result<button::Event,Self> {
        let test = f(button::Event::default());
        match (self, test) {
            (GuiDemoMsg::ButtonA(e), GuiDemoMsg::ButtonA(_)) => Ok(e),
            (GuiDemoMsg::ButtonB(e), GuiDemoMsg::ButtonB(_)) => Ok(e),
            (GuiDemoMsg::ButtonC(e), GuiDemoMsg::ButtonC(_)) => Ok(e),
            (m,_) => Err(m)
        }
    }
}

fn radio_group_query() -> impl Fn(Vec<GuiDemoMsg>) -> Vec<GuiDemoMsg> {
    |mut msgs| {
        if is_changed(GuiDemoMsg::ButtonA, &msgs) ||
           is_changed(GuiDemoMsg::ButtonB, &msgs) ||
           is_changed(GuiDemoMsg::ButtonC, &msgs)
        {
            msgs.push(GuiDemoMsg::ButtonA(Event::QueryState));
            msgs.push(GuiDemoMsg::ButtonB(Event::QueryState));
            msgs.push(GuiDemoMsg::ButtonC(Event::QueryState));
        }
        msgs
    }
}

fn radio_group_execute() -> impl Fn(Vec<GuiDemoMsg>) -> Vec<GuiDemoMsg> {
    |mut msgs| {
        if let (Some(a), Some(b), Some(c)) = (
            get_state(GuiDemoMsg::ButtonA, &msgs),
            get_state(GuiDemoMsg::ButtonB, &msgs),
            get_state(GuiDemoMsg::ButtonC, &msgs))
        {
            if *a && is_changed(GuiDemoMsg::ButtonA, &msgs) {
                msgs.push(GuiDemoMsg::ButtonB(Event::SetState(false)));
                msgs.push(GuiDemoMsg::ButtonC(Event::SetState(false)));
            } else if *b && is_changed(GuiDemoMsg::ButtonB, &msgs) {
                msgs.push(GuiDemoMsg::ButtonA(Event::SetState(false)));
                msgs.push(GuiDemoMsg::ButtonC(Event::SetState(false)));
            } else if *c && is_changed(GuiDemoMsg::ButtonC, &msgs) {
                msgs.push(GuiDemoMsg::ButtonA(Event::SetState(false)));
                msgs.push(GuiDemoMsg::ButtonB(Event::SetState(false)));
            }
        }
        msgs
    }
}

struct GuiDemoState<'a> {
    grid: Ribbon<'a,GuiDemoMsg>,
    grid_proc_query: MessageProcessor<'a,GuiDemoMsg>,
    grid_proc_execute: MessageProcessor<'a,GuiDemoMsg>
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
        let grid_proc_query = MessageProcessor::new()
            .add_multiple(radio_group_query());
        let grid_proc_execute = MessageProcessor::new()
            .add_multiple(radio_group_execute());
        Ok(Self{grid, grid_proc_query, grid_proc_execute})
    }
}

impl EventHandler for GuiDemoState<'_> {

    fn update(&mut self,ctx: &mut Context) -> GameResult {
        let notifications = self.grid.collect();
        if !notifications.is_empty() {
            dbg!(&notifications);
            let queries = self.grid_proc_query.process(notifications);
            dbg!(&queries);
            let responses = self.grid.handle(queries);
            dbg!(&responses);
            let commands = self.grid_proc_execute.process(responses);
            dbg!(&commands);
            let _ = dbg!(self.grid.handle(commands));
        }
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
