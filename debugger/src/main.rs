use core::cartridge::{self, Cartridge};
use std::{fs::File, path::Path, time::Duration};

use sdl3::{event::Event, keyboard::Keycode};

use crate::bitmap::Bitmap;
use egui;
use egui_ash_renderer;

mod bitmap;
mod gui;
mod renderer;
mod sdl;

fn main() -> Result<(), sdl::ContextError> {
    let mut context = sdl::Context::new()?;
    context.main_loop()?;
    Ok(())
}
