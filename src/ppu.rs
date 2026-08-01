use crate::Shared;

pub struct Ppu {
    v: u16,
    t: u16,
    x: u8,
    w: bool,
    mmio: Shared<PpuRegisters>,
    dot: u16,
    scanline: u16,
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
    pub fn new(registers: Shared<PpuRegisters>) -> Self {
        Self { 
            v: 0, 
            t: 0, 
            x: 0, 
            w: false,
            mmio: registers, 
            dot: 0, 
            scanline: 0, 
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
}
