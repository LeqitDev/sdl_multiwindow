use std::cell::RefMut;

use sdl2::{pixels::Color, rect::Point, gfx::primitives::DrawRenderer, render::Canvas};

use crate::{utils::Polygon, widgets::Widget};

#[derive(Clone)]
pub struct Circle {
    x: i32,
    y: i32,
    color: Color,
    radius: i32,
    poly: Option<Polygon>,
    filled: bool,
}

impl Circle {
    pub fn new(x: i32, y: i32, radius: i32, color: Color) -> Self {
        Self {
            x: x + radius,
            y: y + radius,
            color,
            radius,
            poly: None,
            filled: false,
        }
    }

    pub fn filled(x: i32, y: i32, radius: i32, color: Color) -> Self {
        Self {
            x,
            y,
            color,
            radius,
            poly: None,
            filled: true,
        }
    }

    fn get_circle(&self) -> Vec<Point> {
        let mut pxls = vec![];
        let radius = self.radius as i32;
        let mut f = 1 - radius;
        let mut ddF_x = 0;
        let mut ddF_y = -2 * radius;
        let mut x = 0;
        let mut y = radius;
        let x0 = self.x;
        let y0 = self.y;
        
        pxls.push(Point::new(x0, y0 + radius));
        pxls.push(Point::new(x0, y0 - radius));
        pxls.push(Point::new(x0 + radius, y0));
        pxls.push(Point::new(x0 - radius, y0));

        while x < y {
            if f >= 0 {
                y -= 1;
                ddF_y += 2;
                f += ddF_y;
            }
            x += 1;
            ddF_x += 2;
            f += ddF_x + 1;

            pxls.push(Point::new(x0 + x, y0 + y));
            pxls.push(Point::new(x0 - x, y0 + y));
            pxls.push(Point::new(x0 + x, y0 - y));
            pxls.push(Point::new(x0 - x, y0 - y));
            pxls.push(Point::new(x0 + y, y0 + x));
            pxls.push(Point::new(x0 - y, y0 + x));
            pxls.push(Point::new(x0 + y, y0 - x));
            pxls.push(Point::new(x0 - y, y0 - x));
        }

        //sort array
        pxls.sort_by(|a, b| {
            let angle_a = (a.y() as f32 - y0 as f32).atan2(a.x() as f32 - x0 as f32);
            let angle_b = (b.y() as f32 - y0 as f32).atan2(b.x() as f32 - x0 as f32);
            angle_a.partial_cmp(&angle_b).unwrap()
        });

        pxls
    }
    
    /* fn populated_circle(&self, x0: i32, y0: i32) -> Vec<Point> {

        fn check_neighbours(x: i32, y: i32, circle: &Vec<Point>) -> (bool, bool, bool, bool) {
            let mut left = false;
            let mut right = false;
            let mut up = false;
            let mut down = false;

            for point in circle {
                if point.x() == x - 1 && point.y() == y {
                    left = true;
                }
                if point.x() == x + 1 && point.y() == y {
                    right = true;
                }
                if point.x() == x && point.y() == y - 1 {
                    up = true;
                }
                if point.x() == x && point.y() == y + 1 {
                    down = true;
                }
            }
            
            (left, right, up, down)
        }

        fn go_on(x: i32, y: i32, mut circle: Vec<Point>) -> Vec<Point> {
            circle.push(Point::new(x, y));
            let (left, right, up, down) = check_neighbours(x, y, &circle);
            if !left {
                circle = go_on(x - 1, y, circle);
            }
            if !right {
                circle = go_on(x + 1, y, circle);
            }
            if !up {
                circle = go_on(x, y - 1, circle);
            }
            if !down {
                circle = go_on(x, y + 1, circle);
            }
            circle
        }

        let pxls = go_on(x0, y0, self.normal_circle(x0, y0));
        pxls
    } */
}

impl Widget for Circle {
    fn draw(&mut self, canvas: &mut RefMut<'_, Canvas<sdl2::video::Window>>) {
        if self.poly.is_none() {
            self.poly = Some(self.get_circle().into());
        }
        self.poly.as_ref().unwrap().draw(canvas, self.color, self.filled);
    }

    fn set_rect(&mut self, _rect: sdl2::rect::Rect) {
        self.x = _rect.x();
        self.y = _rect.y();
        self.poly = None;
    }

    fn get_rect(&self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(self.x, self.y, self.radius as u32, self.radius as u32)
    }
}