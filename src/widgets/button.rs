use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use sdl2::{
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::{window::MyWindow, Action, utils::style::{Style, StyleValues, TextAlign}, CustomCanvas};

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
        let rect = Rect::new(x, y, width, height);
        let xy = match style.normal.text_align {
            TextAlign::Center => {
                (x + width as i32 / 2, y)
            }
            TextAlign::Left => (x, y),
            TextAlign::Right => {
                (x + width as i32, y)
            }
        };
        Self {
            rect,
            hover: false,
            label: Text::new(xy.0, xy.1, text, style.clone()),
            on_click: Rc::new(RefCell::new(Box::new(on_click))),
            style: style.adjust(rect),
        }
    }

    fn get_style(&self) -> &StyleValues {
        if self.hover {
            &self.style.hover
        } else {
            &self.style.normal
        }
    }
}

impl<'a> Widget for Button<'a> {

    fn draw(&mut self, canvas: &mut RefMut<Canvas<Window>>) {
        canvas.set_draw_color(self.get_style().background_color);
        if self.get_style().border_radius != 0 {
            canvas
                .rounded_rect(self.rect, self.get_style().border_radius);
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
                    self.hover = self.rect.contains_point(Point::new(x, y));
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
