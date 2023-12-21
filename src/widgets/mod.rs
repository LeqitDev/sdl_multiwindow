use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use dyn_clone::DynClone;
use sdl2::{event::Event, rect::Rect, render::Canvas, ttf::Sdl2TtfContext, video::Window};

use crate::window::MyWindow;

pub mod button;
pub mod list;
pub mod scrollview;
pub mod text;

pub trait Widget: DynClone {
    fn get_id(&self) -> u32;
    fn draw(&mut self, canvas: &mut RefMut<Canvas<Window>>);
    fn event(&mut self, _event: Event, _ctx: &MyWindow) {}
    fn set_rect(&mut self, _rect: Rect);
    fn get_rect(&self) -> Rect;
    fn has_changed(&mut self) -> bool {
        false
    }
    fn give_viewport(&mut self, _viewport: Rect) {}
}

dyn_clone::clone_trait_object!(Widget);
