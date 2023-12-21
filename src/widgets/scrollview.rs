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
    scrolling: bool,
    scroll_sensitivity: i32,
    scroll: f32,
    scroll_acceleration: f32,
    v_ratio: f32,
    scroll_thumb_rect: Rect,
    scroll_area_rect: Rect,
    scroll_area_width: u32,
    drag_thumb: bool,
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
            scrolling: false,
            scroll_sensitivity: 40,
            scroll: 0.,
            scroll_acceleration: 0.,
            v_ratio: 1.,
            scroll_thumb_rect: Rect::new(0, 0, 0, 0),
            scroll_area_rect: Rect::new(0, 0, 0, 0),
            scroll_area_width: 8,
            drag_thumb: false,
        };
        obj.update();
        obj
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

    fn draw(&mut self, canvas: &mut std::cell::RefMut<sdl2::render::Canvas<sdl2::video::Window>>) {
        canvas.set_draw_color(Color::WHITE);
        if let Err(e) = canvas.fill_rect(self.rect) {
            print!("{}", e)
        }

        if self.scrolling {
            // add specific scroll friction
            if cfg!(target_os = "macos") {
                self.scroll_acceleration *= 0.85;
            } else {
                self.scroll_acceleration *= 0.4;
            }

            if self.scroll_acceleration.abs() < 0.0005 {
                self.scroll_acceleration = 0.;
                self.scrolling = false;
            }
            self.scroll -= self.scroll_sensitivity as f32 * self.scroll_acceleration;
            // Here you have to set your scrolling bounds i.e. if(scroll_Y < 0) scroll_Y = 0;
            if self.scroll < 0. {
                // if at the top clamp
                self.scroll = 0.;
                self.scroll_acceleration = 0.;
            } else if self.scroll + self.rect.height() as f32
                >= self.rect.height() as f32 / self.v_ratio
            // at the bottom clamp too
            {
                self.scroll =
                    (self.rect.height() as f32 / self.v_ratio) - self.rect.height() as f32;
                self.scroll_acceleration = 0.;
            }

            let mut w_rect = self.widget.get_rect();
            w_rect.set_y(self.rect.y - self.scroll as i32); // apply scroll to the widget
            self.widget.set_rect(w_rect);
        }

        canvas.set_clip_rect(self.rect);
        self.widget.draw(canvas); // draw widget
        canvas.set_clip_rect(None);

        if self.v_ratio < 1. {
            // if inner content greater then the visual height add a scrollbar
            canvas.set_draw_color(Color::GRAY);
            self.scroll_area_rect = Rect::new(
                self.rect.x() + self.rect.width() as i32 - self.scroll_area_width as i32,
                self.rect.y(),
                self.scroll_area_width,
                self.rect.height(),
            );
            let _ = canvas.fill_rect(self.scroll_area_rect);
            canvas.set_draw_color(Color::RED);
            self.scroll_thumb_rect = Rect::new(
                self.rect.x() + self.rect.width() as i32 - self.scroll_area_width as i32,
                self.rect.y() + (self.scroll * self.v_ratio) as i32,
                self.scroll_area_width,
                (self.rect.height() as f32 * self.v_ratio) as u32,
            );
            let _ = canvas.fill_rect(self.scroll_thumb_rect);
        }
    }

    fn event(&mut self, event: sdl2::event::Event) {
        match event {
            sdl2::event::Event::MouseMotion {
                window_id, x, y, ..
            } => {
                if window_id == self.id {
                    let mouse = Point::new(x, y);
                    self.hover = self.rect.contains_point(mouse);
                    if self.v_ratio < 1. && self.drag_thumb {
                        self.scroll = y as f32 / self.v_ratio;
                        self.scrolling = true;
                    }
                    if self.v_ratio < 1. && self.scroll_area_rect.contains_point(mouse)
                        || self.drag_thumb
                    {
                        self.scroll_area_width = 10;
                    } else {
                        self.scroll_area_width = 8;
                    }
                    if self.hover {
                        self.widget.event(event);
                    }
                }
            }
            sdl2::event::Event::MouseButtonDown {
                window_id, x, y, ..
            } => {
                if window_id == self.id {
                    self.widget.event(event);
                    if self.hover && self.scroll_thumb_rect.contains_point(Point::new(x, y)) {
                        self.drag_thumb = true;
                    }
                }
            }
            sdl2::event::Event::MouseButtonUp { .. } => {
                if self.drag_thumb {
                    self.drag_thumb = false;
                }
            }
            sdl2::event::Event::MouseWheel {
                window_id,
                precise_y,
                ..
            } => {
                if self.hover && window_id == self.id {
                    self.scroll_acceleration += precise_y;
                    self.scrolling = true;
                }
            }
            _ => {}
        }
    }

    fn init_ttf_context(
        &mut self,
        ttf_context: &std::rc::Rc<std::cell::RefCell<sdl2::ttf::Sdl2TtfContext>>,
    ) {
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
