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
    scrolling: bool,
    scroll_sensitivity: i32,
    scroll: f32,
    scroll_acceleration: f32,
    scroll_friction: f32,
    scroll_prev_pos: f32,
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
            scrolling: false,
            scroll_sensitivity: 40,
            scroll: 0.,
            scroll_acceleration: 0.,
            scroll_friction: 0.001,
            scroll_prev_pos: 0.,
            v_ratio: 1.,
        };
        obj.update();
        obj
    }

    pub fn update_acc(&mut self, new_acc: f32) {
        self.scroll_acceleration = new_acc;
    }

    pub fn update(&mut self) {
        let mut w_rect = self.widget.get_rect();
        let t_rect = self.rect;

        w_rect.set_x(t_rect.x());
        w_rect.set_y(t_rect.y());
        self.widget.set_rect(w_rect);

        let ratio = (t_rect.height() as f32 / w_rect.height() as f32).min(1.);

        self.v_ratio = ratio;
        println!("{}", self.v_ratio);
    }
}

impl Widget for ScrollView {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn draw(
        &mut self,
        canvas: &mut std::cell::RefMut<sdl2::render::Canvas<sdl2::video::Window>>,
    ) {
        canvas.set_draw_color(Color::WHITE);
        if let Err(e) = canvas.fill_rect(self.rect) {
            print!("{}", e)
        }


        print!("{}, ", self.scroll_acceleration);
        self.update_acc(self.scroll_acceleration * 0.98);

        if self.scrolling {
            println!("{}", self.scroll_acceleration);
            if self.scroll_acceleration.abs() < 0.0005 {self.scroll_acceleration = 0.; self.scrolling = false;}
            self.scroll -= self.scroll_sensitivity as f32 * self.scroll_acceleration;
            // Here you have to set your scrolling bounds i.e. if(scroll_Y < 0) scroll_Y = 0;
            if self.scroll < 0. {
                self.scroll = 0.;
                self.scroll_acceleration = 0.;
            } else if self.scroll + self.rect.height() as f32 >= self.rect.height() as f32 / self.v_ratio {
                self.scroll = (self.rect.height() as f32 / self.v_ratio) - self.rect.height() as f32;
                self.scroll_acceleration = 0.;
            }

            let mut w_rect = self.widget.get_rect();
            w_rect.set_y(self.rect.y - self.scroll as i32);
            self.widget.set_rect(w_rect);
        }

        canvas.set_clip_rect(self.rect);
        self.widget.draw(canvas);
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
            // self.offset_y -= (precise_y * 2.) as i32;
            self.scroll_acceleration += precise_y;
            if self.offset_y <= 0 {
                self.offset_y = 0;
            } else if self.offset_y + self.rect.height() as i32
                >= (self.rect.height() as f32 / self.v_ratio) as i32
            {
                self.offset_y =
                    ((self.rect.height() as f32 / self.v_ratio) as u32 - self.rect.height()) as i32
            }
            self.scrolling = true;
            println!("hi");
            /* let mut w_rect = self.widget.get_rect();
            w_rect.set_y(self.rect.y - self.offset_y);
            self.widget.set_rect(w_rect); */
        }
    }

    fn multi_gesture(&mut self, y: f32, num_fingers: u16) {
        println!("{}, {}", num_fingers, self.hover);
        if num_fingers == 2 && self.hover {
            if !self.scrolling {
                self.scrolling = true;
                self.scroll_prev_pos = y;
            } else {
                let dy = y - self.scroll_prev_pos;
                self.scroll_acceleration = dy * 40.;
                self.scroll_prev_pos = y;
                self.scrolling = true;
            }
        }
    }

    fn finger_down(&mut self) {
        self.scrolling = false;
    }

    fn init_ttf_context(&mut self, ttf_context: &std::rc::Rc<std::cell::RefCell<sdl2::ttf::Sdl2TtfContext>>) {
        self.widget.init_ttf_context(ttf_context);
        self.update();
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }
}
