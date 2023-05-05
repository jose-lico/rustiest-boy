use std::process::exit;

use egui::FontDefinitions;
use egui_winit_platform::{Platform, PlatformDescriptor};
use log::{error, trace};
use wgpu::{Surface, Device, SurfaceConfiguration, Queue};
use winit::{dpi::LogicalSize, event_loop::EventLoop, window};

pub struct Window {
    pub handle: window::Window,
    pub platform : Platform,
    pub surface : Surface,
    pub device: Device,
    pub surface_config : SurfaceConfiguration,
    pub queue : Queue
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
                error!("Failed to create window: {}", error);
                exit(1);
            }
        };

        let descriptor = wgpu::InstanceDescriptor::default();
        let instance = wgpu::Instance::new(descriptor);
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        // WGPU 0.11+ support force fallback (if HW implementation not supported), set it to true or false (optional).
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::default(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ))
        .unwrap();

        let size = window.inner_size();
        let surface_format = surface.get_capabilities(&adapter).formats;
        //let surface_format = surface.get_current_texture().unwrap().texture.format();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8Unorm,
            width: size.width as u32,
            height: size.height as u32,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: surface_format,
        };

        surface.configure(&device, &surface_config);

        let platform = Platform::new(PlatformDescriptor {
            physical_width: size.width as u32,
            physical_height: size.height as u32,
            scale_factor: window.scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

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

        Window { handle: window, platform, surface, device, surface_config, queue }
    }
}
