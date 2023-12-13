extern crate sdl2;

use dyn_clone::DynClone;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::VideoSubsystem;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::time::Duration;

struct MyWindow {
    update: Box<dyn FnMut(CanvasCell, Vec<Box<dyn Widget>>)>,
    id: u32,
    active: bool,
    canvas: CanvasCell,
}

impl MyWindow {
    fn new<F: 'static + FnMut(CanvasCell, Vec<Box<dyn Widget>>)>(
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

    fn update(&mut self, widgets: Vec<Box<dyn Widget>>) {
        (self.update)(self.canvas.clone(), widgets);
    }

    fn create<F: 'static + FnMut(CanvasCell, Vec<Box<dyn Widget>>)>(
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
}

impl Button {
    fn new(id: u32, x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            id,
            rect: Rect::new(x, y, width, height),
            hover: false,
        }
    }
}

impl Widget for Button {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn draw(&self, canvas: &mut RefMut<Canvas<Window>>) {
        canvas.set_draw_color(if !self.hover { Color::GREEN } else { Color::RED });
        if let Err(e) = canvas.fill_rect(self.rect) {
            print!("{}", e)
        }
    }

    fn check_hover(&mut self, x: i32, y: i32) {
        if self.rect.contains_point(Point::new(x, y)) {
            self.hover = true;
        } else {
            self.hover = false;
        }
    }
}

trait Widget: DynClone {
    fn get_id(&self) -> u32;
    fn draw(&self, canvas: &mut RefMut<Canvas<Window>>);
    fn check_hover(&mut self, x: i32, y: i32);
}

dyn_clone::clone_trait_object!(Widget);

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut event_pump = sdl_context.event_pump()?;

    let main_window = MyWindow::create(
        &video_subsystem,
        "Window 1",
        800,
        600,
        move |canvas, widgets| {
            let mut c = canvas.borrow_mut();
            c.set_draw_color(Color::RGB(255, 0, 0));
            c.clear();

            for widget in widgets {
                widget.draw(&mut c);
            }

            c.present();
        },
    );

    let mut widgets: Vec<Box<dyn Widget>> =
        vec![Box::new(Button::new(main_window.id, 10, 10, 200, 20))];

    let mut windows: Vec<MyWindow> = vec![main_window];

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Close,
                    window_id: id,
                    ..
                } => {
                    if let Some(entry_pos) = windows.iter().position(|w| w.id == id) {
                        let entry = windows.get(entry_pos).unwrap();
                        drop(entry.canvas.to_owned());
                        windows.remove(entry_pos);
                        if windows.is_empty() {
                            break 'running;
                        }
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    window_id: id,
                    ..
                } => break 'running,
                Event::MouseMotion {
                    window_id, x, y, ..
                } => {
                    for widget in &mut widgets {
                        widget.check_hover(x, y);
                    }
                }
                _ => {}
            }
        }

        for window in &mut windows {
            if window.active {
                window.update(
                    widgets
                        .iter()
                        .filter(|w| w.get_id() == window.id)
                        .cloned()
                        .collect(),
                );
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
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

type CanvasCell = Rc<RefCell<Canvas<Window>>>;
