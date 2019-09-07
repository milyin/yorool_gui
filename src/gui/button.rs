use crate::gui::{Executable, ILabel, Layoutable};
use crate::models::{
    collect_fired_actions, handler_id, Handler, HandlerId, TButtonBackend, TButtonFrontend,
    THandlers, TRcSelf,
};
use ggez::event::EventHandler;
use ggez::graphics::{self, Align, DrawMode, DrawParam, MeshBuilder, Rect, Text};
use ggez::input::mouse::MouseButton;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::{Rc, Weak};

pub struct Backend<'a> {
    label: String,
    touched: bool,
    on_click_handlers: HashMap<HandlerId, Handler<'a, Self>>,
    pending_handlers: Vec<Handler<'a, Self>>,
    rcself: Option<Weak<RefCell<Self>>>,
}

impl TRcSelf for Backend<'_> {
    fn create() -> Rc<RefCell<Self>> {
        let v = Rc::new(RefCell::new(Self {
            label: String::new(),
            touched: false,
            on_click_handlers: HashMap::new(),
            pending_handlers: Vec::new(),
            rcself: None,
        }));
        v.borrow_mut().rcself = Some(Rc::downgrade(&v.clone()));
        v
    }
    fn rcself(&self) -> Rc<RefCell<Self>> {
        self.rcself.as_ref().unwrap().upgrade().unwrap().clone()
    }
}

impl<'a> THandlers<'a> for Backend<'a> {
    fn remove_handler(&mut self, hid: HandlerId) {
        self.on_click_handlers.remove(&hid);
    }
    fn collect_fired_handlers(&mut self) -> Vec<Handler<'a, Self>> {
        self.pending_handlers.drain(..).collect()
    }
}

impl<'a> TButtonBackend<'a> for Backend<'a> {
    fn set_touched(&mut self, state: bool) {
        self.touched = state
    }
    fn is_touched(&self) -> bool {
        self.touched
    }
    fn click(&mut self) {
        let mut hs = self.on_click_handlers.values().map(|h| h.clone()).collect();
        self.pending_handlers.append(&mut hs);
    }
    fn on_click(&mut self, hid: HandlerId, h: Handler<'a, Self>) {
        self.on_click_handlers.insert(hid, h);
    }
}

impl<'a> ILabel<'a> for Backend<'a> {
    fn get_label(&self) -> String {
        self.label.clone()
    }
    fn set_label(&mut self, label: String) {
        self.label = label;
    }
}

pub struct Builder<'a, BE: TButtonBackend<'a>, FE: TButtonFrontend<'a, BE>> {
    rcfront: Rc<RefCell<FE>>,
    phantom: PhantomData<&'a BE>,
}

impl<'a, BE, FE> Builder<'a, BE, FE>
where
    BE: TButtonBackend<'a>,
    FE: TButtonFrontend<'a, BE>,
{
    pub fn builder() -> Self {
        Self {
            rcfront: FE::create(),
            phantom: PhantomData,
        }
    }
    pub fn set_label<S: Into<String>>(self, label: S) -> Self {
        self.rcfront
            .borrow_mut()
            .backend()
            .borrow_mut()
            .set_label(label.into());
        self
    }
    pub fn on_click(self, handler: impl Fn(Rc<RefCell<BE>>) + 'a) -> Self {
        let rh = Rc::new(move |w| handler(w));
        self.rcfront
            .borrow_mut()
            .backend()
            .borrow_mut()
            .on_click(handler_id(rh.clone()), rh.clone());
        self
    }
    pub fn build(self) -> Rc<RefCell<FE>> {
        self.rcfront
    }
}

pub struct Frontend<'a, BE: TButtonBackend<'a>> {
    rcback: Rc<RefCell<BE>>,
    rect: Rect,
    //    phantom: PhantomData<&'a T>,
    rcself: Option<Weak<RefCell<Self>>>,
}

impl<'a, BE> TRcSelf for Frontend<'a, BE>
where
    BE: TButtonBackend<'a>,
{
    fn create() -> Rc<RefCell<Self>> {
        let v = Rc::new(RefCell::new(Self {
            rcback: BE::create(),
            rect: Rect::zero(),
            rcself: None, //            phantom: PhantomData,
        }));
        v.borrow_mut().rcself = Some(Rc::downgrade(&v.clone()));
        v
    }
    fn rcself(&self) -> Rc<RefCell<Self>> {
        self.rcself.as_ref().unwrap().upgrade().unwrap().clone()
    }
}
impl<'a, BE> TButtonFrontend<'a, BE> for Frontend<'a, BE>
where
    BE: TButtonBackend<'a> + 'a,
{
    fn backend(&self) -> Rc<RefCell<BE>> {
        self.rcback.clone()
    }
}

impl<'a, T> EventHandler for Frontend<'a, T>
where
    T: TButtonBackend<'a>,
{
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

impl<'a, T> Layoutable for Frontend<'a, T>
where
    T: TButtonBackend<'a>,
{
    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }
    fn get_rect(&self) -> Rect {
        self.rect.clone()
    }
}

impl<'a, BE> Executable<'a> for Frontend<'a, BE>
where
    BE: TButtonBackend<'a> + 'a,
{
    fn take_to_execute(&mut self) -> Vec<Rc<dyn Fn() + 'a>> {
        collect_fired_actions(&mut *self.rcback.borrow_mut())
    }
}

/*
pub struct ButtonBuilder<'a> {
    button: Rc<RefCell<Button<'a>>>,
}

impl<'a> ButtonBuilder<'a> {
    pub fn new() -> Self {
        let button = Rc::new(RefCell::new(Button::new()));
        button.borrow_mut().rcself = Some(Rc::downgrade(&button));
        Self { button }
    }

    pub fn set_label<S: Into<String>>(self, label: S) -> Self {
        self.button.borrow_mut().set_label(label.into());
        self
    }

    pub fn on_click(self, handler: impl Fn(Rc<RefCell<Button<'a>>>) + 'a) -> Self {
        self.button.borrow_mut().on_click(handler);
        self
    }

    pub fn build(self) -> Rc<RefCell<Button<'a>>> {
        self.button
    }
}*/
