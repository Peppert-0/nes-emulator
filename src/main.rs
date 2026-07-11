mod cpu;
mod bus;

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
    //print_opcodes(cpu::OPCODES);

    let mut memory_bus = bus::Bus::new();
    memory_bus.write(0xFF, 0xAAAA);
    let value = memory_bus.read(0xAAAA);
    println!{"{:02X}", value};
}

