extern crate yorool_gui;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color, Rect};
use ggez::{Context, ContextBuilder, GameResult};

use yorool_gui::gui::button::ButtonBuilder;
use yorool_gui::gui::checkbox::CheckboxBuilder;
use yorool_gui::gui::panel::PanelBuilder;
use yorool_gui::gui::radio_group::RadioGroupBuilder;
use yorool_gui::gui::ribbon::RibbonBuilder;
use yorool_gui::gui::window_manager::WindowManager;

struct GuiDemoState<'a> {
    window_manager: WindowManager<'a>,
    _radio_panel: RadioPanel,
}

struct RadioPanel {}

impl<'a> RadioPanel {
    fn new(wm: &mut WindowManager<'a>) -> Self {
        let radio_a = CheckboxBuilder::new().build();
        let radio_b = CheckboxBuilder::new().build();
        let radio_c = CheckboxBuilder::new().build();

        let radio_group = RadioGroupBuilder::new()
            .add_widget(radio_a.clone())
            .add_widget(radio_b.clone())
            .add_widget(radio_c.clone())
            .build();

        let radio_ribbon = RibbonBuilder::new()
            .set_horizontal(true)
            .add_widget(radio_a)
            .add_widget(radio_b)
            .add_widget(radio_c)
            .build();

        let add_radio = {
            let radio_group = radio_group.clone();
            let radio_ribbon = radio_ribbon.clone();
            move |_| {
                let radio = CheckboxBuilder::new().build();
                radio_group.borrow_mut().add_widget(radio.clone());
                radio_ribbon.borrow_mut().add_widget(radio.clone());
            }
        };

        let remove_radio = {
            let radio_group = radio_group.clone();
            let radio_ribbon = radio_ribbon.clone();
            move |_| {
                let radio = radio_group.borrow().radios().last().map(|r| r.clone());
                if let Some(radio) = radio {
                    radio_group.borrow_mut().remove_widget(radio.clone());
                    radio_ribbon.borrow_mut().remove_widget(radio.clone());
                }
            }
        };

        let panel = PanelBuilder::new()
            .add_widget(
                RibbonBuilder::new()
                    .set_horizontal(false)
                    .add_widget(radio_ribbon.clone())
                    .add_widget(
                        RibbonBuilder::new()
                            .set_horizontal(true)
                            .add_widget(
                                ButtonBuilder::new()
                                    .set_label("Add")
                                    .on_click(add_radio)
                                    .build(),
                            )
                            .add_widget(
                                ButtonBuilder::new()
                                    .set_label("Remove")
                                    .on_click(remove_radio)
                                    .build(),
                            )
                            .build(),
                    )
                    .build(),
            )
            .build();

        wm.add_window(panel, Rect::zero(), true);

        Self {}
    }
}

impl GuiDemoState<'_> {
    fn new() -> Self {
        let mut window_manager = WindowManager::new();
        let _radio_panel = RadioPanel::new(&mut window_manager);
        Self {
            window_manager,
            _radio_panel,
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
