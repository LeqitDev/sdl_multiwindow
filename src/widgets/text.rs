use std::{
    cell::{RefCell, RefMut},
    path::{Path, PathBuf},
    rc::Rc, env,
};

use lazy_static::lazy_static;
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureQuery},
    ttf::Sdl2TtfContext,
    video::Window, surface::Surface,
};

use super::Widget;

lazy_static! {
    static ref DEFAULT_FONT_PATH: PathBuf = {
        let mut font_path = env::current_dir().unwrap();
        font_path.push("assets");
        font_path.push("OpenSans-Bold.ttf");
        font_path
    };
}

add_new_to_main_with_lifetime!(Text, x: i32, y: i32, text: &str);
add_new_to_zero_with_lifetime!(Text, text: &str);

#[derive(Clone)]
pub struct Text<'a> {
    id: u32,
    text: String,
    rect: Rect,
    texture: Option<Rc<RefCell<Surface<'a>>>>,
    ttf_context: Option<Rc<RefCell<Sdl2TtfContext>>>,
}

impl<'a> Text<'a> {
    pub fn new(id: u32, x: i32, y: i32, text: &str) -> Self {
        Self {
            id,
            text: text.to_string(),
            rect: Rect::new(x, y, 0, 0),
            texture: None,
            ttf_context: None,
        }
    }

    fn update_texture(&mut self) {
        if let Some(ttf_context) = &self.ttf_context {
            let ttf_ctx = ttf_context.borrow_mut();
            let mut font = ttf_ctx
                .load_font(Path::new(&DEFAULT_FONT_PATH.as_os_str()), 16)
                .unwrap();
            font.set_style(sdl2::ttf::FontStyle::BOLD);
            let surface = font.render(&self.text).blended(Color::BLACK).unwrap();
            self.rect.set_width(surface.rect().width());
            self.rect.set_height(surface.rect().height());
            self.texture = Some(Rc::new(RefCell::new(surface)));
        }
    }
}

impl<'a> Widget for Text<'a> {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn init_ttf_context(&mut self, ttf_context: &Rc<RefCell<Sdl2TtfContext>>) {
        self.ttf_context = Some(ttf_context.clone());
        self.update_texture();
    }

    fn draw(&mut self, canvas: &mut RefMut<Canvas<Window>>) {
        if self.texture.is_some() {
            let texture_creator = canvas.texture_creator();
            let texture = texture_creator
                .create_texture_from_surface(&*(self.texture.clone().unwrap().borrow()))
                .unwrap();
            let TextureQuery { width, height, .. } = texture.query();
            let _ratio = width as f32 / height as f32;
            let _ = canvas.copy(
                &texture,
                None,
                Some(self.rect),
            );
        }
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }
}
