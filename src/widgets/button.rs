use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use super::{Widget, text::Text};

#[derive(Clone)]
pub struct Button<'a> {
    id: u32,
    rect: Rect,
    hover: bool,
    label: Text<'a>,
    on_click: Rc<RefCell<Box<dyn Fn()>>>,
}
add_new_to_main_with_lifetime!(
    Button,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    text: &str,
    on_click: Box<dyn Fn()>);

impl<'a> Button<'a> {
    pub fn new<F: 'static + Fn()>(
        id: u32,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        text: &str,
        on_click: F,
    ) -> Self {
        Self {
            id,
            rect: Rect::new(x, y, width, height),
            hover: false,
            label: Text::new(id, x, y, text),
            on_click: Rc::new(RefCell::new(Box::new(on_click))),
        }
    }
}

impl<'a> Widget for Button<'a> {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn draw(
        &mut self,
        canvas: &mut RefMut<Canvas<Window>>,
    ) {
        canvas.set_draw_color(if !self.hover {
            Color::GREEN
        } else {
            Color::RED
        });
        if let Err(e) = canvas.fill_rect(self.rect) {
            print!("{}", e)
        }
        self.label.draw(canvas);
    }
    fn check_hover(&mut self, x: i32, y: i32) {
        self.hover = self.rect.contains_point(Point::new(x, y));
    }
    fn check_click(&self, _x: i32, _y: i32) {
        if self.hover {
            (self.on_click.borrow())();
        }
    }

    fn init_ttf_context(&mut self, ttf_context: &Rc<RefCell<sdl2::ttf::Sdl2TtfContext>>) {
        self.label.init_ttf_context(ttf_context);
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
        self.label.set_rect(rect);
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }
}
