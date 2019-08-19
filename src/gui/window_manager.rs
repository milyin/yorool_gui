use crate::gui::{Layoutable, Widget};
use ggez::event::EventHandler;
use ggez::graphics::Rect;
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};
use std::cell::RefCell;
use std::rc::Rc;

struct Window<'a> {
    widget: Rc<RefCell<dyn Widget<'a> + 'a>>,
    rect: Rect,
    full_screen: bool,
}

pub struct WindowManager<'a> {
    windows: Vec<Window<'a>>,
    rect: Rect,
}

impl<'a> WindowManager<'a> {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            rect: Rect::zero(),
        }
    }

    pub fn add_window(
        &mut self,
        widget: Rc<RefCell<impl Widget<'a> + 'a>>,
        rect: Rect,
        full_screen: bool,
    ) {
        let widget = widget.clone();
        let mut wrect = rect.clone();
        wrect.translate([self.rect.x, self.rect.y]); // FIXME: is it correct?
        widget.borrow_mut().set_rect(wrect);
        self.windows.push(Window {
            widget,
            rect,
            full_screen,
        });
    }
}

impl EventHandler for WindowManager<'_> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let (width, height) = ggez::graphics::drawable_size(ctx);
        for w in &mut self.windows {
            if w.full_screen {
                w.widget
                    .borrow_mut()
                    .set_rect(Rect::new(0., 0., width, height));
            } else {
                w.widget.borrow_mut().set_rect(w.rect.clone());
            }
            w.widget.borrow_mut().update(ctx)?;
            for e in w.widget.borrow_mut().take_to_execute() {
                (*e)()
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        for w in &mut self.windows {
            w.widget.borrow_mut().draw(ctx)?
        }
        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        for w in &mut self.windows {
            w.widget
                .borrow_mut()
                .mouse_button_down_event(ctx, button, x, y);
        }
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        for w in &mut self.windows {
            w.widget
                .borrow_mut()
                .mouse_button_up_event(ctx, button, x, y);
        }
    }
}

impl Layoutable for WindowManager<'_> {
    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }
    fn get_rect(&self) -> Rect {
        self.rect.clone()
    }
}
