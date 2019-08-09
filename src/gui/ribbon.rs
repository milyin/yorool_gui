use crate::gui::Widget;
use crate::gui::{Executable, Layoutable};
use crate::request::MessageSender;
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::Rect;
use ggez::{Context, GameResult};
use std::cell::RefCell;
use std::rc::Rc;

pub type Event = ();

pub struct Ribbon<'a, MSG> {
    widgets: Vec<Rc<RefCell<dyn Widget<'a, MSG> + 'a>>>,
    rect: Rect,
    horizontal: bool,
}

impl<'a, MSG> Ribbon<'a, MSG> {
    pub fn new(horizontal: bool) -> Self {
        Self {
            widgets: Vec::new(),
            rect: Rect::zero(),
            horizontal,
        }
    }

    pub fn add_widget(mut self, widget: impl Widget<'a, MSG> + 'a) -> Self {
        self.widgets.push(Rc::new(RefCell::new(widget)));
        self
    }

    pub fn add_widget_rc(mut self, widget: Rc<RefCell<impl Widget<'a, MSG> + 'a>>) -> Self {
        self.widgets.push(widget.clone());
        self
    }

    fn for_all_res<F: FnMut(Rc<RefCell<dyn Widget<'a, MSG> + 'a>>) -> GameResult>(
        &self,
        mut f: F,
    ) -> GameResult {
        for w in &self.widgets {
            f(w.clone())?
        }
        Ok(())
    }

    fn for_all<F: FnMut(Rc<RefCell<dyn Widget<MSG> + 'a>>)>(&self, mut f: F) {
        for w in &self.widgets {
            f(w.clone())
        }
    }

    fn for_first<RES, F: FnMut(Rc<RefCell<dyn Widget<MSG> + 'a>>) -> Option<RES>>(
        &self,
        mut f: F,
    ) -> Option<RES> {
        for w in &self.widgets {
            if let Some(res) = f(w.clone()) {
                return Some(res);
            }
        }
        None
    }
}

impl<MSG> MessageSender<MSG> for Ribbon<'_, MSG> {
    fn get_message(&mut self) -> Option<MSG> {
        self.for_first(|w| w.borrow_mut().get_message())
    }
}

impl<MSG> EventHandler for Ribbon<'_, MSG> {
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

impl<MSG> Layoutable for Ribbon<'_, MSG> {
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

impl<'a, MSG> Executable<'a> for Ribbon<'a, MSG> {
    fn to_execute(&mut self) -> Vec<Rc<dyn Fn() + 'a>> {
        let mut v = Vec::new();
        for w in &mut self.widgets {
            v.append(&mut w.borrow_mut().to_execute());
        }
        v
    }
}
