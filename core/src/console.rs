use std::cell::RefCell;
use std::rc::Rc;
use std::fs::File;

use crate::Shared;
use crate::cartridge::Cartridge;
use crate::cpu::Cpu;
use crate::bus::{CpuBus, PpuBus};
use crate::ppu::{Ppu, PpuRegisters};

pub struct Console {
    pub cpu: Cpu,
    pub cpu_bus: CpuBus,
    pub cartridge: Shared<Cartridge>,
    pub ppu: Ppu,
    pub ppu_registers: Shared<PpuRegisters>,
}

impl Console {
    pub fn new(rom: &mut File) -> Self {
        let cartridge = Rc::new(RefCell::new(Cartridge::load_from_file(rom)));
        let ppu_registers = Rc::new(RefCell::new(PpuRegisters::new()));
        let cpu = Cpu::new();
        let cpu_bus = CpuBus::new(cartridge.clone(), ppu_registers.clone());
        let ppu_bus = PpuBus::new(cartridge.clone());
        let ppu = Ppu::new(ppu_registers.clone(), ppu_bus);

        Self {cpu, cpu_bus, cartridge, ppu, ppu_registers} 
    }
}
