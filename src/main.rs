use std::fs::File;

use crate::bus::Bus;

mod console;
mod cpu;
mod bus;
mod cartridge;

fn main() -> std::io::Result<()> {
    let mut rom = File::open("nestest.nes")?;
    let mut console = console::console::new(&mut rom);
    //console.cpu.reset(&console.bus);
    console.cpu.pc = 0xC000;
    println!("{}", console.cpu.trace(&console.bus));
    console.cpu.step(&mut console.bus);
    println!("{}", console.cpu.trace(&console.bus));
    Ok(())
}
