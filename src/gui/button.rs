use crate::request::Handler;
use ggez::event::{EventHandler, MouseButton};
use ggez::{Context, GameResult};
use ggez::graphics::{self, Rect, MeshBuilder, DrawMode, DrawParam};
use crate::gui::{Layoutable, Widget};

#[derive(Debug)]
pub enum Message {
    ChangedState
}

#[derive(Debug, Clone)]
pub enum Command {
    SetState(bool)
}

pub trait IButton {
    fn get_state(&self) -> bool;
}

type ToMessage<MSG> = fn(Message) -> MSG;
type FromCommand<CMD> = fn(&CMD) -> Option<Command>;

pub struct Button<MSG,CMD> {
    checked: bool,
    touched: bool,
    rect: Rect,
    fm: ToMessage<MSG>,
    fc: FromCommand<CMD>,
}

impl<MSG,CMD> IButton for Button<MSG,CMD> {
    fn get_state(&self) -> bool { self.checked }
}

impl<MSG,CMD> Button<MSG,CMD>
{
    pub fn new(fm: ToMessage<MSG>, fc: FromCommand<CMD>) -> Self
    {
        Self {
            checked: false,
            touched: false,
            rect: Rect::zero(),
            fm, fc,
        }
    }

    fn handle_command(&mut self, cmd: Command) {
        match cmd {
            Command::SetState(v) => { self.checked = v; }
        }
    }
}

impl<MSG,CMD> Handler<MSG,CMD> for Button<MSG,CMD> where CMD: Clone
{
    fn collect(&mut self) -> Vec<MSG> {Vec::new()}
    fn handle(&mut self, cmds: &[CMD]) {
        for cmd in cmds {
            if let Some(cmd) = (self.fc)(cmd) {
                self.handle_command(cmd)
            }
        }
    }
}

impl<Q,R> EventHandler for Button<Q,R>
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

impl<Q,R> Layoutable for Button<Q,R> {
    fn set_rect(&mut self, x:f32, y:f32, w:f32, h:f32) {
        self.rect.x = x;
        self.rect.y = y;
        self.rect.w = w;
        self.rect.h = h;
    }
}
