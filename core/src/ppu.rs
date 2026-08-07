use crate::{Shared, bus::{Bus, PpuBus}};

pub struct Ppu {
    v: u16,
    t: u16,
    x: u8,
    w: bool,
    mmio: Shared<PpuRegisters>,
    dot: u16,
    scanline: u16,
    bus: PpuBus,
    oam: [u8; 256],
    attribute_shift_low: u8,
    attribute_shift_high: u8,
    pattern_shift_low: u16,
    pattern_shift_high: u16,
} 

pub struct PpuRegisters {
    ppuctrl: u8,
    ppumask: u8,
    ppustatus: u8,
    oamaddr: u8,
    oamdata: u8,
    ppuscroll: u8,
    ppuaddr: u8,
    ppudata: u8,
}

const CTRL_NMI: u8 = 1 << 7;

const STATUS_VBLANK: u8 = 1 << 7;

impl Ppu {
    pub fn new(registers: Shared<PpuRegisters>, bus: PpuBus) -> Self {
        Self { 
            v: 0, 
            t: 0, 
            x: 0, 
            w: false,
            mmio: registers, 
            dot: 0, 
            scanline: 0, 
            bus,
            oam: [0; 256],
            attribute_shift_low: 0,
            attribute_shift_high: 0,
            pattern_shift_low: 0,
            pattern_shift_high: 0,
        }
    }

    fn tick(&mut self) {
        let mut registers = self.mmio.borrow_mut();

        if self.scanline == 241 && self.dot == 1 {
            registers.set_status_vblank(true);
        } else if self.scanline == 261 && self.dot == 1 {
            registers.set_status_vblank(false);
        }

        self.dot += 1;

        if self.dot == 341 {
            self.dot = 0;

            self.scanline += 1;

            if self.scanline == 262 {
                self.scanline = 0;
            }
        }
    }

    fn cpu_read(&mut self, address: u16) -> u8 {
        let mut registers = self.mmio.borrow_mut();
        match address {
            0x2000 => registers.ppuctrl,
            0x2001 => registers.ppumask,
            0x2002 => {
                let status = registers.ppustatus;
                registers.set_status_vblank(false);
                self.w = false;
                status
            },
            0x2003 => registers.oamaddr,
            0x2004 => registers.oamdata,
            0x2005 => registers.ppuscroll,
            0x2006 => registers.ppuaddr,
            0x2007 => registers.ppudata,
            _ => 0,
        }
    }
    fn cpu_write(&mut self, address: u16, value: u8) {
        let mut registers = self.mmio.borrow_mut();
        match address {
            0x2000 => registers.ppuctrl = value,
            0x2001 => registers.ppumask = value,
            0x2003 => registers.oamaddr = value,
            0x2004 => registers.oamdata = value,
            0x2005 => registers.ppuscroll = value,
            0x2006 => registers.ppuaddr = value,
            0x2007 => registers.ppudata = value,
            _ => {},
        }
    }

    fn fetch_nametable_byte(&self) -> u8 {
        let address = 0x2000 | (self.v & 0x0FFF);
        self.bus.read(address)
    }
    fn fetch_attribute_byte(&self) -> u8 {
        let address = 0x23C0 | (self.v & 0x0C00) | ((self.v >> 4) & 0x38) | ((self.v >> 2) & 0x07);
        self.bus.read(address)
    }
    fn fetch_pattern_address(&self, tile: u8) -> u16 {
        let mmio = self.mmio.borrow();
        ((mmio.ctrl_pattern_table() as u16) << 0xC) 
        | ((tile as u16) << 4)
        | self.fine_y() as u16
    }
    fn fetch_pattern_byte_low(&self, address: u16) -> u8 {
        self.bus.read(address)
    }
    fn fetch_pattern_byte_high(&self, address: u16) -> u8 {
        self.bus.read(address + 8)
    }
    fn fetch_palette_low(&self, attribute: u8) -> u8 {
        let quadrant = ((self.course_y() & 2) >> 1) | (self.course_x() & 2);
        match quadrant {
            0 => attribute & 0x01,
            1 => (attribute & 0x04) >> 2,
            2 => (attribute & 0x10) >> 4,
            3 => (attribute & 0x40) >> 6,
            _ => 0,
        }
    }
    fn fetch_palette_high(&self, attribute: u8) -> u8 {
        let quadrant = ((self.course_y() & 2) >> 1) | (self.course_x() & 2);
        match quadrant {
            0 => (attribute & 0x02) >> 1,
            1 => (attribute & 0x08) >> 3,
            2 => (attribute & 0x20) >> 5,
            3 => (attribute & 0x80) >> 7,
            _ => 0,
        }
    }
    fn fetch_pixel(&self) -> u8 {
        ((self.pattern_shift_low as u8) & self.x) |
        (((self.pattern_shift_high as u8) & self.x) << 1) |
        ((self.attribute_shift_low & self.x) << 2) |
        ((self.attribute_shift_high & self.x) << 3)
    }
    fn shift_registers(&mut self) {
        self.attribute_shift_high >>= 1;
        self.attribute_shift_low >>= 1;
        self.pattern_shift_high >>= 1;
        self.pattern_shift_low >>= 1;
    }

    fn increment_horizontal(&mut self) {
        if (self.v & 0x001F) == 31 {
            self.v &= !0x001F;
            self.v ^= 0x0400;
        } else {
            self.v += 1;
        }
    }
    fn increment_vertical(&mut self) {
        if (self.v & 0x7000) != 0x7000 {
            self.v += 0x1000;
        } else {
            self.v &= !0x7000;
            let mut y: u16 = (self.v & 0x03E0) >> 5;
            if y == 29 {
                y = 0;
                self.v ^= 0x0800;
            } else if y == 31 {
                y = 0;
            } else {
                y += 1;
            }
            self.v = (self.v & !0x03E0) | (y << 5);
        }
    }

    fn fine_y(&self) -> u8 {
        ((self.v & 0x7000) >> 12) as u8
    }
    fn course_x(&self) -> u8 {
        (self.v & 0x001F) as u8
    }
    fn course_y(&self) -> u8 {
        ((self.v & 0x03E0) >> 5) as u8
    }
}

impl PpuRegisters {
    pub fn new() -> Self {
        Self { 
            ppuctrl: 0, 
            ppumask: 0, 
            ppustatus: 0, 
            oamaddr: 0, 
            oamdata: 0, 
            ppuscroll: 0, 
            ppuaddr: 0, 
            ppudata: 0, 
        }
    }

    fn set_ctrl_nmi(&mut self, value: bool) {
        if value {self.ppuctrl |= CTRL_NMI}
        else {self.ppuctrl &= !CTRL_NMI}
    }
    fn set_status_vblank(&mut self, value: bool) {
        if value {self.ppustatus |= STATUS_VBLANK}
        else {self.ppustatus &= !STATUS_VBLANK}
    }
    fn ctrl_pattern_table(&self) -> u8 {
        (self.ppuctrl & 0x10) >> 4
    }
}
