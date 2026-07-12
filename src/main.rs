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
    let mut cpu = cpu::Cpu::new();
    let mut bus = bus::Bus::new();

    bus.write(0xA9, 0xAAAA);
    bus.write(0x01, 0xAAAB);

    println!("{:08b}", cpu.status());
    println!("{:?}", cpu.carry());
    cpu.set_carry(true);
    println!("{:08b}", cpu.status());
    println!("{:?}", cpu.carry());
    cpu.set_carry(false);
    cpu.set_zero(true);
    println!("{:08b}", cpu.status());
    println!("{:?}", cpu.carry());
    println!("{:?}", cpu.zero());
}
