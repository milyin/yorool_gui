use crate::gui::{Executable, Layoutable};
use ggez::event::EventHandler;
use ggez::graphics::{self, DrawMode, DrawParam, MeshBuilder, Rect};
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};
use std::rc::Rc;

pub struct Button {
    touched: bool,
    pressed: bool,
    rect: Rect,
}

impl Button {
    pub fn new() -> Self {
        Self {
            touched: false,
            pressed: false,
            rect: Rect::zero(),
        }
    }
}

impl EventHandler for Button {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut rect = self.rect;
        let margin = 5.;
        let press_offset = 10.;
        let dxy = if self.touched { press_offset } else { 0. };
        rect.x += margin + dxy;
        rect.y += margin + dxy;
        rect.w -= margin * 2. + press_offset;
        rect.h -= margin * 2. + press_offset;
        let mesh = MeshBuilder::new()
            .rectangle(DrawMode::fill(), rect, graphics::WHITE)
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
            self.pressed = true;
            self.touched = false;
        } else {
            self.touched = false;
        }
    }
}

impl Layoutable for Button {
    fn set_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.rect.x = x;
        self.rect.y = y;
        self.rect.w = w;
        self.rect.h = h;
    }
}

impl<'a> Executable<'a> for Button {
    fn to_execute(&mut self) -> Vec<Rc<dyn Fn() + 'a>> {
        Vec::new()
    }
}
