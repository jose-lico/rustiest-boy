use std::process::exit;

use log::trace;
use winit::{dpi::LogicalSize, event_loop::EventLoop, window};

pub struct Window {
    pub handle: window::Window,
}

impl Window {
    pub fn new(width: u16, height: u16, event_loop: &EventLoop<()>) -> Self {
        trace!("Creating window");

        let window_result = window::WindowBuilder::new()
            .with_title("rustiest-boy")
            .with_inner_size(LogicalSize::new(width, height))
            .with_resizable(false)
            .build(&event_loop);

        let window = match window_result {
            Ok(window) => window,
            Err(error) => {
                eprintln!("Failed to create window: {}", error);
                exit(1);
            }
        };

        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = web_sys::Element::from(window.canvas());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

        Window { handle: window }
    }
}
