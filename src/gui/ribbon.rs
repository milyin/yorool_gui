use crate::gui::{is_same, Widget};
use crate::gui::{IActions, ILayout};
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::Rect;
use ggez::{Context, GameResult};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Ribbon<'a> {
    widgets: Vec<Rc<RefCell<dyn Widget<'a> + 'a>>>,
    rect: Rect,
    horizontal: bool,
    rcself: Option<Weak<RefCell<Self>>>,
}

impl<'a> Ribbon<'a> {
    fn new() -> Self {
        Self {
            widgets: Vec::new(),
            rect: Rect::zero(),
            horizontal: true,
            rcself: None,
        }
    }

    pub fn set_horizontal(&mut self, horizontal: bool) -> &mut Self {
        self.horizontal = horizontal;
        self
    }

    pub fn is_horizontal(&self) -> bool {
        self.horizontal
    }

    pub fn add_widget(&mut self, widget: Rc<RefCell<dyn Widget<'a> + 'a>>) -> &mut Self {
        self.widgets.push(widget);
        self
    }

    pub fn remove_widget<T: ?Sized>(&mut self, w: Rc<RefCell<T>>) -> &mut Self {
        self.widgets.drain_filter(|pw| is_same(pw, &w)).count();
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

impl ILayout for Ribbon<'_> {
    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
        if self.horizontal {
            let dw = self.rect.w / self.widgets.len() as f32;
            let mut x = self.rect.x;
            self.for_all(|wgt| {
                wgt.borrow_mut()
                    .set_rect(Rect::new(x, self.rect.y, dw, self.rect.h));
                x += dw;
            });
        } else {
            let dh = self.rect.h / self.widgets.len() as f32;
            let mut y = self.rect.y;
            self.for_all(|wgt| {
                wgt.borrow_mut()
                    .set_rect(Rect::new(self.rect.x, y, self.rect.w, dh));
                y += dh;
            });
        }
    }
    fn get_rect(&self) -> Rect {
        self.rect.clone()
    }
}

impl<'a> IActions<'a> for Ribbon<'a> {
    fn collect_fired(&mut self) -> Vec<Rc<dyn Fn() + 'a>> {
        let mut v = Vec::new();
        for w in &mut self.widgets {
            v.append(&mut w.borrow_mut().collect_fired());
        }
        v
    }
}

pub struct Builder<'a> {
    ribbon: Ribbon<'a>,
}

impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Self {
            ribbon: Ribbon::new(),
        }
    }

    pub fn build(self) -> Rc<RefCell<Ribbon<'a>>> {
        let rc = Rc::new(RefCell::new(self.ribbon));
        rc.borrow_mut().rcself = Some(Rc::downgrade(&rc));
        rc
    }

    pub fn set_horizontal(mut self, horizontal: bool) -> Self {
        self.ribbon.set_horizontal(horizontal);
        self
    }

    pub fn add_widget(mut self, w: Rc<RefCell<dyn Widget<'a> + 'a>>) -> Self {
        self.ribbon.add_widget(w);
        self
    }
}
