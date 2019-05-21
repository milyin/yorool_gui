use crate::request::{MessageHandler, IMessageHandler};
use ggez::event::{EventHandler, MouseButton};
use ggez::{Context, GameResult};
use ggez::graphics::{self, Rect, MeshBuilder, DrawMode, DrawParam};
use crate::gui::{Layoutable};
use crate::request;

pub enum Message {
    None
}

pub type Event = request::Event<Message,bool>;

type FPack<MSG> = fn(Event) -> MSG;
type FUnpack<MSG> = fn(MSG) -> Result<Event,MSG>;

pub struct Button<MSG> {
    checked: bool,
    touched: bool,
    changed: bool,
    rect: Rect,
    fpack: FPack<MSG>,
    funpack: FUnpack<MSG>,
}

impl<MSG> Button<MSG>
{
    pub fn new(fpack: FPack<MSG>, funpack: FUnpack<MSG>) -> Self
    {
        Self {
            checked: false,
            touched: false,
            changed: false,
            rect: Rect::zero(),
            fpack, funpack,
        }
    }
}

impl<MSG> MessageHandler<MSG> for Button<MSG> {
    type T = Message;
    type S = bool;
    fn pack(&self, e: Event) -> Option<MSG> { Some((self.fpack)(e)) }
    fn unpack(&self, m: MSG) -> Result<Event, MSG> { (self.funpack)(m) }
    fn handle_custom(&mut self, m: Message) -> Option<Message> { None }
    fn get_state(&self) -> bool { self.checked }
    fn set_state(&mut self, s: bool) { self.checked = s }
    fn collect(&mut self) -> Vec<MSG> {
        let mut ret = Vec::new();
        if self.changed {
            self.changed = false;
            ret.push((self.fpack)(request::Event::Changed))
        }
        ret
    }
}

impl<MSG> EventHandler for Button<MSG>
{
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut rect = self.rect;
        let dsz = if self.touched { 10.} else { 5. };
        rect.x += dsz;
        rect.y += dsz;
        rect.w -= dsz * 2.;
        rect.h -= dsz * 2.;
        let mesh = MeshBuilder::new()
            .rectangle(if self.checked { DrawMode::fill() } else { DrawMode::stroke(1.) },
                       rect, graphics::WHITE)
            .build(ctx)?;
        graphics::draw(
            ctx,
            &mesh,
            DrawParam::default()
        )
    }

    fn mouse_button_down_event(
        &mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32,
    ) {
        if button == MouseButton::Left && self.rect.contains([x,y]) {
            self.touched = true;
        }
    }

    fn mouse_button_up_event(
        &mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32,
    ) {
        if self.touched && self.rect.contains([x,y]) {
            self.checked = !self.checked;
            self.touched = false;
            self.changed = true;
        } else {
            self.touched = false;
        }
    }
}

impl<MSG> Layoutable for Button<MSG> {
    fn set_rect(&mut self, x:f32, y:f32, w:f32, h:f32) {
        self.rect.x = x;
        self.rect.y = y;
        self.rect.w = w;
        self.rect.h = h;
    }
}
