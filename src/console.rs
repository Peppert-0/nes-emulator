use crate::cpu;
use crate::bus;
use crate::bus::Bus;

pub struct console {
    cpu: cpu::Cpu,
    bus: bus::NesBus,
}

impl console {
    fn new() -> Self {
        let cpu = cpu::Cpu::new();
        let bus = bus::NesBus::new();

        Self { cpu, bus }
    }
}
