extern crate yorool_gui;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color, DrawMode, DrawParam, MeshBuilder, Rect};
use ggez::{Context, ContextBuilder, GameResult};

use ggez::nalgebra::Point2;
use std::cell::RefCell;
use std::rc::Rc;
use yorool_gui::gui::button::Button;
use yorool_gui::gui::checkbox::{make_radio, Checkbox};
use yorool_gui::gui::panel::Panel;
use yorool_gui::gui::ribbon::Ribbon;
use yorool_gui::gui::window_manager::WindowManager;

struct GuiDemoState<'a> {
    window_manager: WindowManager<'a>,
    demo_panel: DemoPanel<'a>,
}

struct DemoPanel<'a> {
    radio_a: Rc<RefCell<Checkbox<'a>>>,
    radio_b: Rc<RefCell<Checkbox<'a>>>,
    radio_c: Rc<RefCell<Checkbox<'a>>>,
}

impl<'a> DemoPanel<'a> {
    fn new(wm: &mut WindowManager<'a>) -> Self {
        let radio_a = Rc::new(RefCell::new(Checkbox::<'a>::new()));
        let radio_b = Rc::new(RefCell::new(Checkbox::<'a>::new()));
        let radio_c = Rc::new(RefCell::new(Checkbox::<'a>::new()));

        make_radio(vec![radio_a.clone(), radio_b.clone(), radio_c.clone()]);

        let panel = Rc::new(RefCell::new(Panel::new(
            Ribbon::new(false)
                .add_widget(
                    Ribbon::new(true)
                        .add_widget_rc(radio_a.clone())
                        .add_widget_rc(radio_b.clone())
                        .add_widget_rc(radio_c.clone()),
                )
                .add_widget(Button::new()),
        )));

        wm.add_window(panel, Rect::zero(), true);

        Self {
            radio_a,
            radio_b,
            radio_c,
        }
    }
}

impl GuiDemoState<'_> {
    fn new() -> Self {
        let mut window_manager = WindowManager::new();
        let demo_panel = DemoPanel::new(&mut window_manager);
        Self {
            window_manager,
            demo_panel,
        }
    }
}

impl EventHandler for GuiDemoState<'_> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.window_manager.update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::new(0., 0., 0., 0.));
        self.window_manager.draw(ctx)?;

        if self.demo_panel.radio_a.borrow_mut().get_state() {
            let (w, h) = ggez::graphics::drawable_size(ctx);
            let mesh = MeshBuilder::new()
                .circle(
                    DrawMode::stroke(1.),
                    Point2::new(w / 2., h / 2.),
                    if h > w { w / 2. } else { h / 2. },
                    1.,
                    graphics::WHITE,
                )
                .build(ctx)?;
            graphics::draw(ctx, &mesh, DrawParam::default())?;
        }

        graphics::present(ctx)
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.window_manager
            .mouse_button_down_event(ctx, button, x, y)
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.window_manager.mouse_button_up_event(ctx, button, x, y)
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
    let state = &mut GuiDemoState::new();
    event::run(ctx, event_loop, state)?;
    Ok(())
}
