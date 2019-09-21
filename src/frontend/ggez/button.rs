use crate::backend::button::{Backend, IBackend};
use crate::gui::{ILayout, TRcSelf};
use ggez::event::EventHandler;
use ggez::graphics::{self, Align, DrawMode, DrawParam, MeshBuilder, Rect, Text};
use ggez::input::mouse::MouseButton;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Frontend<'a> {
    rcback: Rc<RefCell<dyn IBackend<'a> + 'a>>,
    rect: Rect,
    rcself: Option<Weak<RefCell<Self>>>,
}

impl<'a> Frontend<'a> {
    pub fn backend(&self) -> Rc<RefCell<dyn IBackend<'a> + 'a>> {
        self.rcback.clone()
    }
}

impl<'a> TRcSelf for Frontend<'a> {
    fn create() -> Rc<RefCell<Self>> {
        let v = Rc::new(RefCell::new(Self {
            rcback: Backend::create(),
            rect: Rect::zero(),
            rcself: None, //            phantom: PhantomData,
        }));
        v.borrow_mut().rcself = Some(Rc::downgrade(&v.clone()));
        v
    }
    fn wrcself(&self) -> Weak<RefCell<Self>> {
        self.rcself.as_ref().unwrap().clone()
    }
}
impl<'a> EventHandler for Frontend<'a> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut rect = self.rect;
        let margin = 5.;
        let press_offset = 10.;
        let dxy = if self.rcback.borrow_mut().is_touched() {
            press_offset
        } else {
            0.
        };
        rect.x += margin + dxy;
        rect.y += margin + dxy;
        rect.w -= margin * 2. + press_offset;
        rect.h -= margin * 2. + press_offset;
        let mesh = MeshBuilder::new()
            .rectangle(DrawMode::fill(), rect, graphics::WHITE)
            .build(ctx)?;
        graphics::draw(ctx, &mesh, DrawParam::default())?;
        let mut text = Text::new(self.rcback.borrow_mut().get_label());
        text.set_bounds([rect.w, rect.h], Align::Center);
        let tdh = (rect.h - text.height(ctx) as f32) / 2.;
        graphics::draw(
            ctx,
            &text,
            (Point2::new(rect.x, rect.y + tdh), graphics::BLACK),
        )
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left && self.rect.contains([x, y]) {
            self.rcback.borrow_mut().set_touched(true);
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        let mut rcback = self.rcback.borrow_mut();
        if rcback.is_touched() && self.rect.contains([x, y]) {
            rcback.set_touched(false);
            rcback.click();
        } else {
            rcback.set_touched(false);
        }
    }
}

impl<'a> ILayout for Frontend<'a> {
    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }
    fn get_rect(&self) -> Rect {
        self.rect.clone()
    }
}
