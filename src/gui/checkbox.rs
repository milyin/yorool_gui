use crate::gui::Layoutable;
use crate::request::{CtrlId, MessageSender, Unpack};
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{self, DrawMode, DrawParam, MeshBuilder, Rect};
use ggez::{Context, GameResult};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Event {
    None,
    Init,
    Pressed,
}

impl Default for Event {
    fn default() -> Event {
        Event::None
    }
}

pub struct Checkbox<MSG> {
    state: bool,
    touched: bool,
    notifications: Vec<MSG>,
    rect: Rect,
    ctrlid: CtrlId<MSG, Event>,
}

impl<MSG> Checkbox<MSG> {
    pub fn new(ctrl: fn(Event) -> MSG) -> Self {
        Self {
            state: false,
            touched: false,
            notifications: vec![ctrl(Event::Init)],
            rect: Rect::zero(),
            ctrlid: ctrl.into(),
        }
    }
}

impl<MSG> MessageSender<MSG> for Checkbox<MSG>
where
    MSG: Unpack<Event>,
{
    fn get_message(&mut self) -> Option<MSG> {
        self.notifications.pop()
    }
}

impl<MSG> EventHandler for Checkbox<MSG> {
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
            self.notifications.push(self.ctrlid.tomsg(Event::Pressed));
        } else {
            self.touched = false;
        }
    }
}

impl<MSG> Layoutable for Checkbox<MSG> {
    fn set_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.rect.x = x;
        self.rect.y = y;
        self.rect.w = w;
        self.rect.h = h;
    }
}
