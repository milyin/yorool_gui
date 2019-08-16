use crate::gui::{Executable, Layoutable};
use ggez::event::EventHandler;
use ggez::graphics::{self, DrawMode, DrawParam, MeshBuilder, Rect};
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub trait IButton: Layoutable {}

type Handler<'a> = Rc<dyn Fn(Rc<RefCell<dyn IButton + 'a>>) + 'a>;

pub struct Button<'a> {
    touched: bool,
    rect: Rect,
    on_click_handlers: Vec<Handler<'a>>,
    pending_handlers: Vec<Rc<dyn Fn() + 'a>>,
    rcself: Option<Weak<RefCell<Self>>>,
}

impl<'a> Button<'a> {
    fn new() -> Self {
        Self {
            touched: false,
            rect: Rect::zero(),
            on_click_handlers: Vec::new(),
            pending_handlers: Vec::new(),
            rcself: None,
        }
    }

    pub fn on_click(&mut self, handler: Handler<'a>) {
        self.on_click_handlers.push(handler)
    }
}

impl IButton for Button<'_> {}

impl<'a> EventHandler for Button<'a> {
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
            self.touched = false;
            for h in &self.on_click_handlers {
                let rcself = self.rcself.as_ref().unwrap().upgrade().unwrap();
                let hc = h.clone();
                self.pending_handlers
                    .push(Rc::new(move || hc(rcself.clone())));
            }
        } else {
            self.touched = false;
        }
    }
}

impl Layoutable for Button<'_> {
    fn set_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.rect.x = x;
        self.rect.y = y;
        self.rect.w = w;
        self.rect.h = h;
    }
}

impl<'a> Executable<'a> for Button<'a> {
    fn to_execute(&mut self) -> Vec<Rc<dyn Fn() + 'a>> {
        self.pending_handlers.drain(..).collect()
    }
}

pub struct ButtonBuilder<'a> {
    ribbon: Button<'a>,
}

impl<'a> ButtonBuilder<'a> {
    pub fn new() -> Self {
        Self {
            ribbon: Button::new(),
        }
    }

    pub fn build(self) -> Rc<RefCell<Button<'a>>> {
        let rc = Rc::new(RefCell::new(self.ribbon));
        rc.borrow_mut().rcself = Some(Rc::downgrade(&rc));
        rc
    }
}
