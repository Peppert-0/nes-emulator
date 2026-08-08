use sdl3::{event::Event, rect::Rect};
use sdl3::keyboard::Keycode;
use sdl3::render::TextureAccess;
use std::path::Path;
use std::time::Duration;
use std::fs::File;
use std::ffi::CString;
use core::{cartridge, debug};
use sdl3::pixels::PixelFormat;
use bytemuck;

use crate::sdl::ContextError;
mod renderer;
mod sdl;

pub fn main() -> Result<(), ContextError> {
    let args: Vec<String> = std::env::args().collect();
    let filename = String::from(&args[1]);
    let path = Path::new(&filename);

    let mut context = sdl::Context::new()?;

    let mut rom = File::open(path);
    let mut rom = rom?;
    let cartridge = cartridge::Cartridge::load_from_file(&mut rom);

    let mut canvas = context.window.into_canvas();

    let pattern_table_0 = debug::draw_pattern_framebuffer(cartridge.chr_slice(), 0);
    let pattern_table_1 = debug::draw_pattern_framebuffer(cartridge.chr_slice(), 1);

    let texture_creator = canvas.texture_creator();

    let mut texture0 = texture_creator.create_texture(PixelFormat::RGBA8888, TextureAccess::Static, 128, 128).unwrap();
    texture0.update(None, bytemuck::cast_slice(&pattern_table_0), 128 * 4).unwrap();
    let mut texture1 = texture_creator.create_texture(PixelFormat::RGBA8888, TextureAccess::Static, 128, 128).unwrap();
    texture1.update(None, bytemuck::cast_slice(&pattern_table_1), 128 * 4).unwrap();

    canvas.clear();
    canvas.copy(&texture0, None, Rect::new(0, 0, 128, 128));
    canvas.copy(&texture1, None, Rect::new(128, 0, 128, 128));
    canvas.present();


    Ok(())
}
