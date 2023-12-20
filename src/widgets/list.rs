use std::{cell::{RefCell, RefMut}, rc::Rc};

use sdl2::{rect::Rect, ttf::Sdl2TtfContext, video::Window, render::Canvas, pixels::Color};

use super::{Widget, text::Text};

#[derive(Clone)]
pub struct List {
    id: u32,
    widgets: Vec<Box<dyn Widget>>,
    rect: Rect,
}

impl List {
    pub fn new(window_id: u32, x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { id: window_id, widgets: vec![], rect: Rect::new(x, y, width, height) }
    }

    pub fn add_widget(mut self, widget: Box<dyn Widget>) -> Self {
        self.widgets.push(widget);
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

    fn init_ttf_context(&mut self, ttf_context: &Rc<RefCell<Sdl2TtfContext>>) {
        let mut y_offset = 0;
        for widget in self.widgets.iter_mut() {
            widget.init_ttf_context(ttf_context);
            let mut w_rect = widget.get_rect();
            w_rect.set_x(self.rect.x());
            w_rect.set_y(self.rect.y() + y_offset);
            widget.set_rect(w_rect);
            y_offset += w_rect.height() as i32;
        }
        self.rect.set_height(y_offset as u32);
    }

    fn draw(&mut self, canvas: &mut RefMut<Canvas<Window>>) {
        canvas.set_draw_color(Color::WHITE);
        let _ = canvas.fill_rect(self.rect);
        for widget in self.widgets.iter_mut() {
            widget.draw(canvas);
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

    fn get_rect(&self) -> Rect {
        self.rect
    }
}