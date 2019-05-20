use crate::request::Handler;
use ggez::event::{EventHandler, MouseButton};
use ggez::{Context, GameResult};
use ggez::graphics::{self, Rect, MeshBuilder, DrawMode, DrawParam};
use crate::gui::{Layoutable};

#[derive(Debug)]
pub enum Message {
    Notification(Notification),
    Command(Command)
}

#[derive(Debug)]
pub enum Notification {
    ChangedState
}

#[derive(Debug)]
pub enum Command {
    SetState(bool)
}

pub trait IButton {
    fn get_state(&self) -> bool;
}

type FPack<MSG> = fn(Message) -> MSG;
type FUnpack<MSG> = fn(MSG) -> Result<Message,MSG>;

pub struct Button<MSG> {
    checked: bool,
    touched: bool,
    rect: Rect,
    fpack: FPack<MSG>,
    funpack: FUnpack<MSG>,
}

impl<MSG> IButton for Button<MSG> {
    fn get_state(&self) -> bool { self.checked }
}

impl<MSG> Button<MSG>
{
    pub fn new(fpack: FPack<MSG>, funpack: FUnpack<MSG>) -> Self
    {
        Self {
            checked: false,
            touched: false,
            rect: Rect::zero(),
            fpack, funpack,
        }
    }

    fn handle_command(&mut self, cmd: Command) {
        match cmd {
            Command::SetState(v) => { self.checked = v; }
        }
    }

}

impl<MSG> Handler<MSG> for Button<MSG>
{
    fn collect(&mut self) -> Vec<MSG> {Vec::new()}
    fn handle(&mut self, msgs: Vec<MSG>) -> Vec<MSG> {
        let mut ret = Vec::new();
        for msg in msgs {
            match (self.funpack)(msg) {
                Ok(Message::Command(cmd)) => self.handle_command(cmd),
                Ok(Message::Notification(_)) => {},
                Err(msg) => ret.push(msg)
            }
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
