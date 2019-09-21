use crate::gui::ILayout;
use ggez::event::EventHandler;

pub mod button;

pub trait Widget: EventHandler + ILayout {}

impl<W> Widget for W where W: EventHandler + ILayout {}
