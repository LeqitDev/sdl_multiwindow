use sdl2::{gfx::primitives::DrawRenderer, rect::Point};

#[derive(Clone)]
pub struct Polygon {
    vx: Vec<i16>,
    vy: Vec<i16>,
}

impl Polygon {
    pub fn new(vx: Vec<i16>, vy: Vec<i16>) -> Self {
        Self {
            vx,
            vy,
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, color: sdl2::pixels::Color, filled: bool) {
        if filled {
            canvas.filled_polygon(self.vx.as_slice(), self.vy.as_slice(), color).unwrap();
        } else {
            canvas.aa_polygon(self.vx.as_slice(), self.vy.as_slice(), color).unwrap();
        }
    }
}

impl From<Vec<Point>> for Polygon {
    fn from(points: Vec<Point>) -> Self {
        let mut vx = vec![];
        let mut vy = vec![];
        for point in points {
            vx.push(point.x() as i16);
            vy.push(point.y() as i16);
        }
        Self {
            vx,
            vy,
        }
    }
}

#[derive(Clone)]
pub struct Style {
    pub background_color: sdl2::pixels::Color,
    pub border_color: sdl2::pixels::Color,
    pub border_width: u32,
    pub border_radius: u32,
    pub text_color: sdl2::pixels::Color,
    pub font_size: u16,
    pub font_name: String,
    pub hover_background_color: sdl2::pixels::Color,
}

impl Style {
    pub fn bg_color(mut self, color: sdl2::pixels::Color) -> Self {
        self.background_color = color;
        self
    }
    pub fn border_color(mut self, color: sdl2::pixels::Color) -> Self {
        self.border_color = color;
        self
    }
    pub fn border_width(mut self, width: u32) -> Self {
        self.border_width = width;
        self
    }
    pub fn border_radius(mut self, radius: u32) -> Self {
        self.border_radius = radius;
        self
    }
    pub fn text_color(mut self, color: sdl2::pixels::Color) -> Self {
        self.text_color = color;
        self
    }
    pub fn font_size(mut self, size: u16) -> Self {
        self.font_size = size;
        self
    }
    pub fn font_name(mut self, name: String) -> Self {
        self.font_name = name;
        self
    }
    pub fn hover_bg_color(mut self, color: sdl2::pixels::Color) -> Self {
        self.hover_background_color = color;
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
            font_name: "OpenSans".to_string(),
            hover_background_color: sdl2::pixels::Color::RGB(255, 255, 255),
        }
    }
}