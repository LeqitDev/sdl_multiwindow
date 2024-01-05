use sdl2::rect::Rect;

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

pub enum Params<T> {
    All(T),
    Normal(T),
    Hover(T),
    Clicked(T),
    Multiple(T, T, T),
}

impl<T> From<T> for Params<T> {
    fn from(value: T) -> Self {
        Params::Normal(value)
    }
}

macro_rules! style_struct {
    ($($field:ident: $type:ty),* $(,)?) => {
        #[derive(Clone)]
        pub struct StyleValues {
            $(pub $field: $type,)*
        }

        /* impl StyleValues {
            $(
                pub fn $field(mut self, $field: $type) -> Self {
                    self.$field = $field;
                    self
                }
            )*
        } */

        #[derive(Clone)]
        pub struct Style {
            pub normal: StyleValues,
            pub hover: StyleValues,
            pub clicked: StyleValues,
        }

        impl Style {
            pub fn new() -> Self {
                Self {
                    normal: StyleValues::default(),
                    hover: StyleValues::default(),
                    clicked: StyleValues::default(),
                }
            }

            $(
                pub fn $field<T: Into<Params<$type>>>(mut self, param: T) -> Self {
                    let (normal, hover, clicked) = match param.into() {
                        Params::All(value) => (value.clone(), value.clone(), value),
                        Params::Normal(value) => (value.clone(), self.hover.$field.clone(), self.clicked.$field.clone()),
                        Params::Hover(value) => (self.normal.$field.clone(), value.clone(), self.clicked.$field.clone()),
                        Params::Clicked(value) => (self.normal.$field.clone(), self.hover.$field.clone(), value.clone()),
                        Params::Multiple(normal, hover, clicked) => (normal, hover, clicked),
                    };
                    self.normal.$field = normal;
                    self.hover.$field = hover;
                    self.clicked.$field = clicked;
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
    text_align: TextAlign,
}

impl StyleValues {
    pub fn adjust(mut self, rect: Rect) -> Self {
        self.border_radius = self.border_radius.min((rect.width() / 2).min(rect.height() / 2));
        self.font_size = self.font_size.min(rect.height() as u16);
        self
    }

}

impl Default for StyleValues {
    fn default() -> Self {
        Self {
            background_color: sdl2::pixels::Color::RGB(255, 255, 255),
            border_color: sdl2::pixels::Color::RGB(0, 0, 0),
            border_width: 1,
            border_radius: 0,
            text_color: sdl2::pixels::Color::RGB(0, 0, 0),
            font_size: 16,
            font_style: FontStyle::Normal,
            text_align: TextAlign::Left,
        }
    }
}

impl Style {
    pub fn adjust(mut self, rect: Rect) -> Self {
        self.normal = self.normal.adjust(rect);
        self.hover = self.hover.adjust(rect);
        self.clicked = self.clicked.adjust(rect);
        self
    }
}