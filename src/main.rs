extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::time::{Duration, SystemTime};
use widgets::button::Button;
use widgets::list::List;
use widgets::scrollview::ScrollView;
use widgets::Widget;
use window::MyWindow;

type CanvasCell = Rc<RefCell<Canvas<Window>>>;
type DrawFn = Box<dyn FnMut(CanvasCell, RefMut<Vec<Box<dyn Widget>>>)>;

macro_rules! add_new_to_main_with_lifetime {
    ($struct_name:ident, $($arg_name:ident : $arg_type:ty),*) => {
        impl<'a> $struct_name<'a> {
            pub fn new_to_main($($arg_name : $arg_type),*) -> Self {
                Self::new(0, $($arg_name),*)
            }
        }
    };
}
macro_rules! add_new_to_main {
    ($struct_name:ident, $($arg_name:ident : $arg_type:ty),*) => {
        impl $struct_name {
            pub fn new_to_main($($arg_name : $arg_type),*) -> Self {
                Self::new(0, $($arg_name),*)
            }
        }
    };
}
macro_rules! add_new_to_zero {
    ($struct_name:ident, $($arg_name:ident : $arg_type:ty),*) => {
        impl $struct_name {
            pub fn new_to_zero($($arg_name : $arg_type),*) -> Self {
                Self::new(0, 0, 0, $($arg_name),*)
            }
        }
    };
}
macro_rules! add_new_to_zero_with_lifetime {
    ($struct_name:ident, $($arg_name:ident : $arg_type:ty),*) => {
        impl<'a> $struct_name<'a> {
            pub fn new_to_zero($($arg_name : $arg_type),*) -> Self {
                Self::new(0, 0, 0, $($arg_name),*)
            }
        }
    };
}

mod widgets;
mod window;

fn main() -> Result<(), String> {
    // unsafe { SDL_SetHint(CString::new("AppleMomentumScrollSupported").unwrap().as_ptr(), CString::new("YES").unwrap().as_ptr()); }

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = Rc::new(RefCell::new(sdl2::ttf::init().map_err(|e| e.to_string())?));

    let mut event_pump = sdl_context.event_pump()?;

    let main_window = MyWindow::create(
        &video_subsystem,
        "Window 1",
        800,
        600,
        move |canvas, mut widgets| {
            let mut c = canvas.borrow_mut();
            c.set_draw_color(Color::RGB(0, 0, 0));
            c.clear();

            for widget in widgets.iter_mut() {
                widget.draw(&mut c);
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
                move |canvas, mut widgets| {
                    let mut c = canvas.borrow_mut();
                    c.set_draw_color(Color::RGB(0, 0, 0));
                    c.clear();

                    for widget in widgets.iter_mut() {
                        widget.draw(&mut c);
                    }

                    c.present();
                },
            ));
        }
    };

    let mut lv = List::new(main_id, 0, 100, 200, 300);

    for i in 0..200 {
        lv = lv.add_text(format!("Text {}", i).as_str());
    }

    widgets.borrow_mut().append(&mut vec![
        Box::new(Button::new(
            main_id,
            10,
            10,
            200,
            20,
            "Hello Rust!",
            Box::new(on_click),
        )),
        Box::new(ScrollView::new(main_id, Box::new(lv), 300, 0, 200, 300)),
    ]);

    for widget in widgets.borrow_mut().iter_mut() {
        widget.init_ttf_context(&ttf_context.clone());
    }

    'running: loop {
        let _now = SystemTime::now();
        for event in event_pump.poll_iter() {
            for widget in widgets.borrow_mut().iter_mut() {
                widget.event(event.clone());
            }
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
                Event::Quit { .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        for window in windows.borrow_mut().iter_mut() {
            if window.is_active() {
                window.update((*widgets).borrow_mut());
            }
        }

        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        /* match (now.elapsed()) {
            Ok(x) => println!("Elapsed time: {}", x.as_millis()),
            Err(_) => {},
        } */
    }

    Ok(())
}
