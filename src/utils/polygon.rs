use sdl2::{rect::Point, surface::Surface, gfx::primitives::DrawRenderer};



#[derive(Clone)]
pub struct Polygon {
    vx: Vec<i16>,
    vy: Vec<i16>,
    extreme_x: (i16, i16),
    extreme_y: (i16, i16),
    center: Point,
}

impl Polygon {
    // create a polygon with the points
    pub fn new(vx: Vec<i16>, vy: Vec<i16>) -> Self {
        assert!(vx.len() == vy.len(), "vx and vy must have the same length");
        let details = Self::get_details(vx.clone(), vy.clone());
        Self {
            vx,
            vy,
            extreme_x: details.1,
            extreme_y: details.2,
            center: details.0,
        }
    }

    // create a polygon with already provided details
    pub fn detailed(vx: Vec<i16>, vy: Vec<i16>, extreme_x: (i16, i16), extreme_y: (i16, i16), center: Point) -> Self {
        assert!(vx.len() == vy.len(), "vx and vy must have the same length");
        Self {
            vx,
            vy,
            extreme_x,
            extreme_y,
            center,
        }
    }

    // draw the polygon
    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, color: sdl2::pixels::Color, filled: bool) {
        if filled {
            canvas.filled_polygon(self.vx.as_slice(), self.vy.as_slice(), color).unwrap();
        } else {
            canvas.aa_polygon(self.vx.as_slice(), self.vy.as_slice(), color).unwrap();
        }
    }

    // shrinking the polygon
    pub fn shrink(&mut self, amount: i16) {
        for i in 0..self.vx.len() {
            if self.vx[i] == self.extreme_x.0 || self.vx[i] == self.extreme_x.1 {
                self.vx[i] = if self.vx[i] > self.center.x as i16 { self.vx[i] - amount } else { self.vx[i] + amount };
            } else {
                self.vy[i] = if self.vy[i] > self.center.y as i16 { self.vy[i] - amount } else { self.vy[i] + amount };
            }
        }
    }

    // Calculates details about the polygon through the points
    pub fn get_details(vx: Vec<i16>, vy: Vec<i16>) -> (Point, (i16, i16), (i16, i16)) {
        let mut extreme_x = (vx[0], vx[0]);
        let mut extreme_y = (vy[0], vy[0]);
        let mut center = Point::new(0, 0);
        for i in 0..vx.len() {
            if vx[i] < extreme_x.0 {
                extreme_x.0 = vx[i];
            }
            if vx[i] > extreme_x.1 {
                extreme_x.1 = vx[i];
            }
            if vy[i] < extreme_y.0 {
                extreme_y.0 = vy[i];
            }
            if vy[i] > extreme_y.1 {
                extreme_y.1 = vy[i];
            }
        }
        center.x = (extreme_x.1 - extreme_x.0) as i32;
        center.y = (extreme_y.1 - extreme_y.0) as i32;
        (center, extreme_x, extreme_y)
    }

    // For anti-aliasing but not finished
    pub fn get_taxture(&self, color: sdl2::pixels::Color, filled: bool) -> Surface<'_> {
        let mut surface = Surface::new(self.extreme_x.1 as u32 - self.extreme_x.0 as u32, self.extreme_y.1 as u32 - self.extreme_y.0 as u32, sdl2::pixels::PixelFormatEnum::RGBA8888).unwrap();

        let canvas = surface.into_canvas().unwrap();
        if filled {
            canvas.filled_polygon(self.vx.as_slice(), self.vy.as_slice(), color).unwrap();
        } else {
            canvas.aa_polygon(self.vx.as_slice(), self.vy.as_slice(), color).unwrap();
        }

        canvas.into_surface()
    }
}

impl From<Vec<Point>> for Polygon {
    fn from(points: Vec<Point>) -> Self {
        let mut vx = vec![];
        let mut vy = vec![];
        if points.len() > 0 {
            let mut extreme_x = (points[0].x() as i16, points[0].x() as i16);
            let mut extreme_y = (points[0].y() as i16, points[0].y() as i16);

            // get extreme points
            for point in points {
                vx.push(point.x() as i16);
                vy.push(point.y() as i16);

                if point.x() < extreme_x.0 as i32 {
                    extreme_x.0 = point.x() as i16;
                }
                if point.x() > extreme_x.1 as i32 {
                    extreme_x.1 = point.x() as i16;
                }
                if point.y() < extreme_y.0 as i32 {
                    extreme_y.0 = point.y() as i16;
                }
                if point.y() > extreme_y.1 as i32 {
                    extreme_y.1 = point.y() as i16;
                }
            }

            let center = Point::new((extreme_x.1 - extreme_x.0) as i32, (extreme_y.1 - extreme_y.0) as i32);
            return Self {
                vx,
                vy,
                extreme_x,
                extreme_y,
                center,
            }
        }
        Self {
            vx,
            vy,
            extreme_x: (0, 0),
            extreme_y: (0, 0),
            center: Point::new(0, 0),
        }
    }
}