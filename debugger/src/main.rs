use core::cartridge::{self, Cartridge};
use std::{fs::File, path::Path, time::Duration};

use sdl3::{event::Event, keyboard::Keycode};

use crate::bitmap::Bitmap;
use egui;
use egui_ash_renderer;

mod bitmap;
mod renderer;
mod sdl;

fn main() -> Result<(), sdl::ContextError> {
    let args: Vec<String> = std::env::args().collect();
    let path = Path::new(&args[1]);
    let mut file = File::open(path)?;
    let cartridge = Cartridge::load_from_file(&mut file);
    let bitmap = Bitmap::from_pattern_table(cartridge.chr_slice(), 0);
    let mut context = sdl::Context::new()?;
    context.renderer.render(bitmap)?;
    context.main_loop()?;
    Ok(())
}
