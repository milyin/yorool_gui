use crate::gui::{Executable, Layoutable, Widget};
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::Rect;
use ggez::{Context, GameResult};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Panel<'a> {
    widget: Option<Rc<RefCell<dyn Widget<'a> + 'a>>>,
}

impl<'a> Panel<'a> {
    pub fn new() -> Self {
        Self { widget: None }
    }

    pub fn set_widget(&mut self, w: Rc<RefCell<dyn Widget<'a> + 'a>>) -> &mut Self {
        self.widget = Some(w);
        self
    }
}

impl EventHandler for Panel<'_> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.widget.as_ref().unwrap().borrow_mut().update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.widget.as_ref().unwrap().borrow_mut().draw(ctx)
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.widget
            .as_ref()
            .unwrap()
            .borrow_mut()
            .mouse_button_down_event(ctx, button, x, y)
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.widget
            .as_ref()
            .unwrap()
            .borrow_mut()
            .mouse_button_up_event(ctx, button, x, y)
    }
}

impl Layoutable for Panel<'_> {
    fn set_rect(&mut self, rect: Rect) {
        self.widget.as_ref().unwrap().borrow_mut().set_rect(rect)
    }
    fn get_rect(&self) -> Rect {
        self.widget.as_ref().unwrap().borrow().get_rect()
    }
}

impl<'a> Executable<'a> for Panel<'a> {
    fn to_execute(&mut self) -> Vec<Rc<dyn Fn() + 'a>> {
        self.widget.as_ref().unwrap().borrow_mut().to_execute()
    }
}

pub struct PanelBuilder<'a> {
    panel: Panel<'a>,
}

impl<'a> PanelBuilder<'a> {
    pub fn new() -> Self {
        Self {
            panel: Panel::new(),
        }
    }

    pub fn build(self) -> Rc<RefCell<Panel<'a>>> {
        Rc::new(RefCell::new(self.panel))
    }

    pub fn set_widget(mut self, w: Rc<RefCell<dyn Widget<'a> + 'a>>) -> Self {
        self.panel.set_widget(w);
        self
    }
}
