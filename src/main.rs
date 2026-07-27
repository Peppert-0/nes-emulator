use std::fs::File;

mod console;
mod cpu;
mod bus;
mod cartridge;

fn main() -> std::io::Result<()> {
    let mut rom = File::open("nestest.nes")?;
    let mut console = console::console::new(&mut rom);
    console.cpu.step(&mut console.bus);
    Ok(())
}
