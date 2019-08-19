use crate::gui::{is_same, Executable, ICheckbox, Layoutable};
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{self, DrawMode, DrawParam, MeshBuilder, Rect};
use ggez::{Context, GameResult};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Checkbox<'a> {
    state: bool,
    touched: bool,
    rect: Rect,
    on_changed_self_handlers: Vec<Rc<dyn Fn(Rc<RefCell<Self>>) + 'a>>,
    on_changed_icheckbox_handlers: Vec<Rc<dyn Fn(Rc<RefCell<dyn ICheckbox<'a> + 'a>>) + 'a>>,
    pending_handlers: Vec<Rc<dyn Fn() + 'a>>,
    rcself: Option<Weak<RefCell<Self>>>,
}

impl<'a> Checkbox<'a> {
    fn new() -> Self {
        Self {
            state: false,
            touched: false,
            rect: Rect::zero(),
            on_changed_self_handlers: Vec::new(),
            on_changed_icheckbox_handlers: Vec::new(),
            pending_handlers: Vec::new(),
            rcself: None,
        }
    }

    fn rcself(&self) -> Rc<RefCell<Self>> {
        self.rcself.as_ref().unwrap().upgrade().unwrap().clone()
    }

    pub fn on_changed(
        &mut self,
        handler: impl Fn(Rc<RefCell<Self>>) + 'a,
    ) -> Rc<dyn Fn(Rc<RefCell<Self>>) + 'a> {
        let rc = Rc::new(handler);
        self.on_changed_rc(rc.clone());
        rc
    }
    pub fn on_changed_rc(&mut self, handler: Rc<dyn Fn(Rc<RefCell<Self>>) + 'a>) {
        self.on_changed_self_handlers.push(handler);
    }
    pub fn remove_handler<T: ?Sized>(&mut self, handler: Rc<T>) {
        self.on_changed_self_handlers
            .drain_filter(move |w| is_same(w, &handler))
            .count();
    }

    fn fire_on_changed(&mut self) {
        for h in &self.on_changed_self_handlers {
            let rcself = self.rcself();
            let hc = h.clone();
            self.pending_handlers
                .push(Rc::new(move || hc(rcself.clone())));
        }
        for h in &self.on_changed_icheckbox_handlers {
            let rcself = self.rcself();
            let hc = h.clone();
            self.pending_handlers
                .push(Rc::new(move || hc(rcself.clone())));
        }
    }
}

impl<'a> ICheckbox<'a> for Checkbox<'a> {
    fn get_state(&self) -> bool {
        self.state
    }
    fn set_state(&mut self, state: bool) {
        self.state = state;
    }
    fn on_changed_rc(&mut self, handler: Rc<dyn Fn(Rc<RefCell<dyn ICheckbox<'a> + 'a>>) + 'a>) {
        self.on_changed_icheckbox_handlers.push(handler);
    }
    fn remove_handler(&mut self, handler: Rc<dyn Fn(Rc<RefCell<dyn ICheckbox<'a> + 'a>>) + 'a>) {
        self.on_changed_icheckbox_handlers
            .drain_filter(move |w| is_same(w, &handler))
            .count();
    }
}

impl EventHandler for Checkbox<'_> {
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
            self.fire_on_changed();
        } else {
            self.touched = false;
        }
    }
}

impl Layoutable for Checkbox<'_> {
    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }
    fn get_rect(&self) -> Rect {
        self.rect.clone()
    }
}

impl<'a> Executable<'a> for Checkbox<'a> {
    fn take_to_execute(&mut self) -> Vec<Rc<dyn Fn() + 'a>> {
        self.pending_handlers.drain(..).collect()
    }
}

pub struct CheckboxBuilder<'a> {
    ribbon: Checkbox<'a>,
}

impl<'a> CheckboxBuilder<'a> {
    pub fn new() -> Self {
        Self {
            ribbon: Checkbox::new(),
        }
    }

    pub fn build(self) -> Rc<RefCell<Checkbox<'a>>> {
        let rc = Rc::new(RefCell::new(self.ribbon));
        rc.borrow_mut().rcself = Some(Rc::downgrade(&rc));
        rc
    }
}
