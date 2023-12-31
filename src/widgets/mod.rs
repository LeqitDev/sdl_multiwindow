use std::cell::RefMut;

use dyn_clone::DynClone;
use sdl2::{event::Event, rect::Rect, render::Canvas, video::Window};

use crate::{window::MyWindow, Action};

pub mod button;
pub mod list;
pub mod scrollview;
pub mod text;
pub mod circle;

pub trait Widget: DynClone {
    fn draw(&mut self, canvas: &mut RefMut<Canvas<Window>>);
    fn event(&mut self, _event: Event, _ctx: &MyWindow) -> Action {Action::None}
    fn set_rect(&mut self, _rect: Rect);
    fn get_rect(&self) -> Rect;
    fn has_changed(&mut self) -> bool {
        false
    }
    fn give_viewport(&mut self, _viewport: Rect) {}
    fn set_window(&mut self, _win: &mut MyWindow) {}
}

dyn_clone::clone_trait_object!(Widget);
