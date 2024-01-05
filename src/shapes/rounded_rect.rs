use sdl2::{render::Canvas, video::Window, pixels::Color, rect::Rect,};

use crate::{widgets::circle::Circle, utils::polygon::Polygon};


pub struct RoundedRect {
    rect: Rect,
    radius: u32,
    polygon: Option<Polygon>,
}

impl RoundedRect {
    pub fn new(x: i32, y: i32, width: u32, height: u32, radius: u32) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            radius,
            polygon: None,
        }
    }

    pub fn from_rect(rect: Rect, radius: u32) -> Self {
        Self {
            rect,
            radius,
            polygon: None,
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>, color: Color) {
        if self.polygon.is_none() {
            let circle = Circle::new(self.rect.x(), self.rect.y(), self.radius as i32, color);
            let mut q = circle.get_quarters();
            let x_offset = self.rect.width() as i32 - self.radius as i32 * 2;
            let y_offset = self.rect.height() as i32 - self.radius as i32 * 2;
            q.1.iter_mut().for_each(|p| p.x += x_offset);
            q.2.iter_mut().for_each(|p| {p.y += y_offset; p.x += x_offset;});
            q.3.iter_mut().for_each(|p| p.y += y_offset);
            
            // merge quarters
            let mut pxls = vec![];
            pxls.append(&mut q.0);
            pxls.append(&mut q.1);
            pxls.append(&mut q.2);
            pxls.append(&mut q.3);

            self.polygon = Some(pxls.into());
        }
        let pol = self.polygon.as_ref().unwrap().clone();
        pol.draw(canvas, color, true);
    }
}