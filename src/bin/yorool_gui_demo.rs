#![feature(async_await)]
extern crate yorool_gui;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};

use yorool_gui::gui::button::Button;
use yorool_gui::gui::panel::Panel;
use yorool_gui::gui::ribbon::Ribbon;
use yorool_gui::gui::{button, Layoutable};
use yorool_gui::request::{
    query_by_ctrlid, CtrlId, MessageHandler, MessageHandlerExecutor, MessagePoolIn,
    MessageRouterAsync, Unpack,
};

#[derive(Debug, Clone)]
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

struct Radio<MSG> {
    buttons: Vec<CtrlId<MSG, button::Event>>,
}

impl<MSG> Radio<MSG>
where
    MSG: Unpack<button::Event>,
{
    fn new() -> Self {
        Self {
            buttons: Vec::new(),
        }
    }
    fn add(mut self, ctrl: CtrlId<MSG, button::Event>) -> Self {
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
    async fn on_change<'a>(
        &'a self,
        router: &'a MessageRouterAsync<MSG>,
        changed: CtrlId<MSG, button::Event>,
    ) {
        if router.query(changed, button::Event::GetState, ()).await {
            for b in &self.buttons {
                router.query(*b, button::Event::SetState, false).await;
            }
            router.query(changed, button::Event::SetState, true).await;
        } else {
            router.query(changed, button::Event::SetState, true).await;
        }
    }
}

impl<MSG> MessageHandlerExecutor<MSG> for Radio<MSG>
where
    MSG: Unpack<button::Event>,
{
    fn execute(&mut self, handler: &mut MessageHandler<MSG>, seed: &mut MessagePoolIn<MSG>) {
        for b in &self.buttons {
            for e in query_by_ctrlid(seed, *b) {
                match e {
                    button::Event::Changed => {
                        let router = MessageRouterAsync::new(Vec::new());
                        router.run(handler, self.on_change(&router, *b));
                    }
                    _ => {}
                }
            }
        }
    }
}

struct GuiDemoState<'a> {
    panel: Panel<'a, (), GridMsg>,
}

impl GuiDemoState<'_> {
    fn new() -> GameResult<Self> {
        let radio = Radio::new()
            .add(GridMsg::ButtonA)
            .add(GridMsg::ButtonB)
            .add(GridMsg::ButtonC);
        let mut grid = Ribbon::new(false)
            .add_widget(Button::new(GridMsg::ButtonA))
            .add_widget(
                Ribbon::new(true)
                    .add_widget(Button::new(GridMsg::ButtonB))
                    .add_widget(Button::new(GridMsg::ButtonC)),
            );
        let router = MessageRouterAsync::new(Vec::new());
        router.run(&mut grid, radio.init(&router, GridMsg::ButtonA));
        let panel = Panel::new(grid).add_handler(radio);
        Ok(Self { panel })
    }
}

impl EventHandler for GuiDemoState<'_> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.panel.update(ctx)?;
        let (w, h) = graphics::drawable_size(ctx);
        self.panel.set_rect(0., 0., w, h);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::new(0., 0., 0., 0.));
        self.panel.draw(ctx)?;
        graphics::present(ctx)
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.panel.mouse_button_down_event(ctx, button, x, y)
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.panel.mouse_button_up_event(ctx, button, x, y)
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
