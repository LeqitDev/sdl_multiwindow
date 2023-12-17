extern crate sdl2;

use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;
use std::time::{Duration, SystemTime};
use widgets::button::Button;
use widgets::scrollview::ScrollView;
use widgets::text::Text;
use widgets::Widget;
use window::MyWindow;

type CanvasCell = Rc<RefCell<Canvas<Window>>>;
type DrawFn = Box<dyn FnMut(CanvasCell, Vec<Box<dyn Widget>>, Rc<RefCell<Sdl2TtfContext>>)>;

mod widgets;
mod window;

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
            c.set_draw_color(Color::RGB(0, 0, 0));
            c.clear();

            for widget in widgets {
                widget.draw(&mut c, &ttf_context);
            }

            c.present();
        },
    );
    let main_id = main_window.get_id();

    let windows = Rc::new(RefCell::new(vec![main_window]));
    let widgets: Rc<RefCell<Vec<Box<dyn Widget>>>> = Rc::new(RefCell::new(vec![]));

    let w_c = windows.clone();

    let on_click = move || {
        let mut w_cb = w_c.borrow_mut();
        if w_cb.len() < 2 {
            w_cb.push(MyWindow::create(
                &video_subsystem,
                "Second Window",
                300,
                500,
                move |canvas, widgets, ttf_context| {
                    let mut c = canvas.borrow_mut();
                    c.set_draw_color(Color::RGB(0, 0, 0));
                    c.clear();

                    for widget in widgets {
                        widget.draw(&mut c, &ttf_context);
                    }

                    c.present();
                },
            ));
        }
    };

    widgets.borrow_mut().append(&mut vec![
        Box::new(Button::new(main_id, 10, 10, 200, 20, on_click)),
        Box::new(Text::new(
            main_id,
            font_path.as_path(),
            "Hello Rust!".to_string(),
            10,
            10,
        )),
        Box::new(ScrollView::new(
            main_id,
            Box::new(Button::new(main_id, 0, 0, 30, 1000, || println!("Hi"))),
            300,
            0,
            200,
            300,
        )),
    ]);

    'running: loop {
        let _now = SystemTime::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Close,
                    window_id: id,
                    ..
                } => {
                    let mut bw = windows.borrow_mut();
                    if let Some(entry_pos) = bw.iter().position(|w| w.get_id() == id) {
                        let entry = bw.get(entry_pos).unwrap();
                        drop(entry.get_canvas());
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
                }
                Event::MouseButtonDown {
                    window_id,
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    for widget in widgets.borrow_mut().iter_mut() {
                        if widget.get_id() == window_id {
                            widget.check_click(x, y);
                        }
                    }
                }
                Event::MouseWheel {
                    window_id,
                    x,
                    y,
                    direction,
                    precise_x,
                    precise_y,
                    ..
                } => {
                    for widget in widgets.borrow_mut().iter_mut() {
                        if widget.get_id() == window_id {
                            widget.check_scroll(x, y, direction, precise_x, precise_y);
                        }
                    }
                }
                Event::Quit { .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        for window in windows.borrow_mut().iter_mut() {
            if window.is_active() {
                window.update(
                    widgets
                        .borrow()
                        .iter()
                        .filter(|w| w.get_id() == window.get_id())
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
