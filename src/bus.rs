use std::fs::File;
use std::rc::Rc;
use std::cell::RefCell;
use crate::Shared;

use crate::{cartridge::Cartridge, ppu::PpuRegisters};

pub trait Bus {
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

pub struct CpuBus {
    pub ram: [u8; 0x0800],
    pub cartridge: Shared<Cartridge>,
    pub ppu_registers: Shared<PpuRegisters>,
}

impl Bus for CpuBus {
    fn read(&self, address: u16) -> u8 {
        let cartridge = self.cartridge.borrow();
        match address {
            0x0000..=0x1FFF => self.ram[(address & 0x07FF) as usize],
            0x4020..=0xFFFF => cartridge.cpu_read(address),
            _ => 0,
        }
    }
    fn write(&mut self, value: u8, address: u16) {
        let mut cartridge = self.cartridge.borrow_mut();
        match address {
            0x0000..=0x1FFF => self.ram[(address & 0x07FF) as usize] = value,
            0x4020..=0xFFFF => cartridge.cpu_write(address, value),
            _ => {}
        }
    }
}

impl CpuBus {
    pub fn new(cartridge: Shared<Cartridge>, ppu_registers: Shared<PpuRegisters>) -> Self {
        Self { ram: [0; 0x0800] , cartridge, ppu_registers}
    }
}

pub struct PpuBus {
    pub cartridge: Shared<Cartridge>,
    pub vram: [u8; 0x0800],
    pub palette_ram: [u8; 0x0020],
    pub registers: Shared<PpuRegisters>,
}
