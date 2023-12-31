extern crate sdl2;

use lazy_static::lazy_static;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;
use shapes::rounded_rect::RoundedRect;
use utils::style::{FontStyle, TextAlign, Style, Params};
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

macro_rules! add_new_to_zero {
    ($struct_name:ident, $($arg_name:ident : $arg_type:ty),*) => {
        impl $struct_name {
            pub fn new_to_zero($($arg_name : $arg_type),*) -> Self {
                Self::new(0, 0, $($arg_name),*)
            }
        }
    };
}
macro_rules! add_new_to_zero_with_lifetime {
    ($struct_name:ident, $($arg_name:ident : $arg_type:ty),*) => {
        impl<'a> $struct_name<'a> {
            pub fn new_to_zero($($arg_name : $arg_type),*) -> Self {
                Self::new(0, 0, $($arg_name),*)
            }
        }
    };
}

impl CustomCanvas for Canvas<Window> {
    fn rounded_rect(&mut self, rect: Rect, radius: u32) {
        let mut rect = RoundedRect::from_rect(rect, radius);
        rect.draw(self, self.draw_color());
    }
}

trait CustomCanvas {
    fn rounded_rect(&mut self, rect: Rect, radius: u32);
}

pub enum Action {
    CreateWindowIfNotExists((u32, MyWindow)),
    None,
}

mod widgets;
mod window;
mod shapes;
mod utils;

lazy_static! {
    static ref TTF_CONTEXT: Sdl2TtfContext = sdl2::ttf::init().unwrap();
    // static ref SDL_CONTEXT: Sdl = sdl2::init().unwrap();
}

fn main() -> Result<(), String> {
    // set sdl2 hint to add anti-aliasing

    let sdl_context = sdl2::init()?;
    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
    let video_subsystem = sdl_context.video()?;
    // let ttf_context = Rc::new(RefCell::new(sdl2::ttf::init().map_err(|e| e.to_string())?));

    let mut event_pump = sdl_context.event_pump()?;

    let mut main_window = MyWindow::create(
        &video_subsystem,
        "Window 1",
        800,
        600,
        move |canvas, mut widgets| {
            let mut c = canvas.borrow_mut();
            c.set_draw_color(Color::RGB(0, 0, 0));
            c.clear();

            c.set_blend_mode(sdl2::render::BlendMode::Blend);

            for widget in widgets.iter_mut() {
                widget.draw(&mut c);
            }

            c.present();
        },
    );

    let windows = Rc::new(RefCell::new(vec![]));

    let on_click = move || {
        // let mut w_cb = w_c.borrow_mut();
                let mut debug_win = MyWindow::create(
                    &video_subsystem,
                    "Second Window",
                    400,
                    800,
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

                let mut lv = List::new(0, 100, 200, 600);

                for i in 0..4000 {
                    lv = lv.add_text(format!("Text {} \t lol", i).as_str());
                }

                debug_win.add_widget(Box::new(ScrollView::new(
                    Box::new(lv),
                    0,
                    0,
                    400,
                    800,
                )));

                Action::CreateWindowIfNotExists((1, debug_win))
    };

    main_window.add_widget(Box::new(Button::new(
        10,
        10,
        200,
        20,
        "Hello Rust!",
        Box::new(on_click),
        Style::new().background_color(Color::RGB(0, 0, 160)).border_radius(20).font_style(FontStyle::Normal).text_align(TextAlign::Right).text_color(Color::RED).font_size(20),
    )));

    main_window.add_widget(Box::new(Button::new(
        10,
        40,
        200,
        20,
        "Hello Rust!",
        Box::new(|| Action::None),
        Style::new().background_color(Params::Hover(Color::RGB(160, 160, 160))).font_style(FontStyle::Bold).text_align(TextAlign::Center).text_color(Color::BLACK),
    )));

    main_window.add_widget(Box::new(Button::new(
        10,
        70,
        200,
        20,
        "Hello Rust!",
        Box::new(|| Action::None),
        Style::new().background_color(Color::RGB(160, 0, 160)).font_style(FontStyle::Bold).text_align(TextAlign::Left).text_color(Color::BLACK).font_size(10),
    )));

    // main_window.add_widget(Box::new(Circle::new(100, 100, 5, Color::RGB(255, 255, 255))));

    windows.borrow_mut().push(main_window);

    'running: loop {
        let _now = SystemTime::now();
        let mut actions: Vec<Action> = Vec::new();
        for event in event_pump.poll_iter() {
            for window in windows.borrow_mut().iter_mut() {
                if window.is_active() {
                    actions.append(&mut window.event(event.clone()));
                }
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

        for action in actions {
            match action {
                Action::CreateWindowIfNotExists((i, win)) => windows.borrow_mut().insert(i as usize, win),
                Action::None => {}
            }
        }

        for window in windows.borrow_mut().iter_mut() {
            window.update();
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        /* match (now.elapsed()) {
            Ok(x) => println!("Elapsed time: {}", x.as_millis()),
            Err(_) => {},
        } */
    }

    Ok(())
}
