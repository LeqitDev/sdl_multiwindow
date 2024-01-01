use std::{
    cell::{RefCell, RefMut},
    env,
    path::{Path, PathBuf},
    rc::Rc, collections::HashMap,
};

use lazy_static::lazy_static;
use sdl2::{
    rect::Rect,
    render::{Canvas, TextureQuery},
    surface::Surface,
    video::Window,
};

use crate::{TTF_CONTEXT, utils::{Style, FontStyle}};

use super::Widget;

lazy_static! {
    static ref FONT_PATHS: HashMap<FontStyle, PathBuf> = {
        let mut paths = HashMap::new();
        let mut font_path = env::current_dir().unwrap();
        font_path.push("assets");
        font_path.push("OpenSans-Bold.ttf");
        paths.insert(FontStyle::Bold, font_path.clone());
        font_path.pop();
        font_path.push("OpenSans-Regular.ttf");
        paths.insert(FontStyle::Normal, font_path);
        paths
    };
}

add_new_to_zero_with_lifetime!(Text, text: &str, style: Style);

#[derive(Clone)]
pub struct Text<'a> {
    text: String,
    rect: Rect,
    texture: Option<Rc<RefCell<Surface<'a>>>>,
    style: Style,
}

impl<'a> Text<'a> {
    pub fn new(x: i32, y: i32, text: &str, style: Style) -> Self {
        let mut s = Self {
            text: text.replace('\t', "    "),
            rect: Rect::new(x, y, 0, 0),
            texture: None,
            style,
        };
        s.update_height();
        s
    }

    pub fn clipped(x: i32, y: i32, width: u32, height: u32, text: &str, style: Style) -> Self {
        let mut s = Self {
            text: text.replace('\t', "    "),
            rect: Rect::new(x, y, width, height),
            texture: None,
            style,
        };
        s.update_height();
        s
    }

    fn update_texture(&mut self) {
        let mut font = TTF_CONTEXT
            .load_font(Path::new(&FONT_PATHS.get(&self.style.font_style).unwrap().as_os_str()), self.style.font_size)
            .unwrap();
        font.set_style(sdl2::ttf::FontStyle::BOLD);
        let surface = font.render(&self.text).blended(self.style.text_color).unwrap();
        if self.rect.width() <= 4 {
            self.rect.set_width(surface.rect().width());
        }
        if self.rect.height() <= 4 {
            self.rect.set_height(surface.rect().height());
        }
        match self.style.text_align {
            crate::utils::TextAlign::Center => {
                self.rect.set_x(self.rect.x() - surface.rect().width() as i32 / 2);
            }
            crate::utils::TextAlign::Left => {}
            crate::utils::TextAlign::Right => {
                self.rect.set_x(self.rect.x() - surface.rect().width() as i32);
            }
        }
        self.texture = Some(Rc::new(RefCell::new(surface)));
    }

    fn update_height(&mut self) {
        let mut font = TTF_CONTEXT
            .load_font(Path::new(&FONT_PATHS.get(&self.style.font_style).unwrap().as_os_str()), 16)
            .unwrap();
        font.set_style(sdl2::ttf::FontStyle::BOLD);
        if self.rect.height() <= 4 {
            self.rect.set_height(font.height() as u32);
        }
    }
}

impl<'a> Widget for Text<'a> {

    fn draw(&mut self, canvas: &mut RefMut<Canvas<Window>>) {
        if self.texture.is_some() {
            let texture_creator = canvas.texture_creator();
            let texture = texture_creator
                .create_texture_from_surface(&*(self.texture.clone().unwrap().borrow()))
                .unwrap();
            let TextureQuery { width, height, .. } = texture.query();
            let _ratio = width as f32 / height as f32;
            
            canvas.copy(&texture, None, Some(Rect::new(self.rect.x(), self.rect.y(), width, height))).unwrap();
        } else {
            self.update_texture();
        }
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }
}
