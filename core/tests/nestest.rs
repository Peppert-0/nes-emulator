use core::{console, cpu::CpuView};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq)]
struct CpuState {
    pc: String,
    bytes: String,
    instruction: String,
    a: String,
    x: String,
    y: String,
    p: String,
    sp: String,
    cycles: String,
}

impl CpuState {
    fn from_cpu_view(mut view: CpuView, cycles: String) -> Self {
        let instruction_view = view.instructions.pop_back().unwrap();
        let pc = format!("{:04X}", instruction_view.address);
        let mut bytes: String = instruction_view
            .bytes
            .iter()
            .map(|byte| format!("{:02X} ", byte))
            .collect();
        bytes = bytes.trim_end().to_string();
        let mut instruction = instruction_view.mnemonic.clone();
        instruction = if instruction == String::from("ISC") {
            String::from("ISB")
        } else {
            instruction
        };
        let a = format!("{:02X}", view.registers.a);
        let x = format!("{:02X}", view.registers.x);
        let y = format!("{:02X}", view.registers.y);
        let p = format!("{:02X}", view.registers.p);
        let sp = format!("{:02X}", view.registers.sp);

        Self {
            pc,
            bytes,
            instruction,
            a,
            x,
            y,
            p,
            sp,
            cycles,
        }
    }
    fn from_log(line: &String) -> Self {
        Self {
            pc: line[0..=3].to_string(),
            bytes: line[6..=13].trim_end().to_string(),
            instruction: line[16..=18].to_string(),
            a: line[50..=51].to_string(),
            x: line[55..=56].to_string(),
            y: line[60..=61].to_string(),
            p: line[65..=66].to_string(),
            sp: line[71..=72].to_string(),
            cycles: line[90..].trim_end().to_string(),
        }
    }
}

fn debug_step(console: &mut console::Console, log: String) {
    let expected = CpuState::from_log(&log);
    let actual = CpuState::from_cpu_view(console.cpu.view(), console.cpu_bus.cycles.to_string());
    assert!(
        expected == actual,
        "\n{}\n{}",
        format_trace(expected, "log"),
        format_trace(actual, "trace"),
    );
    console.cpu.step(&mut console.cpu_bus);
}

fn format_trace(state: CpuState, descriptor: &str) -> String {
    format!(
        "{}:
{}  {}  {}    A:{} X:{} Y:{} P:{} SP:{} CYC:{}
        ",
        descriptor,
        state.pc,
        state.bytes,
        state.instruction,
        state.a,
        state.x,
        state.y,
        state.p,
        state.sp,
        state.cycles,
    )
}

#[test]
fn test_rom() -> std::io::Result<()> {
    let mut rom = File::open("tests/roms/nestest.nes")?;
    let log_file = File::open("tests/logs/nestest.log")?;
    let log = BufReader::new(log_file);
    let mut console = console::Console::new(&mut rom).unwrap();
    console.cpu.pc = 0xC000;
    console.cpu.p = 0x24;
    console.cpu_bus.cycles = 7;
    console.cpu.instruction_history.push_front(
        console
            .cpu
            .disassemble_instruction(&mut console.cpu_bus, console.cpu.pc),
    );

    for line in log.lines() {
        let line = line?;
        debug_step(&mut console, line);
    }

    Ok(())
}
