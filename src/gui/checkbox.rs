use crate::gui::{Executable, Layoutable};
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{self, DrawMode, DrawParam, MeshBuilder, Rect};
use ggez::{Context, GameResult};
use std::rc::Rc;

pub struct Checkbox<'a> {
    state: bool,
    touched: bool,
    rect: Rect,
    on_changed_handlers: Vec<Rc<dyn Fn() + 'a>>,
    pending_handlers: Vec<Rc<dyn Fn() + 'a>>,
}

impl<'a> Checkbox<'a> {
    pub fn new() -> Self {
        Self {
            state: false,
            touched: false,
            rect: Rect::zero(),
            on_changed_handlers: Vec::new(),
            pending_handlers: Vec::new(),
        }
    }

    pub fn get_state(&self) -> bool {
        self.state
    }

    pub fn set_state(&mut self, state: bool) {
        self.state = state;
    }

    pub fn on_changed<F: Fn() + 'a>(&mut self, handler: F) {
        self.on_changed_handlers.push(Rc::new(handler));
    }
}

impl EventHandler for Checkbox<'_> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut rect = self.rect;
        let dsz = if self.touched { 10. } else { 5. };
        rect.x += dsz;
        rect.y += dsz;
        rect.w -= dsz * 2.;
        rect.h -= dsz * 2.;
        let mesh = MeshBuilder::new()
            .rectangle(
                if self.state {
                    DrawMode::fill()
                } else {
                    DrawMode::stroke(1.)
                },
                rect,
                graphics::WHITE,
            )
            .build(ctx)?;
        graphics::draw(ctx, &mesh, DrawParam::default())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left && self.rect.contains([x, y]) {
            self.touched = true;
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        if self.touched && self.rect.contains([x, y]) {
            self.state = !self.state;
            self.touched = false;
            for h in &self.on_changed_handlers {
                self.pending_handlers.push(h.clone());
            }
        } else {
            self.touched = false;
        }
    }
}

impl Layoutable for Checkbox<'_> {
    fn set_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.rect.x = x;
        self.rect.y = y;
        self.rect.w = w;
        self.rect.h = h;
    }
}

impl<'a> Executable<'a> for Checkbox<'a> {
    fn to_execute(&mut self) -> Vec<Rc<dyn Fn() + 'a>> {
        self.pending_handlers.drain(..).collect()
    }
}
