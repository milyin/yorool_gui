extern crate yorool_gui;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};

use std::cell::RefCell;
use std::rc::Rc;
use yorool_gui::gui::button::Button;
use yorool_gui::gui::checkbox::Checkbox;
use yorool_gui::gui::panel::Panel;
use yorool_gui::gui::ribbon::Ribbon;
use yorool_gui::gui::{Executable, Layoutable};

struct GuiDemoState<'a> {
    panel: Panel<'a>,
}

fn make_radio<'a>(checkboxes: Vec<Rc<RefCell<Checkbox<'a>>>>) {
    for n in 0..checkboxes.len() {
        let (head, curr_tail) = checkboxes.split_at(n);
        let (curr, tail) = curr_tail.split_first().unwrap();
        let handler = {
            let curr = curr.clone();
            let others = head
                .iter()
                .chain(tail.iter())
                .map(|v| v.clone())
                .collect::<Vec<_>>();
            move || {
                let mut curr = curr.borrow_mut();
                if curr.get_state() {
                    for w in &others {
                        w.borrow_mut().set_state(false);
                    }
                } else {
                    curr.set_state(true);
                }
            }
        };
        curr.borrow_mut().on_changed(handler);
    }
}

impl GuiDemoState<'_> {
    fn new() -> GameResult<Self> {
        let radio_a = Rc::new(RefCell::new(Checkbox::new()));
        let radio_b = Rc::new(RefCell::new(Checkbox::new()));
        let radio_c = Rc::new(RefCell::new(Checkbox::new()));

        make_radio(vec![radio_a.clone(), radio_b.clone(), radio_c.clone()]);

        let grid = Ribbon::new(false)
            .add_widget(
                Ribbon::new(true)
                    .add_widget_rc(radio_a)
                    .add_widget_rc(radio_b)
                    .add_widget_rc(radio_c),
            )
            .add_widget(Button::new());

        let panel = Panel::new(grid);

        Ok(Self { panel })
    }
}

impl EventHandler for GuiDemoState<'_> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        for e in self.panel.to_execute() {
            (*e)()
        }

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
