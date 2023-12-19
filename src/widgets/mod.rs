use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use dyn_clone::DynClone;
use sdl2::{
    mouse::MouseWheelDirection, rect::Rect, render::Canvas, ttf::Sdl2TtfContext, video::Window,
};

pub mod button;
pub mod scrollview;
pub mod text;
pub mod list;

pub trait Widget: DynClone {
    fn get_id(&self) -> u32;
    fn draw(&mut self, canvas: &mut RefMut<Canvas<Window>>, ttf_context: &Rc<RefCell<Sdl2TtfContext>>);
    fn check_hover(&mut self, _x: i32, _y: i32) {}
    fn check_click(&self, _x: i32, _y: i32) {}
    fn check_scroll(
        &mut self,
        _x: i32,
        _y: i32,
        _direction: MouseWheelDirection,
        _precise_x: f32,
        _precise_y: f32,
    ) {
    }
    fn set_rect(&mut self, _rect: Rect);
    fn get_rect(&self) -> Rect;
    fn get_height(&self) -> u32 {0}
}

dyn_clone::clone_trait_object!(Widget);
