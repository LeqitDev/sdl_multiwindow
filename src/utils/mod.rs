use sdl2::{gfx::primitives::DrawRenderer, rect::{Point, Rect}};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum FontStyle {
    Normal,
    Bold,
}

#[derive(Clone)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

#[derive(Clone)]
pub struct Polygon {
    vx: Vec<i16>,
    vy: Vec<i16>,
    extreme_x: (i16, i16),
    extreme_y: (i16, i16),
    center: Point,
}

impl Polygon {
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

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, color: sdl2::pixels::Color, filled: bool) {
        if filled {
            canvas.filled_polygon(self.vx.as_slice(), self.vy.as_slice(), color).unwrap();
        } else {
            canvas.aa_polygon(self.vx.as_slice(), self.vy.as_slice(), color).unwrap();
        }
    }

    pub fn shrink(&mut self, amount: i16) {
        for i in 0..self.vx.len() {
            if self.vx[i] == self.extreme_x.0 || self.vx[i] == self.extreme_x.1 {
                self.vx[i] = if self.vx[i] > self.center.x as i16 { self.vx[i] - amount } else { self.vx[i] + amount };
            } else {
                self.vy[i] = if self.vy[i] > self.center.y as i16 { self.vy[i] - amount } else { self.vy[i] + amount };
            }
        }
    }

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
}

impl From<Vec<Point>> for Polygon {
    fn from(points: Vec<Point>) -> Self {
        let mut vx = vec![];
        let mut vy = vec![];
        if points.len() > 0 {
            let mut extreme_x = (points[0].x() as i16, points[0].x() as i16);
            let mut extreme_y = (points[0].y() as i16, points[0].y() as i16);

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

macro_rules! style_struct {
    ($($field:ident: $type:ty),* $(,)?) => {
        #[derive(Clone)]
        pub struct Style {
            $(pub $field: $type,)*
        }

        impl Style {
            $(
                pub fn $field(mut self, $field: $type) -> Self {
                    self.$field = $field;
                    self
                }
            )*
        }
    };
}

style_struct! {
    background_color: sdl2::pixels::Color,
    border_color: sdl2::pixels::Color,
    border_width: u32,
    border_radius: u32,
    text_color: sdl2::pixels::Color,
    font_size: u16,
    font_style: FontStyle,
    hover_background_color: sdl2::pixels::Color,
    text_align: TextAlign,
}

impl Style {
    pub fn adjust(mut self, rect: Rect) -> Self {
        self.border_radius = self.border_radius.min((rect.width() / 2).min(rect.height() / 2));
        self.font_size = self.font_size.min(rect.height() as u16);
        self
    }

}

impl Default for Style {
    fn default() -> Self {
        Self {
            background_color: sdl2::pixels::Color::RGB(255, 255, 255),
            border_color: sdl2::pixels::Color::RGB(0, 0, 0),
            border_width: 1,
            border_radius: 0,
            text_color: sdl2::pixels::Color::RGB(0, 0, 0),
            font_size: 16,
            font_style: FontStyle::Normal,
            hover_background_color: sdl2::pixels::Color::RGB(255, 255, 255),
            text_align: TextAlign::Left,
        }
    }
}