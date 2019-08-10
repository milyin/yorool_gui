use crate::gui::{Executable, Layoutable, Widget};
use ggez::event::{EventHandler, MouseButton};
use ggez::{Context, GameResult};
use std::cell::RefCell;
use std::rc::Rc;

pub type Event = ();

pub struct Panel<'a> {
    widget: Rc<RefCell<dyn Widget<'a> + 'a>>,
}

impl<'a> Panel<'a> {
    pub fn new(w: impl Widget<'a> + 'a) -> Self {
        Self {
            widget: Rc::new(RefCell::new(w)),
        }
    }
}

impl EventHandler for Panel<'_> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.widget.borrow_mut().update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.widget.borrow_mut().draw(ctx)
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.widget
            .borrow_mut()
            .mouse_button_down_event(ctx, button, x, y)
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.widget
            .borrow_mut()
            .mouse_button_up_event(ctx, button, x, y)
    }
}

impl Layoutable for Panel<'_> {
    fn set_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.widget.borrow_mut().set_rect(x, y, w, h)
    }
}

impl<'a> Executable<'a> for Panel<'a> {
    fn to_execute(&mut self) -> Vec<Rc<dyn Fn() + 'a>> {
        self.widget.borrow_mut().to_execute()
    }
}
