use crate::gui::{Executable, Layoutable, Widget};
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{self, DrawMode, DrawParam, MeshBuilder, Rect};
use ggez::{Context, GameResult};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub trait ICheckbox {
    fn get_state(&self) -> bool;
    fn set_state(&mut self, state: bool);
}

type Handler<'a> =
    Rc<dyn Fn(Rc<RefCell<dyn Widget<'a> + 'a>>, Rc<RefCell<dyn ICheckbox + 'a>>) + 'a>;

pub struct Checkbox<'a> {
    state: bool,
    touched: bool,
    rect: Rect,
    on_changed_handlers: Vec<Handler<'a>>,
    pending_handlers: Vec<Rc<dyn Fn() + 'a>>,
    rcself: Option<Weak<RefCell<Self>>>,
}

impl<'a> Checkbox<'a> {
    fn new() -> Self {
        Self {
            state: false,
            touched: false,
            rect: Rect::zero(),
            on_changed_handlers: Vec::new(),
            pending_handlers: Vec::new(),
            rcself: None,
        }
    }

    pub fn on_changed(&mut self, handler: Handler<'a>) {
        self.on_changed_handlers.push(handler);
    }
}

impl ICheckbox for Checkbox<'_> {
    fn get_state(&self) -> bool {
        self.state
    }
    fn set_state(&mut self, state: bool) {
        self.state = state;
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
            for h in &self.on_changed_handlers {
                let rcself = self.rcself.as_ref().unwrap().upgrade().unwrap();
                let hc = h.clone();
                self.pending_handlers
                    .push(Rc::new(move || hc(rcself.clone(), rcself.clone())));
            }
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
    fn to_execute(&mut self) -> Vec<Rc<dyn Fn() + 'a>> {
        self.pending_handlers.drain(..).collect()
    }
}
/*
pub fn make_radio<'a>(checkboxes: Vec<Rc<RefCell<Checkbox<'a>>>>) {
    for n in 0..checkboxes.len() {
        let (head, curr_tail) = checkboxes.split_at(n);
        let (curr, tail) = curr_tail.split_first().unwrap();
        let handler = {
            let curr = curr.clone();
            let others = head
                .iter()
                .chain(tail.iter())
                .map(|v| v.clone())
                .collect::<Vec<_>>();
            move || {
                let mut curr = curr.borrow_mut();
                if curr.get_state() {
                    for w in &others {
                        w.borrow_mut().set_state(false);
                    }
                } else {
                    curr.set_state(true);
                }
            }
        };
        curr.borrow_mut().on_changed(handler);
    }
}
*/
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
