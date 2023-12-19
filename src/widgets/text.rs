use std::{
    cell::{RefCell, RefMut},
    path::{Path, PathBuf},
    rc::Rc,
};

use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureQuery},
    ttf::Sdl2TtfContext,
    video::Window, surface::Surface,
};

use super::Widget;

#[derive(Clone)]
pub struct Text<'a> {
    id: u32,
    font_path: PathBuf,
    text: String,
    rect: Rect,
    height: u32,
    texture: Option<Rc<RefCell<Surface<'a>>>>,
}

impl<'a> Text<'a> {
    pub fn new(id: u32, font_path: &Path, text: String, x: i32, y: i32) -> Self {
        Self {
            id,
            font_path: font_path.to_path_buf(),
            text,
            rect: Rect::new(x, y, 0, 0),
            texture: None,
            height: 0,
        }
    }
}

impl<'a> Widget for Text<'a> {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn draw(&mut self, canvas: &mut RefMut<Canvas<Window>>, ttf_context: &Rc<RefCell<Sdl2TtfContext>>) {
        if self.texture.is_none() {
            let ttf_ctx = ttf_context.borrow_mut();
            let mut font = ttf_ctx
                .load_font(Path::new(&self.font_path.as_os_str()), 16)
                .unwrap();
            font.set_style(sdl2::ttf::FontStyle::BOLD);
            let surface = font.render(&self.text).blended(Color::BLACK).unwrap();
            self.texture = Some(Rc::new(RefCell::new(surface)));
        }
        if self.texture.is_some() {
            let texture_creator = canvas.texture_creator();
            let texture = texture_creator
                .create_texture_from_surface(&*(self.texture.clone().unwrap().borrow()))
                .unwrap();
            let TextureQuery { width, height, .. } = texture.query();
            let _ratio = width as f32 / height as f32;
            self.rect.set_width(width);
            self.rect.set_height(height);
            self.height = height;
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
        println!("{}", self.rect.height());
        self.rect
    }

    fn get_height(&self) -> u32 {
        self.height
    }
}
