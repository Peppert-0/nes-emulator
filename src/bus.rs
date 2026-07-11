pub struct Bus {
    pub memory: [u8; 0x10000]
}

impl Bus {
    pub fn new() -> Self {
        Self {
            memory: [0; 0x10000]
        }
    }
    pub fn read(self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    pub fn write(&mut self, value: u8, address: u16) {
        self.memory[address as usize] = value;
    }
}
