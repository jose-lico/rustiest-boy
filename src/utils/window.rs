use winit::{
    dpi::LogicalSize,
    event_loop::{EventLoop},
    window::{Window, WindowBuilder, WindowId},
};

pub struct MyWindow {
    pub id: WindowId,
    pub window: Window,
}

impl MyWindow {
    pub fn new(width: u16, height: u16, event_loop: &EventLoop<()>) -> Self {
        println!("Creating window!");

        let window = WindowBuilder::new()
            .with_title("rustiest-boy")
            .with_inner_size(LogicalSize::new(width, height))
            .with_resizable(false)
            .build(&event_loop)
            .unwrap();

        let id = window.id();

        MyWindow {
            window,
            id,
        }
    }
}
