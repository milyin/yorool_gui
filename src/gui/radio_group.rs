use crate::gui::{is_same, IActions, ICheckbox, ILayout};
use ggez::event::EventHandler;
use ggez::graphics::Rect;
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct RadioGroup<'a> {
    radios: Vec<Rc<RefCell<dyn ICheckbox<'a> + 'a>>>,
    owned_handler: Option<Rc<dyn Fn(Rc<RefCell<dyn ICheckbox<'a> + 'a>>) + 'a>>,
    rcself: Option<Weak<RefCell<Self>>>,
}

impl<'a> RadioGroup<'a> {
    fn new() -> Self {
        Self {
            radios: Vec::new(),
            owned_handler: None,
            rcself: None,
        }
    }

    fn rcself(&self) -> Rc<RefCell<Self>> {
        self.rcself.as_ref().unwrap().upgrade().unwrap().clone()
    }

    fn owned_handler(&mut self) -> Rc<dyn Fn(Rc<RefCell<dyn ICheckbox<'a> + 'a>>) + 'a> {
        if self.owned_handler.is_none() {
            let rcself = self.rcself();
            self.owned_handler = Some(Rc::new(move |c| {
                if c.borrow_mut().get_state() {
                    for r in &rcself.borrow_mut().radios {
                        if !is_same(r, &c) {
                            r.borrow_mut().set_state(false);
                        }
                    }
                } else {
                    c.borrow_mut().set_state(true);
                }
            }));
        }
        self.owned_handler.as_ref().unwrap().clone()
    }

    pub fn add_widget(&mut self, w: Rc<RefCell<dyn ICheckbox<'a> + 'a>>) {
        w.borrow_mut().on_changed_rc(self.owned_handler());
        self.radios.push(w);
    }

    pub fn remove_widget(&mut self, w: Rc<RefCell<dyn ICheckbox<'a> + 'a>>) {
        w.borrow_mut().remove_handler(self.owned_handler());
        self.radios.drain_filter(move |pw| is_same(pw, &w)).count();
    }

    pub fn radios<'b>(&'b self) -> &'b [Rc<RefCell<dyn ICheckbox<'a> + 'a>>] {
        self.radios.as_slice()
    }
}

impl EventHandler for RadioGroup<'_> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }
}

impl ILayout for RadioGroup<'_> {
    fn set_rect(&mut self, _rect: Rect) {}
    fn get_rect(&self) -> Rect {
        Rect::zero()
    }
}

impl<'a> IActions<'a> for RadioGroup<'a> {
    fn collect_fired(&mut self) -> Vec<Rc<dyn Fn() + 'a>> {
        Vec::new()
    }
}

pub struct RadioGroupBuilder<'a> {
    radio_group: Rc<RefCell<RadioGroup<'a>>>,
}

impl<'a> RadioGroupBuilder<'a> {
    pub fn new() -> Self {
        let radio_group = Rc::new(RefCell::new(RadioGroup::new()));
        radio_group.borrow_mut().rcself = Some(Rc::downgrade(&radio_group));
        Self { radio_group }
    }

    pub fn build(self) -> Rc<RefCell<RadioGroup<'a>>> {
        self.radio_group
    }

    pub fn add_widget(self, w: Rc<RefCell<dyn ICheckbox<'a> + 'a>>) -> Self {
        self.radio_group.borrow_mut().add_widget(w);
        self
    }
}
