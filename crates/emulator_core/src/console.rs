use std::cell::RefCell;
use std::fs::File;
use std::rc::Rc;

use crate::bus::{CpuBus, PpuBus};
use crate::cartridge::{Cartridge, CartridgeView, NesParseError};
use crate::cpu::{Cpu, CpuView};
use crate::ppu::{Ppu, PpuRegisters, PpuView};

pub struct Console {
    pub cpu: Cpu,
    pub cpu_bus: CpuBus,
}
pub struct ConsoleView {
    pub cpu_view: CpuView,
    pub cartridge_view: CartridgeView,
    pub ppu_view: PpuView,
}

impl Console {
    pub fn new(rom: &mut File) -> Result<Self, NesParseError> {
        let cartridge = Cartridge::load_from_file(rom)?;
        let ppu = Ppu::new();
        let cpu_bus = CpuBus::new(cartridge, ppu);
        let cpu = Cpu::new();

        Ok(Self { cpu, cpu_bus })
    }
}
impl ConsoleView {
    pub fn new(console: &mut Console) -> Self {
        let cpu_view = console.cpu.view();
        let cartridge_view = console.cpu_bus.cartridge.view();
        let ppu_view = console.cpu_bus.ppu.view();

        Self {
            cpu_view,
            cartridge_view,
            ppu_view,
        }
    }
    pub fn update(&mut self, console: &mut Console) {
        self.cpu_view.update(&mut console.cpu, &console.cpu_bus);
        self.cartridge_view = console.cpu_bus.cartridge.view();
        self.ppu_view = console.cpu_bus.ppu.view();
    }
}
