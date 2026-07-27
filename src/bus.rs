use std::fs::File;

use crate::cartridge::Cartridge;

pub trait Bus {
    fn new(rom: &mut File) -> Self;
    fn read(&self, address: u16) -> u8;
    fn read_u16(&self, address: u16) -> u16 {
        let low = u16::from(self.read(address));
        let high = u16::from(self.read(address.wrapping_add(1)));
        (high << 8) | low
    }
    fn write(&mut self, value: u8, address: u16);
    fn read_u16_zp(&self, address: u8) -> u16 {
        let low = u16::from(self.read(u16::from(address)));
        let high = u16::from(self.read(address.wrapping_add(1) as u16));
        (high << 8) | low
    }
    fn read_u16_bug(&self, address: u16) -> u16 {
        let low = u16::from(self.read(address));
        let high_address = if (address & 0x00FF) == 0x00FF {
            address & 0xFF00
        } else {
            address.wrapping_add(1)
        };

        let high = u16::from(self.read(high_address));

        (high << 8) | low
    }
}

pub struct NesBus {
    pub ram: [u8; 0x0800],
    pub cartridge: Cartridge,
}

impl Bus for NesBus {
    fn new(rom: &mut File) -> Self {
        Self { ram: [0; 0x0800] , cartridge: Cartridge::load_from_file(rom)}
    }

    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.ram[(address & 0x07FF) as usize],
            _ => {
                panic!("Invalid address range")
            }
        }
    }
    fn write(&mut self, value: u8, address: u16) {
        match address {
            0x0000..=0x1FFF => {
                self.ram[(address & 0x07FF) as usize] = value;
            }
            _ => {
                panic!("Invalid address range")
            }
        }
    }
}

impl NesBus {
}
