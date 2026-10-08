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

impl Command for Step {
    fn handle(&self, emulator: &mut Console) {
        for time in 0..self.times {
            emulator.cpu.step(&mut emulator.cpu_bus);
        }
    }
}
