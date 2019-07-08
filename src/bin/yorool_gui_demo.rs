#![feature(async_await)]
#![feature(async_closure)]
extern crate yorool_gui;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};

use yorool_gui::gui::button::Button;
use yorool_gui::gui::checkbox::Checkbox;
use yorool_gui::gui::panel::Panel;
use yorool_gui::gui::ribbon::Ribbon;
use yorool_gui::gui::{button, checkbox, Layoutable};
use yorool_gui::request::{
    query_by_ctrlid, CtrlId, MessagePoolIn, MessageProcessor, MessageReactor, MessageRouterAsync,
    Unpack,
};

#[derive(Debug, Clone)]
enum GridMsg {
    RadioA(checkbox::Event),
    RadioB(checkbox::Event),
    RadioC(checkbox::Event),
    Button(button::Event),
}

//
// TODO: autogenerate it with macro when stabilized
//
impl Unpack<checkbox::Event> for GridMsg {
    fn peek(&self, ctrlid: CtrlId<Self, checkbox::Event>) -> Option<&checkbox::Event> {
        let test = ctrlid.tomsg(checkbox::Event::default());
        match (self, test) {
            (GridMsg::RadioA(ref e), GridMsg::RadioA(_)) => Some(e),
            (GridMsg::RadioB(ref e), GridMsg::RadioB(_)) => Some(e),
            (GridMsg::RadioC(ref e), GridMsg::RadioC(_)) => Some(e),
            _ => None,
        }
    }

    fn unpack(self, ctrlid: CtrlId<Self, checkbox::Event>) -> Result<checkbox::Event, Self> {
        let test = ctrlid.tomsg(checkbox::Event::default());
        match (self, test) {
            (GridMsg::RadioA(e), GridMsg::RadioA(_)) => Ok(e),
            (GridMsg::RadioB(e), GridMsg::RadioB(_)) => Ok(e),
            (GridMsg::RadioC(e), GridMsg::RadioC(_)) => Ok(e),
            (m, _) => Err(m),
        }
    }
}

impl Unpack<button::Event> for GridMsg {
    fn peek(&self, ctrlid: CtrlId<Self, button::Event>) -> Option<&button::Event> {
        let test = ctrlid.tomsg(button::Event::default());
        match (self, test) {
            (GridMsg::Button(ref e), GridMsg::Button(_)) => Some(e),
            _ => None,
        }
    }

    fn unpack(self, ctrlid: CtrlId<Self, button::Event>) -> Result<button::Event, Self> {
        let test = ctrlid.tomsg(button::Event::default());
        match (self, test) {
            (GridMsg::Button(e), GridMsg::Button(_)) => Ok(e),
            (m, _) => Err(m),
        }
    }
}

struct Radio<MSG> {
    buttons: Vec<CtrlId<MSG, checkbox::Event>>,
}

impl<MSG> Radio<MSG>
where
    MSG: Unpack<checkbox::Event>,
{
    fn new() -> Self {
        Self {
            buttons: Vec::new(),
        }
    }
    fn add(mut self, ctrl: fn(checkbox::Event) -> MSG) -> Self {
        self.buttons.push(ctrl.into());
        self
    }
    async fn on_init<'a>(
        &'a self,
        router: &'a MessageRouterAsync<MSG>,
        checkbox: CtrlId<MSG, checkbox::Event>,
    ) {
        if let Some(first) = self.buttons.first() {
            router
                .query(checkbox, checkbox::Event::SetState, *first == checkbox)
                .await;
            return;
        }
        router
            .query(checkbox, checkbox::Event::SetState, false)
            .await;
    }
    async fn on_change<'a>(
        &'a self,
        router: &'a MessageRouterAsync<MSG>,
        changed: CtrlId<MSG, checkbox::Event>,
    ) {
        if router.query(changed, checkbox::Event::GetState, ()).await {
            for b in &self.buttons {
                router.query(*b, checkbox::Event::SetState, false).await;
            }
            router.query(changed, checkbox::Event::SetState, true).await;
        } else {
            router.query(changed, checkbox::Event::SetState, true).await;
        }
    }
}

impl<MSG> MessageReactor<MSG> for Radio<MSG>
where
    MSG: Unpack<checkbox::Event>,
{
    fn react(
        &mut self,
        handler: &mut dyn MessageProcessor<MSG>,
        seed: &mut dyn MessagePoolIn<MSG>,
    ) {
        for b in &self.buttons {
            for e in query_by_ctrlid(seed, *b) {
                let router = MessageRouterAsync::new(Vec::new());
                match e {
                    checkbox::Event::Init => {
                        router.run(handler, self.on_init(&router, *b));
                    }
                    checkbox::Event::Pressed => {
                        router.run(handler, self.on_change(&router, *b));
                    }
                    _ => {}
                }
            }
        }
    }
}

struct GuiDemoState<'a> {
    panel: Panel<'a, GridMsg>,
}

impl GuiDemoState<'_> {
    fn new() -> GameResult<Self> {
        let radio = Radio::new()
            .add(GridMsg::RadioA)
            .add(GridMsg::RadioB)
            .add(GridMsg::RadioC);
        let grid = Ribbon::new(false)
            .add_widget(
                Ribbon::new(true)
                    .add_widget(Checkbox::new(GridMsg::RadioA))
                    .add_widget(Checkbox::new(GridMsg::RadioB))
                    .add_widget(Checkbox::new(GridMsg::RadioC)),
            )
            .add_widget(Button::new(GridMsg::Button));

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cb = ContextBuilder::new("yorool_gui_demo", "milyin")
        .window_setup(WindowSetup::default().title("Yorool GUI demo"))
        .window_mode(WindowMode::default().resizable(true));
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut GuiDemoState::new()?;
    event::run(ctx, event_loop, state)?;
    Ok(())
}
