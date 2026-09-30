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
    let mut context = if args.len() > 1 {
        let path = Path::new(&args[1]);
        let cartridge = sdl::Context::load_rom(path)?;
        sdl::Context::new(cartridge)?
    } else {
        sdl::Context::new(None)?
    };

    context.main_loop()?;
    Ok(())
}
