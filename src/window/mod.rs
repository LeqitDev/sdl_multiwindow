use std::{
    borrow::BorrowMut,
    cell::{RefCell, RefMut},
    rc::Rc,
};

use sdl2::{event::Event, render::Canvas, video::Window, VideoSubsystem};

use crate::{widgets::Widget, CanvasCell, DrawFn};

pub struct MyWindow {
    update: DrawFn,
    id: u32,
    active: bool,
    canvas: CanvasCell,
    widgets: Rc<RefCell<Vec<Box<dyn Widget>>>>,
}

impl MyWindow {
    pub fn new<F: 'static + FnMut(CanvasCell, RefMut<Vec<Box<dyn Widget>>>)>(
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
            widgets: Rc::new(RefCell::new(vec![])),
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_canvas(&self) -> Rc<RefCell<Canvas<Window>>> {
        self.canvas.to_owned()
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn event(&mut self, event: Event) {
        for widget in (*self.widgets).borrow_mut().iter_mut() {
            widget.event(event.clone(), self);
        }
    }

    pub fn update(&mut self) {
        (self.update)(self.canvas.clone(), (*self.widgets).borrow_mut());
    }

    pub fn create<F: 'static + FnMut(CanvasCell, RefMut<Vec<Box<dyn Widget>>>)>(
        video_subsystem: &VideoSubsystem,
        title: &str,
        width: u32,
        height: u32,
        update: F,
    ) -> Self {
        let canvas = to_canvas(add_window(video_subsystem, title, width, height).unwrap()).unwrap();
        MyWindow::new(update, canvas.id, canvas.canvas, true)
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        (*self.widgets).borrow_mut().push(widget);
    }
}

struct MyCanvas {
    id: u32,
    canvas: CanvasCell,
}

impl MyCanvas {
    fn new(id: u32, canvas: CanvasCell) -> Self {
        Self { id, canvas }
    }
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
