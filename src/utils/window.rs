use winit::{
    dpi::LogicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder, WindowId},
};

pub struct MyWindow {
    pub id: WindowId,
    win: Window,
    width: u16,
    height: u16,
}

impl MyWindow {
    pub fn new(w: u16, h: u16, event_loop: &EventLoop<()>) -> Self {
        println!("Creating window!");

        let window = WindowBuilder::new()
            .with_title("rustiest-boy")
            .with_inner_size(LogicalSize::new(w, h))
            .with_resizable(false)
            .build(&event_loop)
            .unwrap();

        let i = window.id();

        MyWindow {
            width: w,
            height: h,
            win: window,
            id: i,
        }
    }
}
