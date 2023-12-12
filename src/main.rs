extern crate sdl2;

use sdl2::VideoSubsystem;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window1 = video_subsystem.window("Window 1", 640, 480)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let window2 = add_window(&video_subsystem).unwrap();
    let window3 = add_window(&video_subsystem).unwrap();

    let id1 = window1.id();
    let mut canvas1 = window1.into_canvas().build().map_err(|e| e.to_string())?;
    let id2 = window2.id();
    let mut canvas2 = window2.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut windows: Vec<(&Canvas<Window>, u32, bool)> = vec![(&canvas1, id1, true), (&canvas2, id2, true)];

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Window { win_event: e, window_id: id, .. } => {
                    match e {
                        sdl2::event::WindowEvent::Close => {
                            windows = windows.iter().filter(|(c, w_id, v)| id != *w_id).collect::<Vec<(&Canvas<Window>, u32, bool)>>();
                        }
                        _ => {}
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Escape), window_id: id, .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas1.set_draw_color(Color::RGB(255, 0, 0));
        canvas1.clear();
        canvas1.present();

        // canvas2.set_draw_color(Color::RGB(0, 255, 0));
        // canvas2.clear();
        // canvas2.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}


fn add_window(video_subsystem: &VideoSubsystem) -> Result<Window, String>{
    let window = video_subsystem.window("Window 2", 640, 480)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    return Ok(window);
}