use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use sdl2::{pixels::Color, rect::Rect, render::Canvas, ttf::Sdl2TtfContext, video::Window};

use super::{text::Text, Widget};

#[derive(Clone)]
pub struct List {
    id: u32,
    widgets: Vec<Box<dyn Widget>>,
    rect: Rect,
    changed: bool,
    need_update: bool,
    viewport: Rect,
}

impl List {
    pub fn new(window_id: u32, x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            id: window_id,
            widgets: vec![],
            rect: Rect::new(x, y, width, height),
            changed: false,
            need_update: false,
            viewport: Rect::new(x, y, width, height),
        }
    }

    pub fn add_widget(mut self, widget: Box<dyn Widget>) -> Self {
        self.widgets.push(widget);
        self.changed = true;
        self
    }

    pub fn add_text(self, text: &str) -> Self {
        self.add_widget(Box::new(Text::new_to_zero(text)))
    }
}

impl Widget for List {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn draw(&mut self, canvas: &mut RefMut<Canvas<Window>>) {
        if self.changed {
            let mut y_offset = 0;
            for widget in self.widgets.iter_mut() {
                // widget.init_ttf_context(ttf_context);
                let mut w_rect = widget.get_rect();
                w_rect.set_x(self.rect.x());
                w_rect.set_y(self.rect.y() + y_offset);
                widget.set_rect(w_rect);
                y_offset += w_rect.height() as i32;
            }
            self.rect.set_height(y_offset as u32);
            self.viewport = self.rect;
            self.changed = false;
            self.need_update = true;
        }
        canvas.set_draw_color(Color::WHITE);
        let _ = canvas.fill_rect(self.rect);
        for widget in self.widgets.iter_mut() {
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
