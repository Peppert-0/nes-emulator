use crate::bus::Bus;

mod console;
mod cpu;
mod bus;
mod cartridge;

fn print_opcodes(opcodes: [cpu::Opcode; 256]) {
    for (i, opcode) in opcodes.iter().enumerate() {
        println!("{:02X}: {:?}", i, opcode);
    }
}

fn print_memory_values(memory: [u8; 0x10000]) {
    for (i, value) in memory.iter().enumerate() {
        println!("{:04X}: {:02X}", i, value);
    }
}

fn main() {
    let mut cpu = cpu::Cpu::new();
    let mut bus = bus::TestBus::new();

    let sum = 0x0800 * 3;
    println!("{:04X}", sum)
}
