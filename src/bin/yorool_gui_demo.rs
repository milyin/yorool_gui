extern crate yorool_gui;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color, Rect};
use ggez::{Context, ContextBuilder, GameResult};

use std::cell::RefCell;
use std::rc::Rc;
use yorool_gui::gui::button::Button;
use yorool_gui::gui::checkbox::{make_radio, Checkbox};
use yorool_gui::gui::panel::PanelBuilder;
use yorool_gui::gui::ribbon::{Ribbon, RibbonBuilder};
use yorool_gui::gui::window_manager::WindowManager;

struct GuiDemoState<'a> {
    window_manager: WindowManager<'a>,
    demo_panel: DemoPanel<'a>,
}

struct DemoPanel<'a> {
    radios_ribbon: Rc<RefCell<Ribbon<'a>>>,
    radios: Vec<Rc<RefCell<Checkbox<'a>>>>,
}

impl<'a> DemoPanel<'a> {
    fn new(wm: &mut WindowManager<'a>) -> Self {
        let mut radios = Vec::new();
        for _i in 0..3 {
            radios.push(Rc::new(RefCell::new(Checkbox::<'a>::new())));
        }

        let radios_ribbon = RibbonBuilder::new().set_horizontal(true).build();

        //        make_radio(vec![radio_a.clone(), radio_b.clone(), radio_c.clone()]);

        let panel = PanelBuilder::new()
            .set_widget_rc(
                RibbonBuilder::new()
                    .set_horizontal(false)
                    .add_widget_rc(radios_ribbon.clone())
                    .add_widget(Button::new())
                    .build(),
            )
            .build();

        for r in &radios {
            radios_ribbon.borrow_mut().add_widget_rc(r.clone());
        }

        wm.add_window(panel, Rect::zero(), true);

        Self {
            radios_ribbon,
            radios,
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
