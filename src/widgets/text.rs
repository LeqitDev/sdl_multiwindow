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
    video::Window,
};

use super::Widget;

#[derive(Clone)]
pub struct Text {
    id: u32,
    font_path: PathBuf,
    text: String,
    x: i32,
    y: i32,
}

impl Text {
    pub fn new(id: u32, font_path: &Path, text: String, x: i32, y: i32) -> Self {
        Self {
            id,
            font_path: font_path.to_path_buf(),
            text,
            x,
            y,
        }
    }
}

impl Widget for Text {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn draw(&self, canvas: &mut RefMut<Canvas<Window>>, ttf_context: &Rc<RefCell<Sdl2TtfContext>>) {
        let ttf_ctx = ttf_context.borrow_mut();
        let mut font = ttf_ctx
            .load_font(Path::new(&self.font_path.as_os_str()), 16)
            .unwrap();
        font.set_style(sdl2::ttf::FontStyle::BOLD);
        let surface = font.render(&self.text).blended(Color::BLACK).unwrap();
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        let _ratio = width as f32 / height as f32;
        let _ = canvas.copy(
            &texture,
            None,
            Some(Rect::new(self.x, self.y, width, height)),
        );
    }
}
