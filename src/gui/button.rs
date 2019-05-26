use crate::gui::Layoutable;
use crate::request::{
    query_by_ctrlid, CtrlId, MessageHandler, MessagePoolIn, MessagePoolOut, Unpack, QR,
};
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{self, DrawMode, DrawParam, MeshBuilder, Rect};
use ggez::{Context, GameResult};

#[derive(Debug)]
pub enum Event {
    None,
    SetState(bool),
    GetState(QR<(), bool>),
    Changed,
}

impl Default for Event {
    fn default() -> Event {
        Event::None
    }
}

pub struct Button<MSG> {
    checked: bool,
    touched: bool,
    changed: bool,
    rect: Rect,
    ctrlid: CtrlId<MSG, Event>,
}

impl<MSG> Button<MSG> {
    pub fn new(ctrlid: CtrlId<MSG, Event>) -> Self {
        Self {
            checked: false,
            touched: false,
            changed: false,
            rect: Rect::zero(),
            ctrlid,
        }
    }
}

impl<MSG> MessageHandler<MSG> for Button<MSG>
where
    MSG: Unpack<Event>,
{
    fn handle(&mut self, src: &mut MessagePoolIn<MSG>, dst: &mut MessagePoolOut<MSG>) {
        for evt in query_by_ctrlid(src, self.ctrlid) {
            match evt {
                Event::SetState(v) => self.checked = v,
                Event::GetState(QR::Query(_)) => {
                    dst.push((self.ctrlid)(Event::GetState(QR::Response(self.checked))))
                }
                _ => {}
            }
        }
        if self.changed {
            self.changed = false;
            dst.push((self.ctrlid)(Event::Changed))
        }
    }
}

impl<MSG> EventHandler for Button<MSG> {
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
                if self.checked {
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
            self.checked = !self.checked;
            self.touched = false;
            self.changed = true;
        } else {
            self.touched = false;
        }
    }
}

impl<MSG> Layoutable for Button<MSG> {
    fn set_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.rect.x = x;
        self.rect.y = y;
        self.rect.w = w;
        self.rect.h = h;
    }
}
