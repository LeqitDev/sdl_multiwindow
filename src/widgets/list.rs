use std::{cell::{RefCell, RefMut}, rc::Rc};

use sdl2::{rect::Rect, ttf::Sdl2TtfContext, video::Window, render::Canvas, pixels::Color};

use super::Widget;

#[derive(Clone)]
pub struct List {
    id: u32,
    widgets: Vec<Box<dyn Widget>>,
    y_offset: i32,
    rect: Rect,
}

impl List {
    pub fn new(window_id: u32, x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { id: window_id, widgets: vec![], rect: Rect::new(x, y, width, height), y_offset: 0 }
    }

    pub fn add_widget(mut self, widget: Box<dyn Widget>) -> Self {
        self.widgets.push(widget);
        self
    }

    fn update_widgets(&mut self) {
        for widget in self.widgets.iter_mut() {
            let mut w_rect = widget.get_rect();
            w_rect.set_y(self.rect.y + self.y_offset);
            self.y_offset += w_rect.height() as i32;
            widget.set_rect(w_rect);
        }
    }
}

impl Widget for List {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn draw(&mut self, canvas: &mut RefMut<Canvas<Window>>, ttf_context: &Rc<RefCell<Sdl2TtfContext>>) {
        canvas.set_draw_color(Color::WHITE);
        canvas.fill_rect(self.rect);
        let mut y_offset = 0;
        for widget in self.widgets.iter_mut() {
            let mut w_rect = widget.get_rect();
            let w_height = widget.get_height();
            println!("{}", w_height);
            if w_rect.height() != 0 {
                println!("{}", w_rect.height());
                w_rect.set_y(self.rect.y + y_offset);
                widget.set_rect(w_rect);
                y_offset += w_rect.height() as i32;
            }
            widget.draw(canvas, ttf_context);
        }
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }
}