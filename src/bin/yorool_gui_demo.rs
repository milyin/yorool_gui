#![feature(async_await)]
extern crate yorool_gui;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};

use yorool_gui::gui::button::Button;
use yorool_gui::gui::panel::Panel;
use yorool_gui::gui::ribbon::Ribbon;
use yorool_gui::gui::{button, Layoutable, Widget};
use yorool_gui::request::{CtrlId, EvtId, MessageHandler, MessageRouterAsync, Unpack, QR};

#[derive(Debug)]
enum GridMsg {
    ButtonA(button::Event),
    ButtonB(button::Event),
    ButtonC(button::Event),
}

//
// TODO: autogenerate it with macro when stabilized
//
impl Unpack<button::Event> for GridMsg {
    fn peek(&self, f: fn(button::Event) -> Self) -> Option<&button::Event> {
        let test = f(button::Event::default());
        match (self, test) {
            (GridMsg::ButtonA(ref e), GridMsg::ButtonA(_)) => Some(e),
            (GridMsg::ButtonB(ref e), GridMsg::ButtonB(_)) => Some(e),
            (GridMsg::ButtonC(ref e), GridMsg::ButtonC(_)) => Some(e),
            _ => None,
        }
    }

    fn unpack(self, f: fn(button::Event) -> Self) -> Result<button::Event, Self> {
        let test = f(button::Event::default());
        match (self, test) {
            (GridMsg::ButtonA(e), GridMsg::ButtonA(_)) => Ok(e),
            (GridMsg::ButtonB(e), GridMsg::ButtonB(_)) => Ok(e),
            (GridMsg::ButtonC(e), GridMsg::ButtonC(_)) => Ok(e),
            (m, _) => Err(m),
        }
    }
}
/*
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
*/

#[derive(Default)]
struct Radio<MSG> {
    buttons: Vec<CtrlId<MSG, button::Event>>,
}

impl<MSG> Radio<MSG>
where
    MSG: Unpack<button::Event>,
{
    fn add(&mut self, ctrl: CtrlId<MSG, button::Event>) -> &mut Self {
        self.buttons.push(ctrl);
        self
    }
    async fn init<'a>(
        &'a self,
        router: &'a MessageRouterAsync<MSG>,
        default: CtrlId<MSG, button::Event>,
    ) {
        for b in &self.buttons {
            router.query(*b, button::Event::SetState, false).await;
        }
        router.query(default, button::Event::SetState, true).await;
    }
}

struct GuiDemoState<'a> {
    grid: Ribbon<'a, GridMsg>, //Panel<'a, (), GridMsg>
}

impl GuiDemoState<'_> {
    fn new() -> GameResult<Self> {
        let grid = //Panel::new(
            Ribbon::new(false)
                .add_widget(Button::new(GridMsg::ButtonA))
                .add_widget(
                    Ribbon::new(true)
                        .add_widget(Button::new(GridMsg::ButtonB))
                        .add_widget(Button::new(GridMsg::ButtonC)),
//                ),
        );
        Ok(Self { grid })
    }
}

impl EventHandler for GuiDemoState<'_> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let pool = Vec::new();
        //message_loop(self.grid.as_message_handler(), pool);
        let router: MessageRouterAsync<GridMsg, Vec<GridMsg>> = MessageRouterAsync::new(pool);
        let (w, h) = graphics::drawable_size(ctx);
        self.grid.set_rect(0., 0., w, h);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::new(0., 0., 0., 0.));
        self.grid.draw(ctx)?;
        graphics::present(ctx)
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.grid.mouse_button_down_event(ctx, button, x, y)
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.grid.mouse_button_up_event(ctx, button, x, y)
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let new_rect = graphics::Rect::new(0., 0., width, height);
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let cb = ContextBuilder::new("yorool_gui_demo", "milyin")
        .window_setup(WindowSetup::default().title("Yorool GUI demo"))
        .window_mode(WindowMode::default().resizable(true));
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut GuiDemoState::new()?;
    event::run(ctx, event_loop, state)?;
    Ok(())
}
