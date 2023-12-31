use std::cell::RefMut;

use sdl2::{render::Canvas, video::Window, pixels::Color, rect::{Rect, Point}, gfx::primitives::DrawRenderer};


pub struct RoundedRect {
    rect: Rect,
    radius: u32,
}

impl RoundedRect {
    pub fn new(x: i32, y: i32, width: u32, height: u32, radius: u32) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            radius,
        }
    }

    pub fn draw(&self, canvas: &mut RefMut<Canvas<Window>>, color: Color) {

         // Draw the four corners 
        /* canvas.filled_circle(x as i16 + radius, y as i16 + radius, radius, Color::RGB(255, 255, 255)).unwrap();
        canvas.filled_circle((x + w as i32) as i16 - radius, y as i16 + radius, radius, Color::RGB(255, 255, 255)).unwrap();
        canvas.filled_circle(x as i16 + radius, (y + h as i32) as i16 - radius, radius, Color::RGB(255, 255, 255)).unwrap();
        canvas.filled_circle((x + w as i32) as i16 - radius, (y + h as i32) as i16 - radius, radius, Color::RGB(255, 255, 255)).unwrap(); */

        canvas.rounded_rectangle(self.rect.x() as i16, self.rect.y() as i16, self.rect.x() as i16 + self.rect.width() as i16, self.rect.y() as i16 + self.rect.height() as i16, self.radius as i16, color).unwrap();
        canvas.string(200, 400, "Hallo haha als wenn", color);
        //polygon dummy data
        let vx = &[1,2,3,4,5,15,16,17,18,19,20];
        let vy = &[5,4,3,2,1,0,1,2,3,4,5];
        canvas.filled_polygon(vx, vy, color).unwrap();

        /* let (x, y, w, h) = (self.rect.x(), self.rect.y(), self.rect.width(), self.rect.height());
        let radius = self.radius as i16;
        // Draw the four sides
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(Rect::new(x + radius as i32, y, w as u32 - (2 * radius as u32), h+1)).unwrap();
        canvas.fill_rect(Rect::new(x, y + radius as i32, w+1, h as u32 - (2 * radius as u32))).unwrap(); */
    }
}