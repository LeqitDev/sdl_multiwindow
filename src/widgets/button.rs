use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    ttf::Sdl2TtfContext,
    video::Window,
};

use super::Widget;

#[derive(Clone)]
pub struct Button {
    id: u32,
    rect: Rect,
    hover: bool,
    on_click: Rc<RefCell<Box<dyn Fn()>>>,
}

impl Button {
    pub fn new<F: 'static + Fn()>(
        id: u32,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        on_click: F,
    ) -> Self {
        Self {
            id,
            rect: Rect::new(x, y, width, height),
            hover: false,
            on_click: Rc::new(RefCell::new(Box::new(on_click))),
        }
    }
}

impl Widget for Button {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn draw(
        &self,
        canvas: &mut RefMut<Canvas<Window>>,
        _ttf_context: &Rc<RefCell<Sdl2TtfContext>>,
    ) {
        canvas.set_draw_color(if !self.hover {
            Color::GREEN
        } else {
            Color::RED
        });
        if let Err(e) = canvas.fill_rect(self.rect) {
            print!("{}", e)
        }
    }
    fn check_hover(&mut self, x: i32, y: i32) {
        self.hover = self.rect.contains_point(Point::new(x, y));
    }
    fn check_click(&self, _x: i32, _y: i32) {
        if self.hover {
            (self.on_click.borrow())();
        }
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn get_rect(&self) -> Option<Rect> {
        Some(self.rect)
    }
}
