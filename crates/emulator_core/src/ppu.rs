use crate::{
    Shared,
    bus::{Bus, PpuBus},
    cartridge::Cartridge,
};

const PALETTE: [u32; 64] = [
    0x3f4346, 0x000a6c, 0x06007e, 0x280072, 0x41004a, 0x4b0010, 0x440000, 0x2e0a00, 0x0d1e00,
    0x002d00, 0x003200, 0x002c04, 0x001c3f, 0x000000, 0x000000, 0x000000, 0x474d50, 0x0031a5,
    0x0d00b1, 0x4e03b8, 0x700089, 0x810043, 0x7b1200, 0x612c00, 0x394500, 0x0e5900, 0x006200,
    0x005c1b, 0x004a68, 0x000000, 0x000000, 0x000000, 0x17191a, 0x004798, 0x00057c, 0x43008a,
    0x740091, 0x8e0761, 0x870d07, 0xa7680c, 0x848000, 0x5b9500, 0x38a00d, 0x249e4c, 0x248f95,
    0x2f3234, 0x000000, 0x000000, 0x17191a, 0x2a2d2f, 0x25282a, 0x26292b, 0x410055, 0x520440,
    0x4f0309, 0x5c340a, 0x4a4513, 0x404d10, 0x395e11, 0x12542f, 0x18534f, 0x404447, 0x000000,
    0x000000,
];

pub struct Ppu {
    v: u16,
    t: u16,
    x: u8,
    w: bool,
    pub mmio: PpuRegisters,
    dot: u16,
    scanline: u16,
    bus: PpuBus,
    oam: [u8; 256],
    pipeline: Pipeline,
}
#[derive(Default)]
struct Pipeline {
    next_nametable_tile: u8,
    next_attribute_byte: u8,
    pattern_low_bitplane: u8,
    pattern_high_bitplane: u8,

    attribute_shift_low: u8,
    attribute_shift_high: u8,
    pattern_shift_low: u16,
    pattern_shift_high: u16,

    pixels: Vec<[u8; 4]>,
}
pub struct PpuView {
    pub v: u16,
    pub t: u16,
    pub x: u8,
    pub w: bool,
    pub mmio: MmioView,
    pub dot: u16,
    pub scanline: u16,
    pub oam: [u8; 256],
}

pub struct PpuRegisters {
    ppuctrl: u8,
    ppumask: u8,
    ppustatus: u8,
    oamaddr: u8,
    oamdata: u8,
    ppuscroll: u8,
    ppuaddr: u16,
    ppudata: u8,
}
pub struct MmioView {
    pub ppuctrl: PpuCtrlView,
    pub ppumask: u8,
    pub ppustatus: PpuStatusView,
    pub oamaddr: u8,
    pub oamdata: u8,
    pub ppuscroll: u8,
    pub ppuaddr: u16,
    pub ppudata: u8,
}
pub struct PpuCtrlView {
    pub value: u8,
    pub base_nametable_address: String,
    pub vram_address_increment: String,
    pub sprite_pattern_table_address: String,
    pub background_pattern_table_address: String,
    pub sprite_size: String,
    pub vblank_nmi: String,
}
impl PpuCtrlView {
    fn new(value: u8) -> Self {
        let base_nametable_address = match value & 3 {
            0 => "0x2000",
            1 => "0x2400",
            2 => "0x2800",
            3 => "0x2C00",
            _ => unreachable!(),
        }
        .to_string();
        let vram_address_increment = match (value >> 2) & 1 {
            0 => "1 (across)",
            1 => "32 (down)",
            _ => unreachable!(),
        }
        .to_string();
        let sprite_pattern_table_address = match (value >> 3) & 1 {
            0 => "0x0000",
            1 => "0x1000",
            _ => unreachable!(),
        }
        .to_string();
        let background_pattern_table_address = match (value >> 4) & 1 {
            0 => "0x0000",
            1 => "0x1000",
            _ => unreachable!(),
        }
        .to_string();
        let sprite_size = match (value >> 6) & 1 {
            0 => "8x8 pixels",
            1 => "8x16 pixels",
            _ => unreachable!(),
        }
        .to_string();
        let vblank_nmi = match (value >> 7) & 1 {
            0 => "off",
            1 => "on",
            _ => unreachable!(),
        }
        .to_string();
        Self {
            value,
            base_nametable_address,
            vram_address_increment,
            sprite_pattern_table_address,
            background_pattern_table_address,
            sprite_size,
            vblank_nmi,
        }
    }
}
pub struct PpuStatusView {
    pub value: u8,
    pub sprite_overflow: bool,
    pub sprite_0_hit: bool,
    pub vblank: bool,
}
impl PpuStatusView {
    fn new(value: u8) -> Self {
        let sprite_overflow = (value >> 5) & 1 == 1;
        let sprite_0_hit = (value >> 6) & 1 == 1;
        let vblank = (value >> 7) & 1 == 1;
        Self {
            value,
            sprite_overflow,
            sprite_0_hit,
            vblank,
        }
    }
}
impl MmioView {
    pub fn new(registers: &PpuRegisters) -> Self {
        Self {
            ppuctrl: PpuCtrlView::new(registers.ppuctrl),
            ppumask: registers.ppumask,
            ppustatus: PpuStatusView::new(registers.ppustatus),
            oamaddr: registers.oamaddr,
            oamdata: registers.oamdata,
            ppuscroll: registers.ppuscroll,
            ppuaddr: registers.ppuaddr,
            ppudata: registers.ppudata,
        }
    }
}

const CTRL_NMI: u8 = 1 << 7;

const STATUS_VBLANK: u8 = 1 << 7;

impl Ppu {
    pub fn new() -> Self {
        let registers = PpuRegisters::new();
        let bus = PpuBus::new();
        let pipeline = Pipeline::default();
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
            pipeline,
        }
    }

    pub fn tick(&mut self, cartridge: &Cartridge) {
        if self.scanline == 241 && self.dot == 1 {
            self.mmio.set_status_vblank(true);
        } else if self.scanline == 261 && self.dot == 1 {
            self.mmio.set_status_vblank(false);
        }
        self.vram_fetch(cartridge);
        if (0..=239).contains(&self.scanline) {
            if self.dot == 256 {
                self.increment_vertical();
            } else if self.dot == 257 {
                self.v = self.t;
            }
        }

        self.shift_registers();

        self.dot += 1;

        if self.dot == 341 {
            self.dot = 0;

            self.scanline += 1;

            if self.scanline == 262 {
                self.scanline = 0;
            }
        }
    }
    pub fn view(&self) -> PpuView {
        let mmio_view = MmioView::new(&self.mmio);
        PpuView {
            v: self.v,
            t: self.t,
            x: self.x,
            w: self.w,
            mmio: mmio_view,
            dot: self.dot,
            scanline: self.scanline,
            oam: self.oam,
        }
    }

    fn vram_fetch(&mut self, cartridge: &Cartridge) {
        if (0..=239).contains(&self.scanline) {
            if self.dot % 8 == 1 {
                self.pipeline.next_nametable_tile = self.fetch_nametable_byte(cartridge);
            } else if self.dot % 8 == 3 {
                self.pipeline.next_attribute_byte = self.fetch_attribute_byte(cartridge);
            } else if self.dot % 8 == 5 {
                self.pipeline.pattern_low_bitplane = self.fetch_pattern_byte_low(
                    cartridge,
                    self.fetch_pattern_address(self.pipeline.next_nametable_tile),
                );
            } else if self.dot % 8 == 7 {
                self.pipeline.pattern_high_bitplane = self.fetch_pattern_byte_high(
                    cartridge,
                    self.fetch_pattern_address(self.pipeline.next_nametable_tile),
                );
            } else if self.dot % 8 == 0 {
                self.increment_horizontal();
            }
        }
    }
    fn fetch_nametable_byte(&self, cartridge: &Cartridge) -> u8 {
        let address = 0x2000 | (self.v & 0x0FFF);
        self.bus.read(cartridge, address)
    }
    fn fetch_attribute_byte(&self, cartridge: &Cartridge) -> u8 {
        let address = 0x23C0 | (self.v & 0x0C00) | ((self.v >> 4) & 0x38) | ((self.v >> 2) & 0x07);
        self.bus.read(cartridge, address)
    }
    fn fetch_pattern_address(&self, tile: u8) -> u16 {
        ((self.mmio.ctrl_pattern_table() as u16) << 0xC)
            | ((tile as u16) << 4)
            | self.fine_y() as u16
    }
    fn fetch_pattern_byte_low(&self, cartridge: &Cartridge, address: u16) -> u8 {
        self.bus.read(cartridge, address)
    }
    fn fetch_pattern_byte_high(&self, cartridge: &Cartridge, address: u16) -> u8 {
        self.bus.read(cartridge, address + 8)
    }
    fn push_pixel(&mut self) {
        let pixel = self.fetch_pixel() & (1 << 4);
    }
    fn fetch_pixel(&self) -> u8 {
        ((self.pipeline.pattern_shift_low as u8) & self.x)
            | (((self.pipeline.pattern_shift_high as u8) & self.x) << 1)
            | ((self.pipeline.attribute_shift_low & self.x) << 2)
            | ((self.pipeline.attribute_shift_high & self.x) << 3)
    }
    fn shift_registers(&mut self) {
        let quadrant = (self.course_y() & 2) << 1 | self.course_x() & 2;
        let attribute_bits = (self.pipeline.next_attribute_byte >> quadrant) & 3;
        let attribute_bit_high = (attribute_bits & 2) >> 1;
        let attribute_bit_low = attribute_bits & 1;
        self.pipeline.attribute_shift_high =
            (self.pipeline.attribute_shift_high >> 1) & (attribute_bit_high << 7);
        self.pipeline.attribute_shift_low =
            (self.pipeline.attribute_shift_low >> 1) & (attribute_bit_low << 7);
        self.pipeline.attribute_shift_low >>= 1;
        self.pipeline.pattern_shift_high >>= 1;
        self.pipeline.pattern_shift_low >>= 1;
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
    pub fn cpu_read(&mut self, address: u16) -> u8 {
        match address {
            0x2000 => self.mmio.ppuctrl,
            0x2001 => self.mmio.ppumask,
            0x2002 => {
                self.w = false;
                self.mmio.ppustatus
            }
            0x2003 => self.mmio.oamaddr,
            0x2004 => self.mmio.oamdata,
            0x2005 => self.mmio.ppuscroll,
            0x2007 => self.mmio.ppudata,
            _ => 0,
        }
    }
    pub fn cpu_write(&mut self, address: u16, value: u8) {
        match address {
            0x2000 => self.mmio.ppuctrl = value,
            0x2001 => self.mmio.ppumask = value,
            0x2003 => self.mmio.oamaddr = value,
            0x2004 => self.mmio.oamdata = value,
            0x2005 => self.mmio.ppuscroll = value,
            0x2006 => {
                if self.w {
                    self.mmio.ppuaddr |= value as u16;
                } else {
                    self.mmio.ppuaddr |= (value as u16) << 8;
                }
            }
            0x2007 => self.mmio.ppudata = value,
            _ => {}
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
        if value {
            self.ppuctrl |= CTRL_NMI
        } else {
            self.ppuctrl &= !CTRL_NMI
        }
    }
    fn set_status_vblank(&mut self, value: bool) {
        if value {
            self.ppustatus |= STATUS_VBLANK
        } else {
            self.ppustatus &= !STATUS_VBLANK
        }
    }
    fn ctrl_pattern_table(&self) -> u8 {
        (self.ppuctrl & 0x10) >> 4
    }
}
