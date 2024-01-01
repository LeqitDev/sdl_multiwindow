use std::cell::RefMut;

use sdl2::{pixels::Color, rect::Point, render::Canvas};

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
        let mut dd_f_x = 0;
        let mut dd_f_y = -2 * radius;
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
                dd_f_y += 2;
                f += dd_f_y;
            }
            x += 1;
            dd_f_x += 2;
            f += dd_f_x + 1;

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

    pub fn get_quarters(&self) -> (Vec<Point>, Vec<Point>, Vec<Point>, Vec<Point>) {
        let pxls = self.get_circle();
        let chunks = pxls.chunks(pxls.len() / 4).map(|x| x.to_vec()).collect::<Vec<Vec<Point>>>();
        (chunks[0].clone(), chunks[1].clone(), chunks[2].clone(), chunks[3].clone())
    }
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