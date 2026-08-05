use std::{fs::File, io::{BufRead, BufReader}};
use core::{console};

#[derive(PartialEq)]
struct CpuState {
    pc: String,
    opcode: String,
    operand: String,
    instruction: String,
    a: String,
    x: String,
    y: String,
    p: String,
    sp: String,
}

impl CpuState {
    fn from_trace(line: &String) -> Self {
        let mut instruction = line[16..=18].to_string();
        instruction = if instruction == String::from("ISC") {
            String::from("ISB")
        }
        else {
            instruction
        };

        Self {
            pc: line[0..=4].to_string(),
            opcode: line[6..=7].to_string(),
            operand: line[9..=13].to_string(),
            instruction: instruction,
            a: line[26..=27].to_string(),
            x: line[31..=32].to_string(),
            y: line[36..=37].to_string(),
            p: line[41..=42].to_string(),
            sp: line[47..=48].to_string(),
        }
    }
    fn from_log(line: &String) -> Self {
        Self {
            pc: line[0..=4].to_string(),
            opcode: line[6..=7].to_string(),
            operand: line[9..=13].to_string(),
            instruction: line[16..=18].to_string(),
            a: line[50..=51].to_string(),
            x: line[55..=56].to_string(),
            y: line[60..=61].to_string(),
            p: line[65..=66].to_string(),
            sp: line[71..=72].to_string(),
        }
    }
}

fn debug_step(console: &mut console::Console, log: String) {
    let trace = console.cpu.trace(&console.cpu_bus);
    let expected = CpuState::from_log(&log);
    let actual = CpuState::from_trace(&trace);
    assert!(expected == actual,
        "\n{}\n{}", format_trace(expected, "log"),
        format_trace(actual, "trace"),
        );
    console.cpu.step(&mut console.cpu_bus);
}

fn format_trace(state: CpuState, descriptor: &str) -> String {
    format!(
        "{}:
{}  {} {}  {}    A:{} X{} Y:{} P:{} SP:{}
        ",
        descriptor,
        state.pc,
        state.opcode,
        state.operand,
        state.instruction,
        state.a,
        state.x,
        state.y,
        state.p,
        state.sp,
        )
}

#[test]
fn test_rom() -> std::io::Result<()> {
    let mut rom = File::open("tests/roms/nestest.nes")?;
    let log_file = File::open("tests/logs/nestest.log")?;
    let log = BufReader::new(log_file);
    let mut console = console::Console::new(&mut rom);
    console.cpu.pc = 0xC000;
    console.cpu.p = 0x24;

    for line in log.lines() {
        let line = line?;
        debug_step(&mut console, line);
    }

    Ok(())
}
