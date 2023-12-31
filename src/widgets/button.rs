use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window, gfx::primitives::DrawRenderer,
};

use crate::{window::MyWindow, Action, utils::Style};

use super::{text::Text, Widget};

#[derive(Clone)]
pub struct Button<'a> {
    rect: Rect,
    hover: bool,
    label: Text<'a>,
    on_click: Rc<RefCell<Box<dyn Fn() -> Action>>>,
    style: Style,
}

impl<'a> Button<'a> {
    pub fn new<F: 'static + Fn() -> Action>(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        text: &str,
        on_click: F,
        style: Style,
    ) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            hover: false,
            label: Text::new(x, y, text),
            on_click: Rc::new(RefCell::new(Box::new(on_click))),
            style,
        }
    }
}

impl<'a> Widget for Button<'a> {

    fn draw(&mut self, canvas: &mut RefMut<Canvas<Window>>) {
        let color = if !self.hover {
            self.style.background_color
        } else {
            self.style.hover_background_color
        };
        canvas.set_draw_color(color);
        if self.style.border_radius != 0 {
            canvas
                .rounded_box(
                    self.rect.x() as i16,
                    self.rect.y() as i16,
                    self.rect.x() as i16 + self.rect.width() as i16,
                    self.rect.y() as i16 + self.rect.height() as i16,
                    self.style.border_radius as i16,
                    color,
                )
                .unwrap();
            canvas
                .fill_rect(self.rect)
                .expect("Could not draw rect");
        } else {
            canvas
                .fill_rect(self.rect)
                .expect("Could not draw rect");
        }
        self.label.draw(canvas);
    }

    fn event(&mut self, event: sdl2::event::Event, win: &MyWindow) -> Action {
        match event {
            sdl2::event::Event::MouseMotion {
                window_id, x, y, ..
            } => {
                if window_id == win.get_id() {
                    self.hover = self.rect.contains_point(Point::new(x, y-1));
                }
            }
            sdl2::event::Event::MouseButtonDown { window_id, .. } => {
                if self.hover && window_id == win.get_id() {
                    println!("{}", win.get_id());
                    return (self.on_click.borrow())();
                }
            }
            _ => {}
        }
        Action::None
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
        self.label.set_rect(rect);
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }
}
