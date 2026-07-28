use std::fs::File;
use nes_emulator::console;

fn debug_step(console: &mut console::console) {
    println!("{}", console.cpu.trace(&console.bus));
    console.cpu.step(&mut console.bus);
}

#[test]
fn test_rom() -> std::io::Result<()> {
    let mut rom = File::open("tests/roms/nestest.nes")?;
    let mut console = console::console::new(&mut rom);
    console.cpu.pc = 0xC000;

    for _ in 0..8991 {
        debug_step(&mut console);
    }

    Ok(())
}
