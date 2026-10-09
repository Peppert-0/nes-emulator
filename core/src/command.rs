use builder::Builder;

use crate::console::Console;

pub trait Command {
    fn handle(&self, emulator: &mut Console);
}

impl Console {
    pub fn execute_command(&mut self, command: &impl Command) -> () {
        command.handle(self);
    }
}

#[derive(Builder)]
pub struct Step {
    #[builder(default = 1)]
    times: u32,
}
#[derive(Builder)]
pub struct Reset {}

impl Command for Step {
    fn handle(&self, emulator: &mut Console) {
        for _time in 0..self.times {
            emulator.cpu.step(&mut emulator.cpu_bus);
            for _i in 0..3 {
                emulator.ppu.tick();
            }
        }
    }
}

impl Command for Reset {
    fn handle(&self, emulator: &mut Console) {
        emulator.cpu.reset(&emulator.cpu_bus);
    }
}
