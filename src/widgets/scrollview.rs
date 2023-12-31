use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
};

use crate::{window::MyWindow, Action};

use super::Widget;

const APPLE_FRICTION: f32 = 0.77;
const NORMAL_FRICTION: f32 = 0.4;

#[derive(Clone)]
pub struct ScrollView {
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
    drag_offset: i32,
    thumb_color: Color,
    area_color: Color,
    thumb_hover_color: Color,
    thumb_hover: bool,
}

impl ScrollView {
    pub fn new(
        widget: Box<dyn Widget>,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Self {
        let mut obj = Self {
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
            drag_offset: 0,
            thumb_color: Color::RGB(//gray
                0x80,
                0x80,
                0x80,
            ),
            thumb_hover_color: Color::RGB(//light gray
                0xA0,
                0xA0,
                0xA0,
            ),
            area_color: Color::RGB(//dark gray
                0x60,
                0x60,
                0x60,
            ),
            thumb_hover: false,
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
        self.widget.give_viewport(Rect::new(self.rect.x(), self.rect.y(), self.rect.width(), self.rect.height() + 100));
    }
}

impl Widget for ScrollView {

    fn draw(&mut self, canvas: &mut std::cell::RefMut<sdl2::render::Canvas<sdl2::video::Window>>) {
        canvas.set_draw_color(Color::WHITE);
        if let Err(e) = canvas.fill_rect(self.rect) {
            print!("{}", e)
        }

        if self.widget.has_changed() {
            self.update();
        }

        if self.scrolling {
            // add specific scroll friction
            if cfg!(target_os = "macos") {
                self.scroll_acceleration *= APPLE_FRICTION;
            } else {
                self.scroll_acceleration *= NORMAL_FRICTION;
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
            canvas.set_draw_color(self.area_color);
            self.scroll_area_rect = Rect::new(
                self.rect.x() + self.rect.width() as i32 - self.scroll_area_width as i32,
                self.rect.y(),
                self.scroll_area_width,
                self.rect.height(),
            );
            let _ = canvas.fill_rect(self.scroll_area_rect);
            canvas.set_draw_color(if self.thumb_hover {
                self.thumb_hover_color
            } else {
                self.thumb_color
            });
            self.scroll_thumb_rect = Rect::new(
                self.rect.x() + self.rect.width() as i32 - self.scroll_area_width as i32,
                self.rect.y() + (self.scroll * self.v_ratio) as i32,
                self.scroll_area_width,
                (self.rect.height() as f32 * self.v_ratio) as u32,
            );
            let _ = canvas.fill_rect(self.scroll_thumb_rect);
        }
    }

    fn event(&mut self, event: sdl2::event::Event, win: &MyWindow) -> Action {
        match event {
            sdl2::event::Event::MouseMotion {
                window_id,
                x,
                y,
                yrel,
                ..
            } => {
                if window_id == win.get_id() {
                    let mouse = Point::new(x, y);
                    self.hover = self.rect.contains_point(mouse);
                    self.thumb_hover = self.scroll_thumb_rect.contains_point(mouse);
                    if self.v_ratio < 1. && self.drag_thumb {
                        self.scroll += yrel as f32 / self.v_ratio;
                        self.scrolling = true;
                        if mouse.y() <= self.rect.y() + self.drag_offset {
                            self.scroll = 0.;
                        } else if mouse.y() >= self.rect.height() as i32 - self.drag_offset {
                            self.scroll = self.rect.height() as f32 / self.v_ratio;
                        }
                    }
                    if self.v_ratio < 1. && self.scroll_area_rect.contains_point(mouse)
                        || self.drag_thumb
                    {
                        self.scroll_area_width = 10;
                    } else {
                        self.scroll_area_width = 8;
                    }
                    if self.hover {
                        self.widget.event(event, win);
                    }
                }
            }
            sdl2::event::Event::MouseButtonDown {
                window_id, x, y, ..
            } => {
                if  window_id == win.get_id() {
                    self.widget.event(event, win);
                    if self.hover && self.scroll_thumb_rect.contains_point(Point::new(x, y)) {
                        self.drag_thumb = true;
                        self.drag_offset = y - self.scroll_thumb_rect.y();
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
                if self.hover && window_id == win.get_id() {
                    self.scroll_acceleration += precise_y;
                    self.scrolling = true;
                }
            }
            _ => {}
        }
        Action::None
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }
}
