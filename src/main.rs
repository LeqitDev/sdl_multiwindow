extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::VideoSubsystem;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

struct MyWindow {
    update: Box<dyn FnMut()>,
    id: u32,
    active: bool,
    canvas: CanvasCell,
}

impl MyWindow {
    fn new<F: 'static + FnMut()>(update: F, id: u32, canvas: CanvasCell, active: bool) -> Self {
        Self {
            update: Box::new(update),
            id,
            active,
            canvas,
        }
    }

    fn default<F: 'static + FnMut()>(update: F, id: u32, canvas: CanvasCell) -> Self {
        Self::new(update, id, canvas, false)
    }
}

struct MyCanvas {
    id: u32,
    canvas: CanvasCell,
    ccanvas: CanvasCell,
}

impl MyCanvas {
    fn new(id: u32, canvas: CanvasCell, ccanvas: CanvasCell) -> Self {
        Self {
            id,
            canvas,
            ccanvas,
        }
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut event_pump = sdl_context.event_pump()?;

    let c1 = to_canvas(add_window(&video_subsystem, "Window 1", 800, 600).unwrap()).unwrap();
    let c2 = to_canvas(add_window(&video_subsystem, "Window 2", 300, 500).unwrap()).unwrap();

    let mut windows: Vec<MyWindow> = vec![
        MyWindow::default(
            move || {
                let mut canvas = c1.canvas.borrow_mut();
                canvas.set_draw_color(Color::RGB(255, 0, 0));
                canvas.clear();
                canvas.present();
            },
            c1.id,
            c1.ccanvas,
        ),
        MyWindow::default(
            move || {
                let mut canvas = c2.canvas.borrow_mut();
                canvas.clear();
                canvas.present();
            },
            c2.id,
            c2.ccanvas,
        ),
    ];

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
                        drop(entry.canvas.borrow());
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
                } => {}
                _ => {}
            }
        }

        for window in &mut windows {
            if window.active {
                (window.update)();
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
    Ok(MyCanvas::new(id, canvas_cell.clone(), canvas_cell))
}

type CanvasCell = Rc<RefCell<Canvas<Window>>>;
