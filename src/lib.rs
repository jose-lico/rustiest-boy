use std::{time::Instant, iter};

use egui_wgpu_backend::{ScreenDescriptor, RenderPass};
use winit::{
    event_loop::{ControlFlow, EventLoop},
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod utils;

use utils::window::Window;

/// A custom event type for the winit app.
enum Event {
    RequestRedraw,
}

/// This is the repaint signal type that egui needs for requesting a repaint from another thread.
/// It sends the custom RequestRedraw event to the winit event loop.
struct ExampleRepaintSignal(std::sync::Mutex<winit::event_loop::EventLoopProxy<Event>>);

impl epi::backend::RepaintSignal for ExampleRepaintSignal {
    fn request_repaint(&self) {
        self.0.lock().unwrap().send_event(Event::RequestRedraw).ok();
    }
}


#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new();

    let mut window = Window::new(500, 450, &event_loop);

    let mut egui_rpass = RenderPass::new(&window.device, wgpu::TextureFormat::Bgra8Unorm , 1);

    let mut demo_app = egui_demo_lib::DemoWindows::default();

    let start_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        window.platform.handle_event(&event);

        match event {
            winit::event::Event::RedrawRequested(..) => {
                window
                    .platform
                    .update_time(start_time.elapsed().as_secs_f64());

                let output_frame = match window.surface.get_current_texture() {
                    Ok(frame) => frame,
                    Err(wgpu::SurfaceError::Outdated) => {
                        // This error occurs when the app is minimized on Windows.
                        // Silently return here to prevent spamming the console with:
                        // "The underlying surface has changed, and therefore the swap chain must be updated"
                        return;
                    }
                    Err(e) => {
                        eprintln!("Dropped frame with error: {}", e);
                        return;
                    }
                };

                let output_view = output_frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                window.platform.begin_frame();

                // Draw the demo application.
                demo_app.ui(&window.platform.context());

                // End the UI frame. We could now handle the output and draw the UI with the backend.
                let full_output = window.platform.end_frame(Some(&window.handle));
                let paint_jobs = window.platform.context().tessellate(full_output.shapes);

                let mut encoder =
                    window
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("encoder"),
                        });

                // Upload all resources for the GPU.
                let screen_descriptor = ScreenDescriptor {
                    physical_width: window.surface_config.width,
                    physical_height: window.surface_config.height,
                    scale_factor: window.handle.scale_factor() as f32,
                };
                let tdelta: egui::TexturesDelta = full_output.textures_delta;

                egui_rpass
                    .add_textures(&window.device, &window.queue, &tdelta)
                    .expect("add texture ok");
                egui_rpass.update_buffers(&window.device, &window.queue, &paint_jobs, &screen_descriptor);

                // Record all render passes.
                egui_rpass
                    .execute(
                        &mut encoder,
                        &output_view,
                        &paint_jobs,
                        &screen_descriptor,
                        Some(wgpu::Color::BLACK),
                    )
                    .unwrap();
                // Submit the commands.
                window.queue.submit(iter::once(encoder.finish()));

                // Redraw egui
                output_frame.present();

                egui_rpass
                    .remove_textures(tdelta)
                    .expect("remove texture ok");
            }

            winit::event::Event::MainEventsCleared | winit::event::Event::UserEvent(()) => {
                window.handle.request_redraw();
            }

            winit::event::Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.handle.id() => match event {
                winit::event::WindowEvent::CloseRequested
                | winit::event::WindowEvent::KeyboardInput {
                    input:
                        winit::event::KeyboardInput {
                            state: winit::event::ElementState::Pressed,
                            virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            _ => {}
        }
    });
}
