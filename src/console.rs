use std::fs::File;

use crate::cpu;
use crate::bus;
use crate::bus::Bus;

pub struct Console {
    pub cpu: cpu::Cpu,
    pub bus: bus::NesBus,
}

impl Console {
    pub fn new(rom: &mut File) -> Self {
        let cpu = cpu::Cpu::new();
        let bus = bus::NesBus::new(rom);

        Self { cpu, bus }
    }
}
