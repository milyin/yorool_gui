use crate::gui::{Executable, Layoutable};
use ggez::event::EventHandler;
use ggez::graphics::{self, DrawMode, DrawParam, MeshBuilder, Rect};
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Button<'a> {
    touched: bool,
    rect: Rect,
    on_click_handlers: Vec<Rc<dyn Fn(Rc<RefCell<Self>>) + 'a>>,
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

    fn rcself(&self) -> Rc<RefCell<Self>> {
        self.rcself.as_ref().unwrap().upgrade().unwrap().clone()
    }

    pub fn on_click(
        &mut self,
        handler: impl Fn(Rc<RefCell<Self>>) + 'a,
    ) -> Rc<dyn Fn(Rc<RefCell<Self>>) + 'a> {
        let rc = Rc::new(handler);
        self.on_click_rc(rc.clone());
        rc
    }

    pub fn on_click_rc(&mut self, handler: Rc<dyn Fn(Rc<RefCell<Self>>) + 'a>) {
        self.on_click_handlers.push(handler)
    }

    fn fire_on_click(&mut self) {
        for h in &self.on_click_handlers {
            let rcself = self.rcself();
            let hc = h.clone();
            self.pending_handlers
                .push(Rc::new(move || hc(rcself.clone())));
        }
    }
}

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
            self.fire_on_click();
        } else {
            self.touched = false;
        }
    }
}

impl Layoutable for Button<'_> {
    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }
    fn get_rect(&self) -> Rect {
        self.rect.clone()
    }
}

impl<'a> Executable<'a> for Button<'a> {
    fn take_to_execute(&mut self) -> Vec<Rc<dyn Fn() + 'a>> {
        self.pending_handlers.drain(..).collect()
    }
}

pub struct ButtonBuilder<'a> {
    button: Rc<RefCell<Button<'a>>>,
}

impl<'a> ButtonBuilder<'a> {
    pub fn new() -> Self {
        let button = Rc::new(RefCell::new(Button::new()));
        button.borrow_mut().rcself = Some(Rc::downgrade(&button));
        Self { button }
    }

    pub fn on_click(self, handler: impl Fn(Rc<RefCell<Button<'a>>>) + 'a) -> Self {
        self.button.borrow_mut().on_click(handler);
        self
    }

    pub fn build(self) -> Rc<RefCell<Button<'a>>> {
        self.button
    }
}
