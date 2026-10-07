use crate::console::Console;

pub trait Command {
    fn handle(&self, emulator: &mut Console);
}

impl Console {
    pub fn execute_command(&mut self, command: &impl Command) -> () {
        command.handle(self);
    }
}

#[derive(Default)]
pub struct Step {
    times: u32,
}

impl Step {
    fn times(mut self, times: u32) -> Self {
        self.times = times;
        self
    }
}
impl Command for Step {
    fn handle(&self, emulator: &mut Console) {
        for time in 0..self.times {
            emulator.cpu.step(&mut emulator.cpu_bus);
        }
    }
}
