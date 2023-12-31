use std::cell::RefMut;

use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use super::{text::Text, Widget};

#[derive(Clone)]
pub struct List {
    widgets: Vec<Box<dyn Widget>>,
    rect: Rect,
    changed: bool,
    need_update: bool,
    viewport: Rect,
}

impl List {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            widgets: vec![],
            rect: Rect::new(x, y, width, height),
            changed: false,
            need_update: false,
            viewport: Rect::new(x, y, width, height),
        }
    }

    pub fn add_widget(mut self, widget: Box<dyn Widget>) -> Self {
        // println!("Added widget {}: {:?}", self.widgets.len(), SystemTime::now());
        self.widgets.push(widget);
        self.changed = true;
        self
    }

    pub fn add_text(self, text: &str) -> Self {
        self.add_widget(Box::new(Text::new_to_zero(text)))
    }
}

impl Widget for List {

    fn draw(&mut self, canvas: &mut RefMut<Canvas<Window>>) {
        if self.changed {
            let mut y_offset = 0;
            for widget in self.widgets.iter_mut() {
                let mut w_rect = widget.get_rect();
                w_rect.set_x(self.rect.x());
                w_rect.set_y(self.rect.y() + y_offset);
                widget.set_rect(w_rect);
                y_offset += w_rect.height() as i32;
            }
            self.rect.set_height(y_offset as u32);
            self.viewport = self.rect;
            if self.viewport.height() > 1500 {
                self.viewport.set_height(1500);
            }
            self.changed = false;
            self.need_update = true;
        }
        canvas.set_draw_color(Color::WHITE);
        let _ = canvas.fill_rect(self.rect);

        for (_i, widget) in self.widgets.iter_mut().enumerate() {
            if self
                .viewport
                .contains_point(widget.get_rect().bottom_left())
                || self.viewport.contains_point(widget.get_rect().top_left())
            {
                widget.draw(canvas);
            }
        }
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
        let mut y_offset = 0;
        for widget in self.widgets.iter_mut() {
            let mut w_rect = widget.get_rect();
            w_rect.set_y(rect.y() + y_offset);
            widget.set_rect(w_rect);
            y_offset += w_rect.height() as i32;
        }
    }

    fn has_changed(&mut self) -> bool {
        if self.need_update {
            self.need_update = false;
            return true;
        }
        false
    }

    fn give_viewport(&mut self, viewport: Rect) {
        self.viewport = viewport;
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }
}
