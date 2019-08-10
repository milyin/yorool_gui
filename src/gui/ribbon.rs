use crate::gui::Widget;
use crate::gui::{Executable, Layoutable};
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::Rect;
use ggez::{Context, GameResult};
use std::cell::RefCell;
use std::rc::Rc;

pub type Event = ();

pub struct Ribbon<'a> {
    widgets: Vec<Rc<RefCell<dyn Widget<'a> + 'a>>>,
    rect: Rect,
    horizontal: bool,
}

impl<'a> Ribbon<'a> {
    pub fn new(horizontal: bool) -> Self {
        Self {
            widgets: Vec::new(),
            rect: Rect::zero(),
            horizontal,
        }
    }

    pub fn add_widget(mut self, widget: impl Widget<'a> + 'a) -> Self {
        self.widgets.push(Rc::new(RefCell::new(widget)));
        self
    }

    pub fn add_widget_rc(mut self, widget: Rc<RefCell<impl Widget<'a> + 'a>>) -> Self {
        self.widgets.push(widget.clone());
        self
    }

    fn for_all_res<F: FnMut(Rc<RefCell<dyn Widget<'a> + 'a>>) -> GameResult>(
        &self,
        mut f: F,
    ) -> GameResult {
        for w in &self.widgets {
            f(w.clone())?
        }
        Ok(())
    }

    fn for_all<F: FnMut(Rc<RefCell<dyn Widget + 'a>>)>(&self, mut f: F) {
        for w in &self.widgets {
            f(w.clone())
        }
    }
}

impl EventHandler for Ribbon<'_> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.for_all_res(|w| w.borrow_mut().update(ctx))
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.for_all_res(|w| w.borrow_mut().draw(ctx))
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.for_all(|w| w.borrow_mut().mouse_button_down_event(ctx, button, x, y))
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.for_all(|w| w.borrow_mut().mouse_button_up_event(ctx, button, x, y))
    }
}

impl Layoutable for Ribbon<'_> {
    fn set_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.rect.x = x;
        self.rect.y = y;
        self.rect.w = w;
        self.rect.h = h;
        if self.horizontal {
            let dw = w / self.widgets.len() as f32;
            let mut x = x;
            self.for_all(|wgt| {
                wgt.borrow_mut().set_rect(x, y, dw, h);
                x += dw;
            });
        } else {
            let dh = h / self.widgets.len() as f32;
            let mut y = y;
            self.for_all(|wgt| {
                wgt.borrow_mut().set_rect(x, y, w, dh);
                y += dh;
            });
        }
    }
}

impl<'a> Executable<'a> for Ribbon<'a> {
    fn to_execute(&mut self) -> Vec<Rc<dyn Fn() + 'a>> {
        let mut v = Vec::new();
        for w in &mut self.widgets {
            v.append(&mut w.borrow_mut().to_execute());
        }
        v
    }
}
