use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use dyn_clone::DynClone;
use sdl2::{event::Event, rect::Rect, render::Canvas, ttf::Sdl2TtfContext, video::Window};

pub mod button;
pub mod list;
pub mod scrollview;
pub mod text;

pub trait Widget: DynClone {
    fn get_id(&self) -> u32;
    fn draw(&mut self, canvas: &mut RefMut<Canvas<Window>>);
    fn event(&mut self, _event: Event) {}
    fn set_rect(&mut self, _rect: Rect);
    fn get_rect(&self) -> Rect;
    fn init_ttf_context(&mut self, _ttf_context: &Rc<RefCell<Sdl2TtfContext>>) {}
    fn multi_gesture(&mut self, _y: f32, _num_fingers: u16) {}
    fn finger_down(&mut self) {}
}

dyn_clone::clone_trait_object!(Widget);
