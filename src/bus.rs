pub trait Bus {
    fn new() -> Self;
    fn read(&self, address: u16) -> u8;
    fn read_u16(&self, address: u16) -> u16 {
        let low = u16::from(self.read(address));
        let high = u16::from(self.read(address.wrapping_add(1)));
        (high << 8) | low
    }
    fn read_u16_zp(&self, address: u8) -> u16 {
        let low = u16::from(self.read(u16::from(address)));
        let high = u16::from(self.read(address.wrapping_add(1) as u16));
        (high << 8) | low
    }
    fn write(&mut self, value: u8, address: u16);
}

pub struct TestBus {
    pub memory: [u8; 0x10000]
}

impl Bus for TestBus {
    fn new() -> Self {
        Self {
            memory: [0; 0x10000]
        }
    }
    fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    fn write(&mut self, value: u8, address: u16) {
        self.memory[address as usize] = value;
    }
}
