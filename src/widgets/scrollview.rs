use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
};

use super::Widget;

#[derive(Clone)]
pub struct ScrollView {
    id: u32,
    widget: Box<dyn Widget>,
    rect: Rect,
    hover: bool,
    offset_y: i32,
    v_ratio: f32,
}

impl ScrollView {
    pub fn new(
        window_id: u32,
        widget: Box<dyn Widget>,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Self {
        let mut obj = Self {
            id: window_id,
            widget,
            rect: Rect::new(x, y, width, height),
            hover: false,
            offset_y: 0,
            v_ratio: 1.,
        };
        obj.update();
        obj
    }

    pub fn update(&mut self) {
        if let Some(mut w_rect) = self.widget.get_rect() {
            let t_rect = self.rect;

            w_rect.set_x(t_rect.x() + w_rect.x());
            w_rect.set_y(t_rect.y() + w_rect.y());
            self.widget.set_rect(w_rect);

            let ratio = (t_rect.height() as f32 / w_rect.height() as f32).min(1.);

            self.v_ratio = ratio;
            println!("{}", self.v_ratio);
        }
    }
}

impl Widget for ScrollView {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn draw(
        &self,
        canvas: &mut std::cell::RefMut<sdl2::render::Canvas<sdl2::video::Window>>,
        ttf_context: &std::rc::Rc<std::cell::RefCell<sdl2::ttf::Sdl2TtfContext>>,
    ) {
        canvas.set_draw_color(Color::WHITE);
        if let Err(e) = canvas.fill_rect(self.rect) {
            print!("{}", e)
        }
        canvas.set_clip_rect(self.rect);
        self.widget.draw(canvas, ttf_context);
        canvas.set_clip_rect(None);

        if self.v_ratio < 1. {
            canvas.set_draw_color(Color::GRAY);
            let _ = canvas.fill_rect(Rect::new(
                self.rect.x() + self.rect.width() as i32 - 10,
                self.rect.y(),
                10,
                self.rect.height(),
            ));
            canvas.set_draw_color(Color::RED);
            let _ = canvas.fill_rect(Rect::new(
                self.rect.x() + self.rect.width() as i32 - 10,
                self.rect.y() + (self.offset_y as f32 * self.v_ratio) as i32,
                10,
                (self.rect.height() as f32 * self.v_ratio) as u32,
            ));
        }
    }

    fn check_hover(&mut self, x: i32, y: i32) {
        self.hover = self.rect.contains_point(Point::new(x, y));
        if self.hover {
            self.widget.check_hover(x, y);
        } else {
            self.widget.check_hover(-1, -1);
        }
    }

    fn check_click(&self, x: i32, y: i32) {
        self.widget.check_click(x, y);
    }

    fn check_scroll(
        &mut self,
        _x: i32,
        _y: i32,
        _direction: sdl2::mouse::MouseWheelDirection,
        _precise_x: f32,
        precise_y: f32,
    ) {
        if self.hover {
            self.offset_y -= precise_y as i32 * 2;
            if self.offset_y <= 0 {
                self.offset_y = 0;
            } else if self.offset_y + self.rect.height() as i32
                >= (self.rect.height() as f32 / self.v_ratio) as i32
            {
                self.offset_y =
                    ((self.rect.height() as f32 / self.v_ratio) as u32 - self.rect.height()) as i32
            }
            if let Some(mut w_rect) = self.widget.get_rect() {
                w_rect.set_y(self.rect.y - self.offset_y);
                self.widget.set_rect(w_rect);
            }
        }
    }
}
