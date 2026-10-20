use crate::{Shared, cartridge, ppu::Ppu};
use std::cell::RefCell;
use std::fs::File;
use std::rc::Rc;

use crate::{cartridge::Cartridge, ppu::PpuRegisters};

pub trait Bus {
    fn read(&mut self, address: u16) -> u8;
    fn immutable_read(&self, address: u16) -> u8;
    fn read_u16(&mut self, address: u16) -> u16;
    fn immutable_read_u16(&self, address: u16) -> u16;
    fn write(&mut self, value: u8, address: u16);
    fn read_u16_zp(&mut self, address: u8) -> u16;
    fn read_u16_bug(&mut self, address: u16) -> u16;
}

pub struct CpuBus {
    pub ram: [u8; 0x0800],
    pub ppu: Ppu,
    pub cartridge: Cartridge,
    pub cycles: u64,
}

impl Bus for CpuBus {
    fn read(&mut self, address: u16) -> u8 {
        self.cycles += 1;
        for _ in 0..3 {
            self.ppu.tick(&self.cartridge);
        }
        match address {
            0x0000..=0x1FFF => self.ram[(address & 0x07FF) as usize],
            0x2000..=0x3FFF => self.ppu.mmio.cpu_read(0x2000 + (address & 0x0007)),
            0x4020..=0xFFFF => self.cartridge.cpu_read(address),
            _ => 0,
        }
    }
    fn immutable_read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.ram[(address & 0x07FF) as usize],
            0x2000..=0x3FFF => self.ppu.mmio.cpu_read(0x2000 + (address & 0x0007)),
            0x4020..=0xFFFF => self.cartridge.cpu_read(address),
            _ => 0,
        }
    }
    fn write(&mut self, value: u8, address: u16) {
        self.cycles += 1;
        for _ in 0..3 {
            self.ppu.tick(&self.cartridge);
        }
        match address {
            0x0000..=0x1FFF => self.ram[(address & 0x07FF) as usize] = value,
            0x2000..=0x3FFF => self.ppu.mmio.cpu_write(0x2000 + (address & 0x0007), value),
            0x4020..=0xFFFF => self.cartridge.cpu_write(address, value),
            _ => {}
        }
    }
    fn read_u16(&mut self, address: u16) -> u16 {
        let low = u16::from(self.read(address));
        let high = u16::from(self.read(address.wrapping_add(1)));
        (high << 8) | low
    }
    fn immutable_read_u16(&self, address: u16) -> u16 {
        let low = u16::from(self.immutable_read(address));
        let high = u16::from(self.immutable_read(address.wrapping_add(1)));
        (high << 8) | low
    }
    fn read_u16_zp(&mut self, address: u8) -> u16 {
        let low = u16::from(self.read(u16::from(address)));
        let high = u16::from(self.read(address.wrapping_add(1) as u16));
        (high << 8) | low
    }
    fn read_u16_bug(&mut self, address: u16) -> u16 {
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

impl CpuBus {
    pub fn new(cartridge: Cartridge, ppu: Ppu) -> Self {
        Self {
            ram: [0; 0x0800],
            ppu,
            cartridge,
            cycles: 0,
        }
    }
}

pub struct PpuBus {
    pub vram: [u8; 0x0800],
    pub palette_ram: [u8; 0x0020],
}

impl PpuBus {
    pub fn read(&self, cartridge: &Cartridge, address: u16) -> u8 {
        match address {
            0x0000..=0x3EFF => cartridge
                .mapper
                .ppu_read(&cartridge.chr, &self.vram, address),
            0x3F00..=0x3FFF => {
                let offset = address - 0x3F00;
                self.palette_ram[(offset & 0x0020) as usize]
            }
            _ => 0,
        }
    }
    pub fn write(&mut self, cartridge: &mut Cartridge, value: u8, address: u16) {
        match address {
            0x3F00..=0x3FFF => {
                let offset = address - 0x3F00;
                self.palette_ram[(offset & 0x0020) as usize] = value;
            }
            _ => {}
        }
    }
}

impl PpuBus {
    pub fn new() -> Self {
        Self {
            vram: [0; 0x0800],
            palette_ram: [0; 0x0020],
        }
    }
}
