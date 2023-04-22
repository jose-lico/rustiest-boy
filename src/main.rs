extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::ttf;

use std::time::Duration;

static SCREEN_WIDTH: u32 = 500;
static SCREEN_HEIGHT: u32 = 450;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let ttf_context = ttf::init().map_err(|e| e.to_string())?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rustiest-boy", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(209, 13, 115));

    let texture_creator = canvas.texture_creator();

    let font = ttf_context.load_font("res/PokemonGbFont.ttf", 20)?;

    let text = font
        .render("Hello World")
        .blended(Color::RGBA(255, 255, 255, 255))
        .map_err(|e| e.to_string())?;

    let texture = texture_creator
        .create_texture_from_surface(&text)
        .map_err(|e| e.to_string())?;

    let TextureQuery { width, height, .. } = texture.query();

    let target = Rect::new(0, 0, width.try_into().unwrap(), height.try_into().unwrap());

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.clear();

        canvas.copy(&texture, None, Some(target))?;
        
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
