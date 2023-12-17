extern crate sdl2;

use dyn_clone::DynClone;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureQuery};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;
use sdl2::VideoSubsystem;
use std::cell::{RefCell, RefMut};
use std::env;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::{Duration, SystemTime};

type CanvasCell = Rc<RefCell<Canvas<Window>>>;
type DrawFn = Box<dyn FnMut(CanvasCell, Vec<Box<dyn Widget>>, Rc<RefCell<Sdl2TtfContext>>)>;

struct MyWindow {
    update: DrawFn,
    id: u32,
    active: bool,
    canvas: CanvasCell,
}

impl MyWindow {
    fn new<F: 'static + FnMut(CanvasCell, Vec<Box<dyn Widget>>, Rc<RefCell<Sdl2TtfContext>>)>(
        update: F,
        id: u32,
        canvas: CanvasCell,
        active: bool,
    ) -> Self {
        Self {
            update: Box::new(update),
            id,
            active,
            canvas,
        }
    }

    fn update(&mut self, widgets: Vec<Box<dyn Widget>>, ttf_context: Rc<RefCell<Sdl2TtfContext>>) {
        (self.update)(self.canvas.clone(), widgets, ttf_context);
    }

    fn create<F: 'static + FnMut(CanvasCell, Vec<Box<dyn Widget>>, Rc<RefCell<Sdl2TtfContext>>)>(
        video_subsystem: &VideoSubsystem,
        title: &str,
        width: u32,
        height: u32,
        update: F,
    ) -> Self {
        let canvas = to_canvas(add_window(video_subsystem, title, width, height).unwrap()).unwrap();
        MyWindow::new(update, canvas.id, canvas.canvas, true)
    }
}

/* struct WindowSettings{
    video_subsystem: Rc<RefCell<VideoSubsystem>>,
    title: String,
        width: u32,
        height: u32,
        update: Box<dyn FnMut(CanvasCell)>,
}

impl WindowSettings {
    fn new<F: 'static + FnMut(CanvasCell)>(video_subsystem: Rc<RefCell<VideoSubsystem>>, title: &str, width: u32, height: u32, update: F) -> Self {
        Self { video_subsystem, title: title.to_string(), width, height, update: Box::new(update) }
    }
}

impl Into<MyWindow> for WindowSettings {
    fn into(self) -> MyWindow {
        MyWindow::create(&self.video_subsystem.borrow(), self.title.as_str(), self.width, self.height, self.update)
    }
} */

struct MyCanvas {
    id: u32,
    canvas: CanvasCell,
}

impl MyCanvas {
    fn new(id: u32, canvas: CanvasCell) -> Self {
        Self { id, canvas }
    }
}

#[derive(Clone)]
struct Button {
    id: u32,
    rect: Rect,
    hover: bool,
    on_click: Rc<RefCell<Box<dyn Fn()>>>,
}

impl Button {
    fn new<F: 'static + Fn()>(id: u32, x: i32, y: i32, width: u32, height: u32, on_click: F) -> Self {
        Self {
            id,
            rect: Rect::new(x, y, width, height),
            hover: false,
            on_click: Rc::new(RefCell::new(Box::new(on_click))),
        }
    }
}

impl Widget for Button {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn draw(&self, canvas: &mut RefMut<Canvas<Window>>, ttf_context: Rc<RefCell<Sdl2TtfContext>>) {
        canvas.set_draw_color(if !self.hover {
            Color::GREEN
        } else {
            Color::RED
        });
        if let Err(e) = canvas.fill_rect(self.rect) {
            print!("{}", e)
        }
    }

    fn check_hover(&mut self, x: i32, y: i32) {
        self.hover = self.rect.contains_point(Point::new(x, y));
    }

    fn check_click(&self, x: i32, y: i32) {
        let c = self.on_click.clone();
        c.borrow()();
    }
}

#[derive(Clone)]
struct Text {
    id: u32,
    font_path: PathBuf,
    text: String,
    x: i32,
    y: i32,
}

impl Text {
    fn new(id: u32, font_path: PathBuf, text: String, x: i32, y: i32) -> Self {
        Self {
            id,
            font_path,
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

    fn draw(&self, canvas: &mut RefMut<Canvas<Window>>, ttf_context: Rc<RefCell<Sdl2TtfContext>>) {
        let ttf_ctx = ttf_context.borrow_mut();
        let mut font = ttf_ctx
            .load_font(Path::new(&self.font_path.as_os_str()), 16)
            .unwrap();
        font.set_style(sdl2::ttf::FontStyle::BOLD);
        let surface = font.render("Hello Rust!").blended(Color::BLACK).unwrap();
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        let ratio = width as f32/height as f32;
        let _ = canvas.copy(&texture, None, Some(Rect::new(self.x, self.y, width as u32, height)));
    }

    fn check_hover(&mut self, _x: i32, _y: i32) {}

    fn check_click(&self, x: i32, y: i32) {
    }
}

trait Widget: DynClone {
    fn get_id(&self) -> u32;
    fn draw(&self, canvas: &mut RefMut<Canvas<Window>>, ttf_context: Rc<RefCell<Sdl2TtfContext>>);
    fn check_hover(&mut self, x: i32, y: i32);
    fn check_click(&self, x: i32, y: i32);
}

dyn_clone::clone_trait_object!(Widget);

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = Rc::new(RefCell::new(sdl2::ttf::init().map_err(|e| e.to_string())?));

    let mut event_pump = sdl_context.event_pump()?;

    let mut font_path = env::current_dir().unwrap();
    font_path.push("assets");
    font_path.push("OpenSans-Bold.ttf");

    let main_window = MyWindow::create(
        &video_subsystem,
        "Window 1",
        800,
        600,
        move |canvas, widgets, ttf_context| {
            let mut c = canvas.borrow_mut();
            c.set_draw_color(Color::RGB(255, 0, 0));
            c.clear();

            for widget in widgets {
                widget.draw(&mut c, ttf_context.clone());
            }

            c.present();
        },
    );
    let main_id = main_window.id;

    let windows = Rc::new(RefCell::new(vec![main_window]));
    let widgets: Rc<RefCell<Vec<Box<dyn Widget>>>> = Rc::new(RefCell::new(vec![]));

    let w_c = windows.clone();

    let on_click = move || {
        let mut w_cb = w_c.borrow_mut();
        if w_cb.len() < 2 {
            w_cb.push(MyWindow::create(&video_subsystem, "Second Window", 300, 500, move |c, w, t| {}));
        }
    };
    

    widgets.borrow_mut().append(&mut vec![
        Box::new(Button::new(main_id, 10, 10, 200, 20, on_click)),
        Box::new(Text::new(
            main_id,
            font_path,
            "Hello Rust!".to_string(),
            10,
            10,
        )),
    ]);

    'running: loop {
        let now = SystemTime::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Close,
                    window_id: id,
                    ..
                } => {
                    let mut bw = windows.borrow_mut();
                    if let Some(entry_pos) = bw.iter().position(|w| w.id == id) {
                        let entry = bw.get(entry_pos).unwrap();
                        drop(entry.canvas.to_owned());
                        bw.remove(entry_pos);
                        if bw.is_empty() || entry_pos == 0 {
                            break 'running;
                        }
                    }
                }
                Event::MouseMotion {
                    window_id, x, y, ..
                } => {
                    for widget in widgets.borrow_mut().iter_mut() {
                        if widget.get_id() == window_id {
                            widget.check_hover(x, y);
                        }
                    }
                },
                Event::MouseButtonDown { window_id, mouse_btn: MouseButton::Left, clicks, x, y, ..} => {
                    for widget in widgets.borrow_mut().iter_mut() {
                        if widget.get_id() == window_id {
                            widget.check_click(x, y);
                        }
                    }
                },
                Event::Quit { .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        for window in windows.borrow_mut().iter_mut() {
            if window.active {
                window.update(
                    widgets.borrow()
                        .iter()
                        .filter(|w| w.get_id() == window.id)
                        .cloned()
                        .collect(),
                    ttf_context.clone(),
                );
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        /* match (now.elapsed()) {
            Ok(x) => println!("Elapsed time: {}", x.as_millis()),
            Err(_) => {},
        } */
    }

    Ok(())
}

fn add_window(
    video_subsystem: &VideoSubsystem,
    title: &str,
    width: u32,
    height: u32,
) -> Result<Window, String> {
    let window = video_subsystem
        .window(title, width, height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    Ok(window)
}

fn to_canvas(window: Window) -> Result<MyCanvas, String> {
    let id = window.id();
    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let canvas_cell = Rc::new(RefCell::new(canvas));
    Ok(MyCanvas::new(id, canvas_cell))
}
