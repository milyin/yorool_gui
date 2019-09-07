use crate::gui::{IActions, ILayout, Widget};
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::Rect;
use ggez::{Context, GameResult};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Panel<'a> {
    widgets: Vec<Rc<RefCell<dyn Widget<'a> + 'a>>>,
}

impl<'a> Panel<'a> {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
        }
    }

    pub fn add_widget(&mut self, w: Rc<RefCell<dyn Widget<'a> + 'a>>) -> &mut Self {
        self.widgets.push(w);
        self
    }
}

impl EventHandler for Panel<'_> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        for w in &self.widgets {
            w.borrow_mut().update(ctx)?
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        for w in &self.widgets {
            w.borrow_mut().draw(ctx)?
        }
        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        for w in &self.widgets {
            w.borrow_mut().mouse_button_down_event(ctx, button, x, y)
        }
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        for w in &self.widgets {
            w.borrow_mut().mouse_button_up_event(ctx, button, x, y)
        }
    }
}

impl ILayout for Panel<'_> {
    fn set_rect(&mut self, rect: Rect) {
        for w in &self.widgets {
            w.borrow_mut().set_rect(rect)
        }
    }
    fn get_rect(&self) -> Rect {
        let mut max_rect = Rect::zero();
        for w in &self.widgets {
            max_rect = max_rect.combine_with(w.borrow().get_rect())
        }
        max_rect
    }
}

impl<'a> IActions<'a> for Panel<'a> {
    fn collect_fired(&mut self) -> Vec<Rc<dyn Fn() + 'a>> {
        let mut v = Vec::new();
        for w in &mut self.widgets {
            v.append(&mut w.borrow_mut().collect_fired());
        }
        v
    }
}

pub struct Builder<'a> {
    panel: Panel<'a>,
}

impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Self {
            panel: Panel::new(),
        }
    }

    pub fn build(self) -> Rc<RefCell<Panel<'a>>> {
        Rc::new(RefCell::new(self.panel))
    }

    pub fn add_widget(mut self, w: Rc<RefCell<dyn Widget<'a> + 'a>>) -> Self {
        self.panel.add_widget(w);
        self
    }
}
