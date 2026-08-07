use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::render::TextureAccess;
use sdl3::pixels::PixelFormat;
use std::time::Duration;
use std::fs::File;
use core::{cartridge, debug};

pub fn main() -> std::io::Result<()> {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut rom = File::open("core/tests/roms/nestest.nes");
    let mut rom = rom?;
    let cartridge = cartridge::Cartridge::load_from_file(&mut rom);

    let framebuffer = debug::draw_pattern_framebuffer(cartridge.chr_slice());

    let window = video_subsystem.window("rust-sdl3 demo", 128, 128)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();

    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator.create_texture(PixelFormat::RGBA8888, TextureAccess::Static, 128, 128).unwrap();
    texture.update(None, bytemuck::cast_slice(&framebuffer), 128 * 4).unwrap();

    canvas.clear();
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running Ok(())
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        //canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
