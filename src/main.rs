extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::ttf;
use sdl2::video::GLProfile;

use std::time::Duration;

static SCREEN_WIDTH: u32 = 500;
static SCREEN_HEIGHT: u32 = 650;

const TARGET_FRAME_TIME: u64 = 1_000_000_000 / 60;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn find_sdl_dx11_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "direct3d11" {
            return Some(index as u32);
        }
    }
    None
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let ttf_context = ttf::init().map_err(|e| e.to_string())?;
    let video_subsystem = sdl_context.video()?;
    let timer_subsystem = sdl_context.timer()?;

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(4, 2);
    gl_attr.set_accelerated_visual(true);

    let window = video_subsystem
        .window("rustiest-boy", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.
        into_canvas()
        .index(find_sdl_dx11_driver().unwrap())
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    let font = ttf_context.load_font("res/PokemonGbFont.ttf", 20)?;

    let mut text = font
        .render("Hello World")
        .blended(Color::RGBA(255, 255, 255, 255))
        .map_err(|e| e.to_string())?;

    let mut text_texture = texture_creator
        .create_texture_from_surface(&text)
        .map_err(|e| e.to_string())?;

    let TextureQuery { width, height, .. } = text_texture.query();

    let mut text_rect = Rect::new(0, 0, width.try_into().unwrap(), height.try_into().unwrap());

    let background_rect = Rect::new(0, 0, 500, 200);

    let mut event_pump = sdl_context.event_pump()?;

    let mut cap_framerate = true;

    'running: loop {
        let frame_start = timer_subsystem.performance_counter();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::V),
                    ..
                } => cap_framerate = !cap_framerate,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(209, 13, 115));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(35, 35, 35));
        canvas.fill_rect(background_rect).unwrap();

        canvas.copy(&text_texture, None, Some(text_rect))?;

        canvas.present();

        // Fake processing ~ 4.16ms
        ::std::thread::sleep(Duration::new(
            0,
            (TARGET_FRAME_TIME / 4).try_into().unwrap(),
        ));

        let emulation_end = timer_subsystem.performance_counter();

        // Might be redudant but keep for now just in case
        let elapsed =
            (emulation_end - frame_start) * 1_000_000_000 / timer_subsystem.performance_frequency();

        if cap_framerate && elapsed < TARGET_FRAME_TIME {
            ::std::thread::sleep(Duration::new(
                0,
                (TARGET_FRAME_TIME - elapsed).try_into().unwrap(),
            ));
        }

        let frame_end = timer_subsystem.performance_counter();

        let framerate = 1_000_000_000.0 / (frame_end - frame_start) as f32;

        //println!("fps: {0}", framerate);

        text = font
            .render(format!("FPS: {:.2}", framerate).as_str())
            .blended(Color::RGBA(255, 255, 255, 255))
            .map_err(|e| e.to_string())?;

        text_texture = texture_creator
            .create_texture_from_surface(&text)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = text_texture.query();

        text_rect = Rect::new(0, 0, width.try_into().unwrap(), height.try_into().unwrap());
    }

    println!("Context: {0}.{1}", gl_attr.context_major_version(),  gl_attr.context_minor_version());

    Ok(())
}
