use std::collections::VecDeque;

use crate::bus::{self, Bus, CpuBus};

pub static OPCODES: [Opcode; 256] = [
    Opcode {
        instruction: Instruction::BRK,
        mode: AddressingMode::Implicit,
        cycles: 7,
    }, // 0x00
    Opcode {
        instruction: Instruction::ORA,
        mode: AddressingMode::IndirectX,
        cycles: 6,
    }, // 0x01
    Opcode {
        instruction: Instruction::KIL,
        mode: AddressingMode::Implicit,
        cycles: 0,
    }, // 0x02
    Opcode {
        instruction: Instruction::SLO,
        mode: AddressingMode::IndirectX,
        cycles: 8,
    }, // 0x03
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::ZeroPage,
        cycles: 3,
    }, // 0x04
    Opcode {
        instruction: Instruction::ORA,
        mode: AddressingMode::ZeroPage,
        cycles: 3,
    }, // 0x05
    Opcode {
        instruction: Instruction::ASL,
        mode: AddressingMode::ZeroPage,
        cycles: 5,
    }, // 0x06
    Opcode {
        instruction: Instruction::SLO,
        mode: AddressingMode::ZeroPage,
        cycles: 5,
    }, // 0x07
    Opcode {
        instruction: Instruction::PHP,
        mode: AddressingMode::Implicit,
        cycles: 3,
    }, // 0x08
    Opcode {
        instruction: Instruction::ORA,
        mode: AddressingMode::Immediate,
        cycles: 2,
    }, // 0x09
    Opcode {
        instruction: Instruction::ASL,
        mode: AddressingMode::Accumulator,
        cycles: 2,
    }, // 0x0A
    Opcode {
        instruction: Instruction::ANC,
        mode: AddressingMode::Immediate,
        cycles: 2,
    }, // 0x0B
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::Absolute,
        cycles: 4,
    }, // 0x0C
    Opcode {
        instruction: Instruction::ORA,
        mode: AddressingMode::Absolute,
        cycles: 4,
    }, // 0x0D
    Opcode {
        instruction: Instruction::ASL,
        mode: AddressingMode::Absolute,
        cycles: 6,
    }, // 0x0E
    Opcode {
        instruction: Instruction::SLO,
        mode: AddressingMode::Absolute,
        cycles: 6,
    }, // 0x0F
    Opcode {
        instruction: Instruction::BPL,
        mode: AddressingMode::Relative,
        cycles: 2,
    }, // 0x10
    Opcode {
        instruction: Instruction::ORA,
        mode: AddressingMode::IndirectY,
        cycles: 5,
    }, // 0x11
    Opcode {
        instruction: Instruction::KIL,
        mode: AddressingMode::Implicit,
        cycles: 0,
    }, // 0x12
    Opcode {
        instruction: Instruction::SLO,
        mode: AddressingMode::IndirectY,
        cycles: 8,
    }, // 0x13
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
    }, // 0x14
    Opcode {
        instruction: Instruction::ORA,
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
    }, // 0x15
    Opcode {
        instruction: Instruction::ASL,
        mode: AddressingMode::ZeroPageX,
        cycles: 6,
    }, // 0x16
    Opcode {
        instruction: Instruction::SLO,
        mode: AddressingMode::ZeroPageX,
        cycles: 6,
    }, // 0x17
    Opcode {
        instruction: Instruction::CLC,
        mode: AddressingMode::Implicit,
        cycles: 2,
    }, // 0x18
    Opcode {
        instruction: Instruction::ORA,
        mode: AddressingMode::AbsoluteY,
        cycles: 4,
    }, // 0x19
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::Implicit,
        cycles: 2,
    }, // 0x1A
    Opcode {
        instruction: Instruction::SLO,
        mode: AddressingMode::AbsoluteY,
        cycles: 7,
    }, // 0x1B
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
    }, // 0x1C
    Opcode {
        instruction: Instruction::ORA,
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
    }, // 0x1D
    Opcode {
        instruction: Instruction::ASL,
        mode: AddressingMode::AbsoluteX,
        cycles: 7,
    }, // 0x1E
    Opcode {
        instruction: Instruction::SLO,
        mode: AddressingMode::AbsoluteX,
        cycles: 7,
    }, // 0x1F
    Opcode {
        instruction: Instruction::JSR,
        mode: AddressingMode::Absolute,
        cycles: 6,
    }, // 0x20
    Opcode {
        instruction: Instruction::AND,
        mode: AddressingMode::IndirectX,
        cycles: 6,
    }, // 0x21
    Opcode {
        instruction: Instruction::KIL,
        mode: AddressingMode::Implicit,
        cycles: 0,
    }, // 0x22
    Opcode {
        instruction: Instruction::RLA,
        mode: AddressingMode::IndirectX,
        cycles: 8,
    }, // 0x23
    Opcode {
        instruction: Instruction::BIT,
        mode: AddressingMode::ZeroPage,
        cycles: 3,
    }, // 0x24
    Opcode {
        instruction: Instruction::AND,
        mode: AddressingMode::ZeroPage,
        cycles: 3,
    }, // 0x25
    Opcode {
        instruction: Instruction::ROL,
        mode: AddressingMode::ZeroPage,
        cycles: 5,
    }, // 0x26
    Opcode {
        instruction: Instruction::RLA,
        mode: AddressingMode::ZeroPage,
        cycles: 5,
    }, // 0x27
    Opcode {
        instruction: Instruction::PLP,
        mode: AddressingMode::Implicit,
        cycles: 4,
    }, // 0x28
    Opcode {
        instruction: Instruction::AND,
        mode: AddressingMode::Immediate,
        cycles: 2,
    }, // 0x29
    Opcode {
        instruction: Instruction::ROL,
        mode: AddressingMode::Accumulator,
        cycles: 2,
    }, // 0x2A
    Opcode {
        instruction: Instruction::ANC,
        mode: AddressingMode::Immediate,
        cycles: 2,
    }, // 0x2B
    Opcode {
        instruction: Instruction::BIT,
        mode: AddressingMode::Absolute,
        cycles: 4,
    }, // 0x2C
    Opcode {
        instruction: Instruction::AND,
        mode: AddressingMode::Absolute,
        cycles: 4,
    }, // 0x2D
    Opcode {
        instruction: Instruction::ROL,
        mode: AddressingMode::Absolute,
        cycles: 6,
    }, // 0x2E
    Opcode {
        instruction: Instruction::RLA,
        mode: AddressingMode::Absolute,
        cycles: 6,
    }, // 0x2F
    Opcode {
        instruction: Instruction::BMI,
        mode: AddressingMode::Relative,
        cycles: 2,
    }, // 0x30
    Opcode {
        instruction: Instruction::AND,
        mode: AddressingMode::IndirectY,
        cycles: 5,
    }, // 0x31
    Opcode {
        instruction: Instruction::KIL,
        mode: AddressingMode::Implicit,
        cycles: 0,
    }, // 0x32
    Opcode {
        instruction: Instruction::RLA,
        mode: AddressingMode::IndirectY,
        cycles: 8,
    }, // 0x33
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
    }, // 0x34
    Opcode {
        instruction: Instruction::AND,
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
    }, // 0x35
    Opcode {
        instruction: Instruction::ROL,
        mode: AddressingMode::ZeroPageX,
        cycles: 6,
    }, // 0x36
    Opcode {
        instruction: Instruction::RLA,
        mode: AddressingMode::ZeroPageX,
        cycles: 6,
    }, // 0x37
    Opcode {
        instruction: Instruction::SEC,
        mode: AddressingMode::Implicit,
        cycles: 2,
    }, // 0x38
    Opcode {
        instruction: Instruction::AND,
        mode: AddressingMode::AbsoluteY,
        cycles: 4,
    }, // 0x39
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::Implicit,
        cycles: 2,
    }, // 0x3A
    Opcode {
        instruction: Instruction::RLA,
        mode: AddressingMode::AbsoluteY,
        cycles: 7,
    }, // 0x3B
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
    }, // 0x3C
    Opcode {
        instruction: Instruction::AND,
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
    }, // 0x3D
    Opcode {
        instruction: Instruction::ROL,
        mode: AddressingMode::AbsoluteX,
        cycles: 7,
    }, // 0x3E
    Opcode {
        instruction: Instruction::RLA,
        mode: AddressingMode::AbsoluteX,
        cycles: 7,
    }, // 0x3F
    Opcode {
        instruction: Instruction::RTI,
        mode: AddressingMode::Implicit,
        cycles: 6,
    }, // 0x40
    Opcode {
        instruction: Instruction::EOR,
        mode: AddressingMode::IndirectX,
        cycles: 6,
    }, // 0x41
    Opcode {
        instruction: Instruction::KIL,
        mode: AddressingMode::Implicit,
        cycles: 0,
    }, // 0x42
    Opcode {
        instruction: Instruction::SRE,
        mode: AddressingMode::IndirectX,
        cycles: 8,
    }, // 0x43
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::ZeroPage,
        cycles: 3,
    }, // 0x44
    Opcode {
        instruction: Instruction::EOR,
        mode: AddressingMode::ZeroPage,
        cycles: 3,
    }, // 0x45
    Opcode {
        instruction: Instruction::LSR,
        mode: AddressingMode::ZeroPage,
        cycles: 5,
    }, // 0x46
    Opcode {
        instruction: Instruction::SRE,
        mode: AddressingMode::ZeroPage,
        cycles: 5,
    }, // 0x47
    Opcode {
        instruction: Instruction::PHA,
        mode: AddressingMode::Implicit,
        cycles: 3,
    }, // 0x48
    Opcode {
        instruction: Instruction::EOR,
        mode: AddressingMode::Immediate,
        cycles: 2,
    }, // 0x49
    Opcode {
        instruction: Instruction::LSR,
        mode: AddressingMode::Accumulator,
        cycles: 2,
    }, // 0x4A
    Opcode {
        instruction: Instruction::ALR,
        mode: AddressingMode::Immediate,
        cycles: 2,
    }, // 0x4B
    Opcode {
        instruction: Instruction::JMP,
        mode: AddressingMode::Absolute,
        cycles: 3,
    }, // 0x4C
    Opcode {
        instruction: Instruction::EOR,
        mode: AddressingMode::Absolute,
        cycles: 4,
    }, // 0x4D
    Opcode {
        instruction: Instruction::LSR,
        mode: AddressingMode::Absolute,
        cycles: 6,
    }, // 0x4E
    Opcode {
        instruction: Instruction::SRE,
        mode: AddressingMode::Absolute,
        cycles: 6,
    }, // 0x4F
    Opcode {
        instruction: Instruction::BVC,
        mode: AddressingMode::Relative,
        cycles: 2,
    }, // 0x50
    Opcode {
        instruction: Instruction::EOR,
        mode: AddressingMode::IndirectY,
        cycles: 5,
    }, // 0x51
    Opcode {
        instruction: Instruction::KIL,
        mode: AddressingMode::Implicit,
        cycles: 0,
    }, // 0x52
    Opcode {
        instruction: Instruction::SRE,
        mode: AddressingMode::IndirectY,
        cycles: 8,
    }, // 0x53
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
    }, // 0x54
    Opcode {
        instruction: Instruction::EOR,
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
    }, // 0x55
    Opcode {
        instruction: Instruction::LSR,
        mode: AddressingMode::ZeroPageX,
        cycles: 6,
    }, // 0x56
    Opcode {
        instruction: Instruction::SRE,
        mode: AddressingMode::ZeroPageX,
        cycles: 6,
    }, // 0x57
    Opcode {
        instruction: Instruction::CLI,
        mode: AddressingMode::Implicit,
        cycles: 2,
    }, // 0x58
    Opcode {
        instruction: Instruction::EOR,
        mode: AddressingMode::AbsoluteY,
        cycles: 4,
    }, // 0x59
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::Implicit,
        cycles: 2,
    }, // 0x5A
    Opcode {
        instruction: Instruction::SRE,
        mode: AddressingMode::AbsoluteY,
        cycles: 7,
    }, // 0x5B
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
    }, // 0x5C
    Opcode {
        instruction: Instruction::EOR,
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
    }, // 0x5D
    Opcode {
        instruction: Instruction::LSR,
        mode: AddressingMode::AbsoluteX,
        cycles: 7,
    }, // 0x5E
    Opcode {
        instruction: Instruction::SRE,
        mode: AddressingMode::AbsoluteX,
        cycles: 7,
    }, // 0x5F
    Opcode {
        instruction: Instruction::RTS,
        mode: AddressingMode::Implicit,
        cycles: 6,
    }, // 0x60
    Opcode {
        instruction: Instruction::ADC,
        mode: AddressingMode::IndirectX,
        cycles: 6,
    }, // 0x61
    Opcode {
        instruction: Instruction::KIL,
        mode: AddressingMode::Implicit,
        cycles: 0,
    }, // 0x62
    Opcode {
        instruction: Instruction::RRA,
        mode: AddressingMode::IndirectX,
        cycles: 8,
    }, // 0x63
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::ZeroPage,
        cycles: 3,
    }, // 0x64
    Opcode {
        instruction: Instruction::ADC,
        mode: AddressingMode::ZeroPage,
        cycles: 3,
    }, // 0x65
    Opcode {
        instruction: Instruction::ROR,
        mode: AddressingMode::ZeroPage,
        cycles: 5,
    }, // 0x66
    Opcode {
        instruction: Instruction::RRA,
        mode: AddressingMode::ZeroPage,
        cycles: 5,
    }, // 0x67
    Opcode {
        instruction: Instruction::PLA,
        mode: AddressingMode::Implicit,
        cycles: 4,
    }, // 0x68
    Opcode {
        instruction: Instruction::ADC,
        mode: AddressingMode::Immediate,
        cycles: 2,
    }, // 0x69
    Opcode {
        instruction: Instruction::ROR,
        mode: AddressingMode::Accumulator,
        cycles: 2,
    }, // 0x6A
    Opcode {
        instruction: Instruction::ARR,
        mode: AddressingMode::Immediate,
        cycles: 2,
    }, // 0x6B
    Opcode {
        instruction: Instruction::JMP,
        mode: AddressingMode::Indirect,
        cycles: 5,
    }, // 0x6C
    Opcode {
        instruction: Instruction::ADC,
        mode: AddressingMode::Absolute,
        cycles: 4,
    }, // 0x6D
    Opcode {
        instruction: Instruction::ROR,
        mode: AddressingMode::Absolute,
        cycles: 6,
    }, // 0x6E
    Opcode {
        instruction: Instruction::RRA,
        mode: AddressingMode::Absolute,
        cycles: 6,
    }, // 0x6F
    Opcode {
        instruction: Instruction::BVS,
        mode: AddressingMode::Relative,
        cycles: 2,
    }, // 0x70
    Opcode {
        instruction: Instruction::ADC,
        mode: AddressingMode::IndirectY,
        cycles: 5,
    }, // 0x71
    Opcode {
        instruction: Instruction::KIL,
        mode: AddressingMode::Implicit,
        cycles: 0,
    }, // 0x72
    Opcode {
        instruction: Instruction::RRA,
        mode: AddressingMode::IndirectY,
        cycles: 8,
    }, // 0x73
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
    }, // 0x74
    Opcode {
        instruction: Instruction::ADC,
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
    }, // 0x75
    Opcode {
        instruction: Instruction::ROR,
        mode: AddressingMode::ZeroPageX,
        cycles: 6,
    }, // 0x76
    Opcode {
        instruction: Instruction::RRA,
        mode: AddressingMode::ZeroPageX,
        cycles: 6,
    }, // 0x77
    Opcode {
        instruction: Instruction::SEI,
        mode: AddressingMode::Implicit,
        cycles: 2,
    }, // 0x78
    Opcode {
        instruction: Instruction::ADC,
        mode: AddressingMode::AbsoluteY,
        cycles: 4,
    }, // 0x79
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::Implicit,
        cycles: 2,
    }, // 0x7A
    Opcode {
        instruction: Instruction::RRA,
        mode: AddressingMode::AbsoluteY,
        cycles: 7,
    }, // 0x7B
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
    }, // 0x7C
    Opcode {
        instruction: Instruction::ADC,
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
    }, // 0x7D
    Opcode {
        instruction: Instruction::ROR,
        mode: AddressingMode::AbsoluteX,
        cycles: 7,
    }, // 0x7E
    Opcode {
        instruction: Instruction::RRA,
        mode: AddressingMode::AbsoluteX,
        cycles: 7,
    }, // 0x7F
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::Immediate,
        cycles: 2,
    }, // 0x80
    Opcode {
        instruction: Instruction::STA,
        mode: AddressingMode::IndirectX,
        cycles: 6,
    }, // 0x81
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::Immediate,
        cycles: 2,
    }, // 0x82
    Opcode {
        instruction: Instruction::SAX,
        mode: AddressingMode::IndirectX,
        cycles: 6,
    }, // 0x83
    Opcode {
        instruction: Instruction::STY,
        mode: AddressingMode::ZeroPage,
        cycles: 3,
    }, // 0x84
    Opcode {
        instruction: Instruction::STA,
        mode: AddressingMode::ZeroPage,
        cycles: 3,
    }, // 0x85
    Opcode {
        instruction: Instruction::STX,
        mode: AddressingMode::ZeroPage,
        cycles: 3,
    }, // 0x86
    Opcode {
        instruction: Instruction::SAX,
        mode: AddressingMode::ZeroPage,
        cycles: 3,
    }, // 0x87
    Opcode {
        instruction: Instruction::DEY,
        mode: AddressingMode::Implicit,
        cycles: 2,
    }, // 0x88
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::Immediate,
        cycles: 2,
    }, // 0x89
    Opcode {
        instruction: Instruction::TXA,
        mode: AddressingMode::Implicit,
        cycles: 2,
    }, // 0x8A
    Opcode {
        instruction: Instruction::XAA,
        mode: AddressingMode::Immediate,
        cycles: 2,
    }, // 0x8B
    Opcode {
        instruction: Instruction::STY,
        mode: AddressingMode::Absolute,
        cycles: 4,
    }, // 0x8C
    Opcode {
        instruction: Instruction::STA,
        mode: AddressingMode::Absolute,
        cycles: 4,
    }, // 0x8D
    Opcode {
        instruction: Instruction::STX,
        mode: AddressingMode::Absolute,
        cycles: 4,
    }, // 0x8E
    Opcode {
        instruction: Instruction::SAX,
        mode: AddressingMode::Absolute,
        cycles: 4,
    }, // 0x8F
    Opcode {
        instruction: Instruction::BCC,
        mode: AddressingMode::Relative,
        cycles: 2,
    }, // 0x90
    Opcode {
        instruction: Instruction::STA,
        mode: AddressingMode::IndirectY,
        cycles: 6,
    }, // 0x91
    Opcode {
        instruction: Instruction::KIL,
        mode: AddressingMode::Implicit,
        cycles: 0,
    }, // 0x92
    Opcode {
        instruction: Instruction::AHX,
        mode: AddressingMode::IndirectY,
        cycles: 6,
    }, // 0x93
    Opcode {
        instruction: Instruction::STY,
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
    }, // 0x94
    Opcode {
        instruction: Instruction::STA,
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
    }, // 0x95
    Opcode {
        instruction: Instruction::STX,
        mode: AddressingMode::ZeroPageY,
        cycles: 4,
    }, // 0x96
    Opcode {
        instruction: Instruction::SAX,
        mode: AddressingMode::ZeroPageY,
        cycles: 4,
    }, // 0x97
    Opcode {
        instruction: Instruction::TYA,
        mode: AddressingMode::Implicit,
        cycles: 2,
    }, // 0x98
    Opcode {
        instruction: Instruction::STA,
        mode: AddressingMode::AbsoluteY,
        cycles: 5,
    }, // 0x99
    Opcode {
        instruction: Instruction::TXS,
        mode: AddressingMode::Implicit,
        cycles: 2,
    }, // 0x9A
    Opcode {
        instruction: Instruction::TAS,
        mode: AddressingMode::AbsoluteY,
        cycles: 5,
    }, // 0x9B
    Opcode {
        instruction: Instruction::SHY,
        mode: AddressingMode::AbsoluteX,
        cycles: 5,
    }, // 0x9C
    Opcode {
        instruction: Instruction::STA,
        mode: AddressingMode::AbsoluteX,
        cycles: 5,
    }, // 0x9D
    Opcode {
        instruction: Instruction::SHX,
        mode: AddressingMode::AbsoluteY,
        cycles: 5,
    }, // 0x9E
    Opcode {
        instruction: Instruction::AHX,
        mode: AddressingMode::AbsoluteY,
        cycles: 5,
    }, // 0x9F
    Opcode {
        instruction: Instruction::LDY,
        mode: AddressingMode::Immediate,
        cycles: 2,
    }, // 0xA0
    Opcode {
        instruction: Instruction::LDA,
        mode: AddressingMode::IndirectX,
        cycles: 6,
    }, // 0xA1
    Opcode {
        instruction: Instruction::LDX,
        mode: AddressingMode::Immediate,
        cycles: 2,
    }, // 0xA2
    Opcode {
        instruction: Instruction::LAX,
        mode: AddressingMode::IndirectX,
        cycles: 6,
    }, // 0xA3
    Opcode {
        instruction: Instruction::LDY,
        mode: AddressingMode::ZeroPage,
        cycles: 3,
    }, // 0xA4
    Opcode {
        instruction: Instruction::LDA,
        mode: AddressingMode::ZeroPage,
        cycles: 3,
    }, // 0xA5
    Opcode {
        instruction: Instruction::LDX,
        mode: AddressingMode::ZeroPage,
        cycles: 3,
    }, // 0xA6
    Opcode {
        instruction: Instruction::LAX,
        mode: AddressingMode::ZeroPage,
        cycles: 3,
    }, // 0xA7
    Opcode {
        instruction: Instruction::TAY,
        mode: AddressingMode::Implicit,
        cycles: 2,
    }, // 0xA8
    Opcode {
        instruction: Instruction::LDA,
        mode: AddressingMode::Immediate,
        cycles: 2,
    }, // 0xA9
    Opcode {
        instruction: Instruction::TAX,
        mode: AddressingMode::Implicit,
        cycles: 2,
    }, // 0xAA
    Opcode {
        instruction: Instruction::LAX,
        mode: AddressingMode::Immediate,
        cycles: 2,
    }, // 0xAB
    Opcode {
        instruction: Instruction::LDY,
        mode: AddressingMode::Absolute,
        cycles: 4,
    }, // 0xAC
    Opcode {
        instruction: Instruction::LDA,
        mode: AddressingMode::Absolute,
        cycles: 4,
    }, // 0xAD
    Opcode {
        instruction: Instruction::LDX,
        mode: AddressingMode::Absolute,
        cycles: 4,
    }, // 0xAE
    Opcode {
        instruction: Instruction::LAX,
        mode: AddressingMode::Absolute,
        cycles: 4,
    }, // 0xAF
    Opcode {
        instruction: Instruction::BCS,
        mode: AddressingMode::Relative,
        cycles: 2,
    }, // 0xB0
    Opcode {
        instruction: Instruction::LDA,
        mode: AddressingMode::IndirectY,
        cycles: 5,
    }, // 0xB1
    Opcode {
        instruction: Instruction::KIL,
        mode: AddressingMode::Implicit,
        cycles: 0,
    }, // 0xB2
    Opcode {
        instruction: Instruction::LAX,
        mode: AddressingMode::IndirectY,
        cycles: 5,
    }, // 0xB3
    Opcode {
        instruction: Instruction::LDY,
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
    }, // 0xB4
    Opcode {
        instruction: Instruction::LDA,
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
    }, // 0xB5
    Opcode {
        instruction: Instruction::LDX,
        mode: AddressingMode::ZeroPageY,
        cycles: 4,
    }, // 0xB6
    Opcode {
        instruction: Instruction::LAX,
        mode: AddressingMode::ZeroPageY,
        cycles: 4,
    }, // 0xB7
    Opcode {
        instruction: Instruction::CLV,
        mode: AddressingMode::Implicit,
        cycles: 2,
    }, // 0xB8
    Opcode {
        instruction: Instruction::LDA,
        mode: AddressingMode::AbsoluteY,
        cycles: 4,
    }, // 0xB9
    Opcode {
        instruction: Instruction::TSX,
        mode: AddressingMode::Implicit,
        cycles: 2,
    }, // 0xBA
    Opcode {
        instruction: Instruction::LAS,
        mode: AddressingMode::AbsoluteY,
        cycles: 4,
    }, // 0xBB
    Opcode {
        instruction: Instruction::LDY,
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
    }, // 0xBC
    Opcode {
        instruction: Instruction::LDA,
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
    }, // 0xBD
    Opcode {
        instruction: Instruction::LDX,
        mode: AddressingMode::AbsoluteY,
        cycles: 4,
    }, // 0xBE
    Opcode {
        instruction: Instruction::LAX,
        mode: AddressingMode::AbsoluteY,
        cycles: 4,
    }, // 0xBF
    Opcode {
        instruction: Instruction::CPY,
        mode: AddressingMode::Immediate,
        cycles: 2,
    }, // 0xC0
    Opcode {
        instruction: Instruction::CMP,
        mode: AddressingMode::IndirectX,
        cycles: 6,
    }, // 0xC1
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::Immediate,
        cycles: 2,
    }, // 0xC2
    Opcode {
        instruction: Instruction::DCP,
        mode: AddressingMode::IndirectX,
        cycles: 8,
    }, // 0xC3
    Opcode {
        instruction: Instruction::CPY,
        mode: AddressingMode::ZeroPage,
        cycles: 3,
    }, // 0xC4
    Opcode {
        instruction: Instruction::CMP,
        mode: AddressingMode::ZeroPage,
        cycles: 3,
    }, // 0xC5
    Opcode {
        instruction: Instruction::DEC,
        mode: AddressingMode::ZeroPage,
        cycles: 5,
    }, // 0xC6
    Opcode {
        instruction: Instruction::DCP,
        mode: AddressingMode::ZeroPage,
        cycles: 5,
    }, // 0xC7
    Opcode {
        instruction: Instruction::INY,
        mode: AddressingMode::Implicit,
        cycles: 2,
    }, // 0xC8
    Opcode {
        instruction: Instruction::CMP,
        mode: AddressingMode::Immediate,
        cycles: 2,
    }, // 0xC9
    Opcode {
        instruction: Instruction::DEX,
        mode: AddressingMode::Implicit,
        cycles: 2,
    }, // 0xCA
    Opcode {
        instruction: Instruction::AXS,
        mode: AddressingMode::Immediate,
        cycles: 2,
    }, // 0xCB
    Opcode {
        instruction: Instruction::CPY,
        mode: AddressingMode::Absolute,
        cycles: 4,
    }, // 0xCC
    Opcode {
        instruction: Instruction::CMP,
        mode: AddressingMode::Absolute,
        cycles: 4,
    }, // 0xCD
    Opcode {
        instruction: Instruction::DEC,
        mode: AddressingMode::Absolute,
        cycles: 6,
    }, // 0xCE
    Opcode {
        instruction: Instruction::DCP,
        mode: AddressingMode::Absolute,
        cycles: 6,
    }, // 0xCF
    Opcode {
        instruction: Instruction::BNE,
        mode: AddressingMode::Relative,
        cycles: 2,
    }, // 0xD0
    Opcode {
        instruction: Instruction::CMP,
        mode: AddressingMode::IndirectY,
        cycles: 5,
    }, // 0xD1
    Opcode {
        instruction: Instruction::KIL,
        mode: AddressingMode::Implicit,
        cycles: 0,
    }, // 0xD2
    Opcode {
        instruction: Instruction::DCP,
        mode: AddressingMode::IndirectY,
        cycles: 8,
    }, // 0xD3
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
    }, // 0xD4
    Opcode {
        instruction: Instruction::CMP,
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
    }, // 0xD5
    Opcode {
        instruction: Instruction::DEC,
        mode: AddressingMode::ZeroPageX,
        cycles: 6,
    }, // 0xD6
    Opcode {
        instruction: Instruction::DCP,
        mode: AddressingMode::ZeroPageX,
        cycles: 6,
    }, // 0xD7
    Opcode {
        instruction: Instruction::CLD,
        mode: AddressingMode::Implicit,
        cycles: 2,
    }, // 0xD8
    Opcode {
        instruction: Instruction::CMP,
        mode: AddressingMode::AbsoluteY,
        cycles: 4,
    }, // 0xD9
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::Implicit,
        cycles: 2,
    }, // 0xDA
    Opcode {
        instruction: Instruction::DCP,
        mode: AddressingMode::AbsoluteY,
        cycles: 7,
    }, // 0xDB
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
    }, // 0xDC
    Opcode {
        instruction: Instruction::CMP,
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
    }, // 0xDD
    Opcode {
        instruction: Instruction::DEC,
        mode: AddressingMode::AbsoluteX,
        cycles: 7,
    }, // 0xDE
    Opcode {
        instruction: Instruction::DCP,
        mode: AddressingMode::AbsoluteX,
        cycles: 7,
    }, // 0xDF
    Opcode {
        instruction: Instruction::CPX,
        mode: AddressingMode::Immediate,
        cycles: 2,
    }, // 0xE0
    Opcode {
        instruction: Instruction::SBC,
        mode: AddressingMode::IndirectX,
        cycles: 6,
    }, // 0xE1
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::Immediate,
        cycles: 2,
    }, // 0xE2
    Opcode {
        instruction: Instruction::ISC,
        mode: AddressingMode::IndirectX,
        cycles: 8,
    }, // 0xE3
    Opcode {
        instruction: Instruction::CPX,
        mode: AddressingMode::ZeroPage,
        cycles: 3,
    }, // 0xE4
    Opcode {
        instruction: Instruction::SBC,
        mode: AddressingMode::ZeroPage,
        cycles: 3,
    }, // 0xE5
    Opcode {
        instruction: Instruction::INC,
        mode: AddressingMode::ZeroPage,
        cycles: 5,
    }, // 0xE6
    Opcode {
        instruction: Instruction::ISC,
        mode: AddressingMode::ZeroPage,
        cycles: 5,
    }, // 0xE7
    Opcode {
        instruction: Instruction::INX,
        mode: AddressingMode::Implicit,
        cycles: 2,
    }, // 0xE8
    Opcode {
        instruction: Instruction::SBC,
        mode: AddressingMode::Immediate,
        cycles: 2,
    }, // 0xE9
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::Implicit,
        cycles: 2,
    }, // 0xEA
    Opcode {
        instruction: Instruction::SBC,
        mode: AddressingMode::Immediate,
        cycles: 2,
    }, // 0xEB
    Opcode {
        instruction: Instruction::CPX,
        mode: AddressingMode::Absolute,
        cycles: 4,
    }, // 0xEC
    Opcode {
        instruction: Instruction::SBC,
        mode: AddressingMode::Absolute,
        cycles: 4,
    }, // 0xED
    Opcode {
        instruction: Instruction::INC,
        mode: AddressingMode::Absolute,
        cycles: 6,
    }, // 0xEE
    Opcode {
        instruction: Instruction::ISC,
        mode: AddressingMode::Absolute,
        cycles: 6,
    }, // 0xEF
    Opcode {
        instruction: Instruction::BEQ,
        mode: AddressingMode::Relative,
        cycles: 2,
    }, // 0xF0
    Opcode {
        instruction: Instruction::SBC,
        mode: AddressingMode::IndirectY,
        cycles: 5,
    }, // 0xF1
    Opcode {
        instruction: Instruction::KIL,
        mode: AddressingMode::Implicit,
        cycles: 0,
    }, // 0xF2
    Opcode {
        instruction: Instruction::ISC,
        mode: AddressingMode::IndirectY,
        cycles: 8,
    }, // 0xF3
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
    }, // 0xF4
    Opcode {
        instruction: Instruction::SBC,
        mode: AddressingMode::ZeroPageX,
        cycles: 4,
    }, // 0xF5
    Opcode {
        instruction: Instruction::INC,
        mode: AddressingMode::ZeroPageX,
        cycles: 6,
    }, // 0xF6
    Opcode {
        instruction: Instruction::ISC,
        mode: AddressingMode::ZeroPageX,
        cycles: 6,
    }, // 0xF7
    Opcode {
        instruction: Instruction::SED,
        mode: AddressingMode::Implicit,
        cycles: 2,
    }, // 0xF8
    Opcode {
        instruction: Instruction::SBC,
        mode: AddressingMode::AbsoluteY,
        cycles: 4,
    }, // 0xF9
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::Implicit,
        cycles: 2,
    }, // 0xFA
    Opcode {
        instruction: Instruction::ISC,
        mode: AddressingMode::AbsoluteY,
        cycles: 7,
    }, // 0xFB
    Opcode {
        instruction: Instruction::NOP,
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
    }, // 0xFC
    Opcode {
        instruction: Instruction::SBC,
        mode: AddressingMode::AbsoluteX,
        cycles: 4,
    }, // 0xFD
    Opcode {
        instruction: Instruction::INC,
        mode: AddressingMode::AbsoluteX,
        cycles: 7,
    }, // 0xFE
    Opcode {
        instruction: Instruction::ISC,
        mode: AddressingMode::AbsoluteX,
        cycles: 7,
    }, // 0xFF
];

#[derive(Debug, Clone, Copy)]
enum Instruction {
    LDA,
    STA,
    LDX,
    STX,
    LDY,
    STY,
    TAX,
    TXA,
    TAY,
    TYA,
    ADC,
    SBC,
    INC,
    DEC,
    INX,
    DEX,
    INY,
    DEY,
    ASL,
    LSR,
    ROL,
    ROR,
    AND,
    ORA,
    EOR,
    BIT,
    CMP,
    CPX,
    CPY,
    BCC,
    BCS,
    BEQ,
    BNE,
    BPL,
    BMI,
    BVC,
    BVS,
    JMP,
    JSR,
    RTS,
    BRK,
    RTI,
    PHA,
    PLA,
    PHP,
    PLP,
    TXS,
    TSX,
    CLC,
    SEC,
    CLI,
    SEI,
    CLD,
    SED,
    CLV,
    NOP,
    KIL,
    SLO,
    ANC,
    RLA,
    SRE,
    ALR,
    RRA,
    ARR,
    SAX,
    XAA,
    AHX,
    TAS,
    SHY,
    SHX,
    LAX,
    LAS,
    DCP,
    AXS,
    ISC,
}

// flags
const CARRY: u8 = 1 << 0;
const ZERO: u8 = 1 << 1;
const INTERRUPT: u8 = 1 << 2;
const DECIMAL: u8 = 1 << 3;
const BREAK: u8 = 1 << 4;
const UNUSED: u8 = 1 << 5;
const OVERFLOW: u8 = 1 << 6;
const NEGATIVE: u8 = 1 << 7;

#[derive(Debug, Clone, Copy)]
pub enum AddressingMode {
    ZeroPageX,
    ZeroPageY,
    ZeroPage,
    AbsoluteX,
    AbsoluteY,
    Absolute,
    IndirectX,
    IndirectY,
    Indirect,
    Implicit,
    Accumulator,
    Immediate,
    Relative,
}

#[derive(Clone, Copy)]
pub enum Operand {
    Address(u16),
    Relative(i8),
    Accumulator,
    Implied,
}

pub struct Cpu {
    a: u8,
    x: u8,
    y: u8,
    pub pc: u16,
    sp: u8,
    pub p: u8,
    pub instruction_history: VecDeque<InstructionView>,
}
pub struct CpuView {
    pub registers: RegisterView,
    pub flags: FlagView,
    pub instructions: VecDeque<InstructionView>,
}
pub struct RegisterView {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u8,
    pub p: u8,
}
pub struct FlagView {
    pub negative: bool,
    pub overflow: bool,
    pub decimal: bool,
    pub interrupt_disable: bool,
    pub zero: bool,
    pub carry: bool,
}
#[derive(Clone)]
pub struct InstructionView {
    pub address: u16,
    pub bytes: Vec<u8>,
    pub mnemonic: String,
    pub operand: Option<String>,
    pub cycles: Option<u8>,
}

#[derive(Debug, Clone, Copy)]
pub struct Opcode {
    instruction: Instruction,
    mode: AddressingMode,
    cycles: u8,
}

impl CpuView {
    pub fn update<B: bus::Bus>(&mut self, cpu: &mut Cpu, bus: &B) -> () {
        self.registers = RegisterView::new(cpu);
        self.flags = FlagView::new(cpu);
        self.instructions = cpu.instruction_history.clone().into();
    }
}
impl RegisterView {
    fn new(cpu: &Cpu) -> Self {
        Self {
            a: cpu.a,
            x: cpu.x,
            y: cpu.y,
            pc: cpu.pc,
            sp: cpu.sp,
            p: cpu.p,
        }
    }
}
impl FlagView {
    fn new(cpu: &Cpu) -> Self {
        Self {
            negative: cpu.negative(),
            overflow: cpu.overflow(),
            decimal: cpu.decimal(),
            interrupt_disable: cpu.interrupt(),
            zero: cpu.zero(),
            carry: cpu.carry(),
        }
    }
}
impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            pc: 0x0000,
            sp: 0xFD,
            p: 0b0000_0100,
            instruction_history: VecDeque::new(),
        }
    }

    pub fn view(&mut self) -> CpuView {
        let registers = RegisterView::new(&self);
        let flags = FlagView::new(&self);
        let instructions: VecDeque<InstructionView> = self.instruction_history.clone().into();

        CpuView {
            registers,
            flags,
            instructions,
        }
    }
    fn update_instruction_history<B: bus::Bus>(&mut self, bus: &mut B, cycles: u8) {
        if let Some(instruction) = self.instruction_history.back_mut() {
            instruction.cycles = Some(cycles);
        };
        self.instruction_history
            .push_back(self.disassemble_instruction(bus, self.pc));
        if self.instruction_history.len() > 5000 {
            self.instruction_history.pop_front();
        };
    }
    pub fn disassemble_instruction<B: bus::Bus>(
        &self,
        bus: &mut B,
        address: u16,
    ) -> InstructionView {
        let opcode_byte = bus.immutable_read(address);
        let opcode = OPCODES[opcode_byte as usize];
        let mnemonic = format!("{:?}", opcode.instruction);
        let operand_byte = bus.immutable_read(address.wrapping_add(1));
        let operand_byte_2 = bus.immutable_read(address.wrapping_add(2));
        let operand_bytes = bus.immutable_read_u16(address.wrapping_add(1));
        let mut bytes = Vec::new();
        bytes.push(opcode_byte);
        let operand = match opcode.mode {
            AddressingMode::Implicit => None,
            AddressingMode::Accumulator => Some(format!("A")),
            AddressingMode::Immediate => {
                bytes.push(operand_byte);
                Some(format!("#${:02X}", operand_byte))
            }
            AddressingMode::ZeroPage => {
                bytes.push(operand_byte);
                Some(format!("${:02X}", operand_byte))
            }
            AddressingMode::ZeroPageX => {
                bytes.push(operand_byte);
                Some(format!("${:02X}, X", operand_byte))
            }
            AddressingMode::ZeroPageY => {
                bytes.push(operand_byte);
                Some(format!("${:02X}, Y", operand_byte))
            }
            AddressingMode::Absolute => {
                bytes.push(operand_byte);
                bytes.push(operand_byte_2);
                Some(format!("${:04X}", operand_bytes))
            }
            AddressingMode::AbsoluteX => {
                bytes.push(operand_byte);
                bytes.push(operand_byte_2);
                Some(format!("${:04X}, X", operand_bytes))
            }
            AddressingMode::AbsoluteY => {
                bytes.push(operand_byte);
                bytes.push(operand_byte_2);
                Some(format!("${:04X}, Y", operand_bytes))
            }
            AddressingMode::Indirect => {
                bytes.push(operand_byte);
                bytes.push(operand_byte_2);
                Some(format!("(${:04X})", operand_bytes))
            }
            AddressingMode::IndirectX => {
                bytes.push(operand_byte);
                Some(format!("(${:02X}, X)", operand_byte))
            }
            AddressingMode::IndirectY => {
                bytes.push(operand_byte);
                Some(format!("(${:02X}), Y", operand_byte))
            }
            AddressingMode::Relative => {
                bytes.push(operand_byte);
                Some(format!("${:02X}", operand_byte))
            }
        };

        InstructionView {
            address: self.pc,
            bytes,
            mnemonic,
            operand,
            cycles: None,
        }
    }
    pub fn nmi_detected<B: bus::Bus>(&self, bus: &mut B) -> bool {
        let nmi = bus.immutable_read(0x2000) & (1 << 7) != 0;
        let vblank = bus.immutable_read(0x2002) & (1 << 7) != 0;
        nmi && vblank
    }
    pub fn handle_nmi(&mut self, bus: &mut CpuBus) {
        let cycles = bus.cycles;
        self.push_pc(bus);
        self.push_flags(bus);
        self.pc = bus.read_u16(0xFFFA);
        let cycle_diff = bus.cycles - cycles;
        self.update_instruction_history(bus, cycle_diff as u8);
    }

    pub fn reset(&mut self, bus: &mut CpuBus) {
        let cycles = bus.cycles;
        self.pc = bus.read_u16(0xFFFC);
        self.sp = self.sp.wrapping_sub(3);
        let cycle_diff = bus.cycles - cycles;
        self.update_instruction_history(bus, cycle_diff as u8);
    }
    pub fn step(&mut self, bus: &mut CpuBus) {
        let cycles = bus.cycles;
        let opcode = self.fetch(bus);
        self.execute(bus, opcode);
        let cycle_diff = bus.cycles - cycles;
        self.update_instruction_history(bus, cycle_diff as u8);
        if self.nmi_detected(bus) {
            self.handle_nmi(bus);
        }
    }
    pub fn fetch<B: Bus>(&mut self, bus: &mut B) -> Opcode {
        let opcode = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        OPCODES[opcode as usize]
    }
    pub fn decode<B: bus::Bus>(&mut self, bus: &mut B, opcode: Opcode) -> Operand {
        match opcode.mode {
            AddressingMode::Implicit => self.implicit(bus),
            AddressingMode::Accumulator => self.accumulator(bus),
            AddressingMode::Immediate => self.immediate(),
            AddressingMode::ZeroPage => self.zero_page(bus),
            AddressingMode::ZeroPageX => self.zero_page_x(bus),
            AddressingMode::ZeroPageY => self.zero_page_y(bus),
            AddressingMode::Relative => self.relative(bus),
            AddressingMode::Absolute => {
                if let Instruction::JSR = opcode.instruction {
                    self.absolute_jsr(bus)
                } else {
                    self.absolute(bus)
                }
            }
            AddressingMode::AbsoluteX => {
                if let Instruction::STA
                | Instruction::LSR
                | Instruction::ASL
                | Instruction::ROR
                | Instruction::ROL
                | Instruction::DEC
                | Instruction::INC = opcode.instruction
                {
                    self.absolute_x_store(bus)
                } else {
                    self.absolute_x_internal(bus)
                }
            }
            AddressingMode::AbsoluteY => {
                if let Instruction::STA = opcode.instruction {
                    self.absolute_y_store(bus)
                } else {
                    self.absolute_y_internal(bus)
                }
            }
            AddressingMode::Indirect => {
                if let Instruction::JMP = opcode.instruction {
                    self.indirect_bug(bus)
                } else {
                    self.indirect(bus)
                }
            }
            AddressingMode::IndirectX => self.indirect_x(bus),
            AddressingMode::IndirectY => {
                if let Instruction::STA = opcode.instruction {
                    self.indirect_y_store(bus)
                } else {
                    self.indirect_y_internal(bus)
                }
            }
        }
    }
    pub fn execute<B: bus::Bus>(&mut self, bus: &mut B, opcode: Opcode) {
        let operand = self.decode(bus, opcode);
        match opcode.instruction {
            Instruction::LDA => {
                self.lda(bus, operand);
            }
            Instruction::STA => {
                self.sta(bus, operand);
            }
            Instruction::LDX => {
                self.ldx(bus, operand);
            }
            Instruction::STX => {
                self.stx(bus, operand);
            }
            Instruction::LDY => {
                self.ldy(bus, operand);
            }
            Instruction::STY => {
                self.sty(bus, operand);
            }
            Instruction::TAX => {
                self.tax();
            }
            Instruction::TXA => {
                self.txa();
            }
            Instruction::TAY => {
                self.tay();
            }
            Instruction::TYA => {
                self.tya();
            }
            Instruction::ADC => {
                self.adc(bus, operand);
            }
            Instruction::SBC => {
                self.sbc(bus, operand);
            }
            Instruction::INC => {
                self.inc(bus, operand);
            }
            Instruction::DEC => {
                self.dec(bus, operand);
            }
            Instruction::INX => {
                self.inx();
            }
            Instruction::DEX => {
                self.dex();
            }
            Instruction::INY => {
                self.iny();
            }
            Instruction::DEY => {
                self.dey();
            }
            Instruction::ASL => {
                self.asl(bus, operand);
            }
            Instruction::LSR => {
                self.lsr(bus, operand);
            }
            Instruction::ROL => {
                self.rol(bus, operand);
            }
            Instruction::ROR => {
                self.ror(bus, operand);
            }
            Instruction::AND => {
                self.and(bus, operand);
            }
            Instruction::ORA => {
                self.ora(bus, operand);
            }
            Instruction::EOR => {
                self.eor(bus, operand);
            }
            Instruction::BIT => {
                self.bit(bus, operand);
            }
            Instruction::CMP => {
                self.cmp(bus, operand);
            }
            Instruction::CPX => {
                self.cpx(bus, operand);
            }
            Instruction::CPY => {
                self.cpy(bus, operand);
            }
            Instruction::BCC => {
                self.bcc(bus, operand);
            }
            Instruction::BCS => {
                self.bcs(bus, operand);
            }
            Instruction::BEQ => {
                self.beq(bus, operand);
            }
            Instruction::BNE => {
                self.bne(bus, operand);
            }
            Instruction::BPL => {
                self.bpl(bus, operand);
            }
            Instruction::BMI => {
                self.bmi(bus, operand);
            }
            Instruction::BVC => {
                self.bvc(bus, operand);
            }
            Instruction::BVS => {
                self.bvs(bus, operand);
            }
            Instruction::JMP => {
                self.jmp(operand);
            }
            Instruction::JSR => {
                self.jsr(bus);
            }
            Instruction::RTS => {
                self.rts(bus);
            }
            Instruction::BRK => {
                self.brk(bus);
            }
            Instruction::RTI => {
                self.rti(bus);
            }
            Instruction::PHA => {
                self.pha(bus);
            }
            Instruction::PLA => {
                self.pla(bus);
            }
            Instruction::PHP => {
                self.php(bus);
            }
            Instruction::PLP => {
                self.plp(bus);
            }
            Instruction::TXS => {
                self.txs();
            }
            Instruction::TSX => {
                self.tsx();
            }
            Instruction::CLC => {
                self.clc();
            }
            Instruction::SEC => {
                self.sec();
            }
            Instruction::CLI => {
                self.cli();
            }
            Instruction::SEI => {
                self.sei();
            }
            Instruction::CLD => {
                self.cld();
            }
            Instruction::SED => {
                self.sed();
            }
            Instruction::CLV => {
                self.clv();
            }
            Instruction::NOP => {
                if !matches!(opcode.mode, AddressingMode::Implicit) {
                    self.nop(bus, operand);
                }
            }
            Instruction::LAX => {
                self.lda(bus, operand);
                self.tax();
            }
            Instruction::SAX => {
                self.sax(bus, operand);
            }
            Instruction::DCP => {
                self.dcp(bus, operand);
            }
            Instruction::ISC => {
                self.isc(bus, operand);
            }
            Instruction::SLO => {
                self.slo(bus, operand);
            }
            Instruction::RLA => {
                self.rla(bus, operand);
            }
            Instruction::SRE => self.sre(bus, operand),
            Instruction::RRA => self.rra(bus, operand),
            _ => {}
        }
    }
}

// addressing modes
impl Cpu {
    pub fn implicit<B: bus::Bus>(&self, bus: &mut B) -> Operand {
        bus.read(self.pc.wrapping_add(1));
        Operand::Implied
    }
    pub fn accumulator(&self, bus: &mut impl Bus) -> Operand {
        bus.read(self.pc.wrapping_add(1));
        Operand::Accumulator
    }
    pub fn immediate(&mut self) -> Operand {
        let address = self.pc;
        self.pc = self.pc.wrapping_add(1);
        Operand::Address(address)
    }
    pub fn zero_page<B: bus::Bus>(&mut self, bus: &mut B) -> Operand {
        let address = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        Operand::Address(u16::from(address))
    }
    pub fn zero_page_x<B: bus::Bus>(&mut self, bus: &mut B) -> Operand {
        let base = bus.read(self.pc);
        bus.read(base as u16);
        let address = base.wrapping_add(self.x);
        self.pc = self.pc.wrapping_add(1);
        Operand::Address(u16::from(address))
    }
    pub fn zero_page_y<B: bus::Bus>(&mut self, bus: &mut B) -> Operand {
        let base = bus.read(self.pc);
        bus.read(base as u16);
        let address = base.wrapping_add(self.y);
        self.pc = self.pc.wrapping_add(1);
        Operand::Address(u16::from(address))
    }
    pub fn relative<B: bus::Bus>(&mut self, bus: &mut B) -> Operand {
        let base = bus.read(self.pc) as i8;
        self.pc = self.pc.wrapping_add(1);
        Operand::Relative(base)
    }
    pub fn absolute<B: bus::Bus>(&mut self, bus: &mut B) -> Operand {
        let base = self.pc;
        let address = bus.read_u16(base);
        self.pc = self.pc.wrapping_add(2);
        Operand::Address(address)
    }
    pub fn absolute_x_store<B: bus::Bus>(&mut self, bus: &mut B) -> Operand {
        let base = bus.read_u16(self.pc);
        let address = base.wrapping_add(self.x as u16);
        bus.read(address);
        self.pc = self.pc.wrapping_add(2);
        Operand::Address(address)
    }
    fn absolute_x_internal(&mut self, bus: &mut impl Bus) -> Operand {
        let base = bus.read_u16(self.pc);
        let address = base.wrapping_add(self.x as u16);
        self.page_crossed(bus, base, address);
        self.pc = self.pc.wrapping_add(2);
        Operand::Address(address)
    }
    pub fn absolute_y_internal<B: bus::Bus>(&mut self, bus: &mut B) -> Operand {
        let base = bus.read_u16(self.pc);
        let address = base.wrapping_add(self.y as u16);
        self.page_crossed(bus, base, address);
        self.pc = self.pc.wrapping_add(2);
        Operand::Address(address)
    }
    fn absolute_y_store(&mut self, bus: &mut impl Bus) -> Operand {
        let base = bus.read_u16(self.pc);
        let sum = base.wrapping_add(self.y as u16);
        bus.read(sum);
        self.pc = self.pc.wrapping_add(2);
        Operand::Address(sum)
    }
    fn absolute_jsr(&self, bus: &impl Bus) -> Operand {
        let address = bus.immutable_read_u16(self.pc);
        Operand::Address(address)
    }
    pub fn indirect<B: bus::Bus>(&mut self, bus: &mut B) -> Operand {
        let base = bus.read_u16(self.pc);
        let address = bus.read_u16(base);
        self.pc = self.pc.wrapping_add(2);
        Operand::Address(address)
    }
    pub fn indirect_bug<B: bus::Bus>(&mut self, bus: &mut B) -> Operand {
        let base = bus.read_u16(self.pc);
        let address = bus.read_u16_bug(base);
        self.pc = self.pc.wrapping_add(2);
        Operand::Address(address)
    }
    pub fn indirect_x<B: bus::Bus>(&mut self, bus: &mut B) -> Operand {
        let base = bus.read(self.pc);
        bus.read(base as u16);
        let sum = base.wrapping_add(self.x);
        let address = bus.read_u16_zp(sum);
        self.pc = self.pc.wrapping_add(1);
        Operand::Address(address)
    }
    pub fn indirect_y_internal<B: bus::Bus>(&mut self, bus: &mut B) -> Operand {
        let indirect_low = bus.read(self.pc);
        let base = bus.read_u16_zp(indirect_low);
        let sum = base.wrapping_add(u16::from(self.y));
        self.page_crossed(bus, base, sum);
        self.pc = self.pc.wrapping_add(1);
        Operand::Address(sum)
    }
    fn indirect_y_store(&mut self, bus: &mut impl Bus) -> Operand {
        let indirect_low = bus.read(self.pc);
        let base = bus.read_u16_zp(indirect_low);
        let sum = base.wrapping_add(u16::from(self.y));
        bus.read(sum);
        self.pc = self.pc.wrapping_add(1);
        Operand::Address(sum)
    }
}

// instructions
impl Cpu {
    pub fn lda<B: bus::Bus>(&mut self, bus: &mut B, operand: Operand) {
        let value = operand.address_value(bus);
        self.a = value;
        self.set_zn(value);
    }
    pub fn sta<B: bus::Bus>(&self, bus: &mut B, operand: Operand) {
        let address = operand.address();
        bus.write(self.a, address);
    }
    pub fn ldx<B: bus::Bus>(&mut self, bus: &mut B, operand: Operand) {
        let value = operand.address_value(bus);
        self.x = value;
        self.set_zn(value);
    }
    pub fn stx<B: bus::Bus>(&self, bus: &mut B, operand: Operand) {
        let address = operand.address();
        bus.write(self.x, address);
    }
    pub fn ldy<B: bus::Bus>(&mut self, bus: &mut B, operand: Operand) {
        let value = operand.address_value(bus);
        self.y = value;
        self.set_zn(value);
    }
    pub fn sty<B: bus::Bus>(&self, bus: &mut B, operand: Operand) {
        let address = operand.address();
        bus.write(self.y, address);
    }
    pub fn tax(&mut self) {
        self.x = self.a;
        self.set_zn(self.a);
    }
    pub fn txa(&mut self) {
        self.a = self.x;
        self.set_zn(self.x);
    }
    pub fn tay(&mut self) {
        self.y = self.a;
        self.set_zn(self.a);
    }
    pub fn tya(&mut self) {
        self.a = self.y;
        self.set_zn(self.y);
    }
    pub fn adc<B: bus::Bus>(&mut self, bus: &mut B, operand: Operand) {
        let value = operand.address_value(bus);
        let sum = u16::from(self.a)
            .wrapping_add(u16::from(value))
            .wrapping_add(u16::from(self.carry()));
        self.set_czvn_adc(sum, value);
        self.a = sum as u8;
    }
    pub fn sbc<B: bus::Bus>(&mut self, bus: &mut B, operand: Operand) {
        let value = operand.address_value(bus);
        let sum = i16::from(self.a) - i16::from(value) - i16::from(!self.carry());
        self.set_czvn_sbc(sum, value);
        self.a = sum as u8;
    }
    pub fn inc<B: bus::Bus>(&mut self, bus: &mut B, operand: Operand) {
        let address = operand.address();
        let old_value = bus.read(address);
        bus.write(old_value, address);
        let new_value = old_value.wrapping_add(1);
        bus.write(new_value, address);
        self.set_zn(new_value);
    }
    pub fn dec<B: bus::Bus>(&mut self, bus: &mut B, operand: Operand) {
        let address = operand.address();
        let old_value = bus.read(address);
        bus.write(old_value, address);
        let new_value = old_value.wrapping_sub(1);
        bus.write(new_value, address);
        self.set_zn(new_value);
    }
    pub fn inx(&mut self) {
        let value = self.x.wrapping_add(1);
        self.x = value;
        self.set_zn(value);
    }
    pub fn dex(&mut self) {
        let value = self.x.wrapping_sub(1);
        self.x = value;
        self.set_zn(value);
    }
    pub fn iny(&mut self) {
        let value = self.y.wrapping_add(1);
        self.y = value;
        self.set_zn(value);
    }
    pub fn dey(&mut self) {
        let value = self.y.wrapping_sub(1);
        self.y = value;
        self.set_zn(value);
    }
    pub fn asl<B: bus::Bus>(&mut self, bus: &mut B, operand: Operand) {
        if let Operand::Accumulator = operand {
            let value = u16::from(self.a);
            let result = (value << 1) as u8;
            self.a = result;
            self.set_carry((value & 0x80) != 0);
            self.set_zn(result);
        } else if let Operand::Address(address) = operand {
            let value = bus.read(address);
            bus.write(value, address);
            let result = value << 1;
            bus.write(result, address);
            self.set_carry((value & 0x80) != 0);
            self.set_zn(result);
        }
    }
    pub fn lsr<B: bus::Bus>(&mut self, bus: &mut B, operand: Operand) {
        if let Operand::Accumulator = operand {
            let value = self.a;
            let result = value >> 1;
            self.a = result;
            self.set_carry((value & 0x01) != 0);
            self.set_zero(result == 0);
            self.set_negative(false);
        } else if let Operand::Address(address) = operand {
            let value = bus.read(address);
            bus.write(value, address);
            let result = value >> 1;
            bus.write(result, address);
            self.set_carry((value & 0x01) != 0);
            self.set_zero(result == 0);
            self.set_negative(false);
        }
    }
    pub fn rol<B: bus::Bus>(&mut self, bus: &mut B, operand: Operand) {
        if let Operand::Accumulator = operand {
            let value = self.a;
            let carry = u8::from(self.carry());
            let result = (value << 1) | carry;
            self.a = result;
            self.set_carry((value & 0x80) != 0);
            self.set_zn(result);
        } else if let Operand::Address(address) = operand {
            let value = bus.read(address);
            bus.write(value, address);
            let carry = u8::from(self.carry());
            let result = (value << 1) | carry;
            bus.write(result, address);
            self.set_carry((value & 0x80) != 0);
            self.set_zn(result);
        }
    }
    pub fn ror<B: bus::Bus>(&mut self, bus: &mut B, operand: Operand) {
        if let Operand::Accumulator = operand {
            let value = self.a;
            let carry = (u8::from(self.carry())) << 7;
            let result = (value >> 1) | carry;
            self.a = result;
            self.set_carry((value & 0x01) != 0);
            self.set_zn(result);
        } else if let Operand::Address(address) = operand {
            let value = bus.read(address);
            bus.write(value, address);
            let carry = (u8::from(self.carry())) << 7;
            let result = (value >> 1) | carry;
            bus.write(result, address);
            self.set_carry((value & 0x01) != 0);
            self.set_zn(result);
        }
    }
    pub fn and<B: bus::Bus>(&mut self, bus: &mut B, operand: Operand) {
        let address = operand.address();
        let value = bus.read(address);
        let result = self.a & value;
        self.a = result;
        self.set_zn(result);
    }
    pub fn ora<B: bus::Bus>(&mut self, bus: &mut B, operand: Operand) {
        let address = operand.address();
        let value = bus.read(address);
        let result = self.a | value;
        self.a = result;
        self.set_zn(result);
    }
    pub fn eor<B: bus::Bus>(&mut self, bus: &mut B, operand: Operand) {
        let address = operand.address();
        let value = bus.read(address);
        let result = self.a ^ value;
        self.a = result;
        self.set_zn(result);
    }
    pub fn bit<B: bus::Bus>(&mut self, bus: &mut B, operand: Operand) {
        let address = operand.address();
        let value = bus.read(address);
        let result = self.a & value;
        self.set_zero(result == 0);
        self.set_overflow((value & 0x40) != 0);
        self.set_negative((value & 0x80) != 0);
    }
    pub fn cmp<B: bus::Bus>(&mut self, bus: &mut B, operand: Operand) {
        let address = operand.address();
        let value = bus.read(address);
        self.set_czn_cmp(self.a, value);
    }
    pub fn cpx<B: bus::Bus>(&mut self, bus: &mut B, operand: Operand) {
        let address = operand.address();
        let value = bus.read(address);
        self.set_czn_cmp(self.x, value);
    }
    pub fn cpy<B: bus::Bus>(&mut self, bus: &mut B, operand: Operand) {
        let address = operand.address();
        let value = bus.read(address);
        self.set_czn_cmp(self.y, value);
    }
    pub fn bcc(&mut self, bus: &mut impl Bus, operand: Operand) {
        let offset = operand.offset();
        if !self.carry() {
            self.branch(bus, offset);
        }
    }
    pub fn bcs(&mut self, bus: &mut impl Bus, operand: Operand) {
        let offset = operand.offset();
        if self.carry() {
            self.branch(bus, offset);
        }
    }
    pub fn beq(&mut self, bus: &mut impl Bus, operand: Operand) {
        let offset = operand.offset();
        if self.zero() {
            self.branch(bus, offset);
        }
    }
    pub fn bne(&mut self, bus: &mut impl Bus, operand: Operand) {
        let offset = operand.offset();
        if !self.zero() {
            self.branch(bus, offset);
        }
    }
    pub fn bpl(&mut self, bus: &mut impl Bus, operand: Operand) {
        let offset = operand.offset();
        if !self.negative() {
            self.branch(bus, offset);
        }
    }
    pub fn bmi(&mut self, bus: &mut impl Bus, operand: Operand) {
        let offset = operand.offset();
        if self.negative() {
            self.branch(bus, offset);
        }
    }
    pub fn bvc(&mut self, bus: &mut impl Bus, operand: Operand) {
        let offset = operand.offset();
        if !self.overflow() {
            self.branch(bus, offset);
        }
    }
    pub fn bvs(&mut self, bus: &mut impl Bus, operand: Operand) {
        let offset = operand.offset();
        if self.overflow() {
            self.branch(bus, offset);
        }
    }
    pub fn jmp(&mut self, operand: Operand) {
        let address = operand.address();
        self.pc = address;
    }
    pub fn jsr<B: bus::Bus>(&mut self, bus: &mut B) {
        let low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        bus.read(0x0100 | self.sp as u16);
        self.push_pc(bus);
        let high = bus.read(self.pc);
        self.pc = (high as u16) << 8 | low as u16;
    }
    pub fn rts<B: bus::Bus>(&mut self, bus: &mut B) {
        self.pull_pc(bus);
        bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
    }
    pub fn brk<B: bus::Bus>(&mut self, bus: &mut B) {
        self.push_pc(bus);
        self.push_flags(bus);
        self.set_interrupt(true);
        self.pc = bus.read_u16(0xFFFE);
    }
    pub fn rti<B: bus::Bus>(&mut self, bus: &mut B) {
        self.pull_flags(bus);
        self.pull_pc(bus);
    }
    pub fn pha<B: bus::Bus>(&mut self, bus: &mut B) {
        self.push(bus, self.a);
    }
    pub fn pla<B: bus::Bus>(&mut self, bus: &mut B) {
        let result = self.pull(bus);
        bus.read(0x0100 & self.sp as u16);
        self.a = result;
        self.set_zn(result);
    }
    pub fn php<B: bus::Bus>(&mut self, bus: &mut B) {
        self.push_flags(bus);
    }
    pub fn plp<B: bus::Bus>(&mut self, bus: &mut B) {
        self.pull_flags(bus);
        bus.read(0x0100 & self.sp as u16);
    }
    pub fn txs(&mut self) {
        self.sp = self.x;
    }
    pub fn tsx(&mut self) {
        self.x = self.sp;
        self.set_zn(self.sp);
    }
    pub fn clc(&mut self) {
        self.set_carry(false);
    }
    pub fn sec(&mut self) {
        self.set_carry(true);
    }
    pub fn cli(&mut self) {
        self.set_interrupt(false);
    }
    pub fn sei(&mut self) {
        self.set_interrupt(true);
    }
    pub fn cld(&mut self) {
        self.set_decimal(false);
    }
    pub fn sed(&mut self) {
        self.set_decimal(true);
    }
    pub fn clv(&mut self) {
        self.set_overflow(false);
    }
    pub fn sax<B: bus::Bus>(&self, bus: &mut B, operand: Operand) {
        let address = operand.address();
        let value = self.a & self.x;
        bus.write(value, address);
    }
    fn nop(&mut self, bus: &mut impl Bus, operand: Operand) {
        bus.read(operand.address());
    }
    fn dcp(&mut self, bus: &mut impl Bus, operand: Operand) {
        let address = operand.address();
        let old_value = bus.read(address);
        bus.write(old_value, address);
        let new_value = old_value.wrapping_sub(1);
        bus.write(new_value, address);
        self.set_zn(new_value);
        self.set_czn_cmp(self.a, new_value);
    }
    fn isc(&mut self, bus: &mut impl Bus, operand: Operand) {
        let address = operand.address();
        let old_value = bus.read(address);
        bus.write(old_value, address);
        let new_value = old_value.wrapping_add(1);
        bus.write(new_value, address);
        self.set_zn(new_value);
        let sum = i16::from(self.a) - i16::from(new_value) - i16::from(!self.carry());
        self.set_czvn_sbc(sum, new_value);
        self.a = sum as u8;
    }
    fn slo(&mut self, bus: &mut impl Bus, operand: Operand) {
        let address = operand.address();
        let value = bus.read(operand.address());
        bus.write(value, address);
        let result = value << 1;
        bus.write(result, address);
        self.set_carry((value & 0x80) != 0);
        self.set_zn(result);

        let result = self.a | result;
        self.a = result;
        self.set_zn(result);
    }
    fn rla(&mut self, bus: &mut impl Bus, operand: Operand) {
        let address = operand.address();
        let value = bus.read(address);
        bus.write(value, address);
        let carry = u8::from(self.carry());
        let result = (value << 1) | carry;
        bus.write(result, address);
        self.set_carry((value & 0x80) != 0);
        self.set_zn(result);

        let result = self.a & result;
        self.a = result;
        self.set_zn(result);
    }
    fn sre(&mut self, bus: &mut impl Bus, operand: Operand) {
        let address = operand.address();
        let value = bus.read(address);
        bus.write(value, address);
        let result = value >> 1;
        bus.write(result, address);
        self.set_carry((value & 0x01) != 0);
        self.set_zero(result == 0);
        self.set_negative(false);

        let result = self.a ^ result;
        self.a = result;
        self.set_zn(result);
    }
    fn rra(&mut self, bus: &mut impl Bus, operand: Operand) {
        let address = operand.address();
        let value = bus.read(address);
        bus.write(value, address);
        let carry = (u8::from(self.carry())) << 7;
        let result = (value >> 1) | carry;
        bus.write(result, address);
        self.set_carry((value & 0x01) != 0);
        self.set_zn(result);

        let sum = u16::from(self.a)
            .wrapping_add(u16::from(result))
            .wrapping_add(u16::from(self.carry()));
        self.set_czvn_adc(sum, result);
        self.a = sum as u8;
    }

    fn page_crossed(&self, bus: &mut impl Bus, old_address: u16, new_address: u16) {
        let old_high = old_address >> 8;
        let new_high = new_address >> 8;
        let new_low = new_address & 0x00FF;
        if old_high != new_high {
            bus.read(old_high << 8 | new_low);
        };
    }
    pub fn branch(&mut self, bus: &mut impl Bus, offset: i8) {
        bus.read(self.pc);
        let result = self.pc.wrapping_add_signed(i16::from(offset));
        self.page_crossed(bus, self.pc, result);
        self.pc = result;
    }
    pub fn push<B: bus::Bus>(&mut self, bus: &mut B, value: u8) {
        bus.write(value, 0x0100 + u16::from(self.sp));
        self.sp = self.sp.wrapping_sub(1);
    }
    pub fn pull<B: bus::Bus>(&mut self, bus: &mut B) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        bus.read(0x0100 + u16::from(self.sp))
    }
    pub fn push_pc<B: bus::Bus>(&mut self, bus: &mut B) {
        let high = (self.pc >> 8) as u8;
        let low = self.pc as u8;
        self.push(bus, high);
        self.push(bus, low);
    }
    pub fn push_flags<B: bus::Bus>(&mut self, bus: &mut B) {
        let flags = (self.p | BREAK) | (1 << 4);
        self.push(bus, flags);
    }
    pub fn pull_pc<B: bus::Bus>(&mut self, bus: &mut B) {
        bus.read(0x0100 | self.sp as u16);
        let low = u16::from(self.pull(bus));
        let high = u16::from(self.pull(bus));
        let address = (high << 8) | low;
        self.pc = address;
    }
    pub fn pull_flags<B: bus::Bus>(&mut self, bus: &mut B) {
        let flags = self.pull(bus);
        self.p = (flags & !BREAK) | UNUSED;
    }
}

// flags
impl Cpu {
    pub fn status(&self) -> u8 {
        self.p
    }

    pub fn carry(&self) -> bool {
        self.p & CARRY != 0
    }
    pub fn zero(&self) -> bool {
        self.p & ZERO != 0
    }
    pub fn interrupt(&self) -> bool {
        self.p & INTERRUPT != 0
    }
    pub fn decimal(&self) -> bool {
        self.p & DECIMAL != 0
    }
    pub fn overflow(&self) -> bool {
        self.p & OVERFLOW != 0
    }
    pub fn negative(&self) -> bool {
        self.p & NEGATIVE != 0
    }

    pub fn set_carry(&mut self, value: bool) {
        if value {
            self.p |= CARRY;
        } else {
            self.p &= !CARRY;
        }
    }
    pub fn set_zero(&mut self, value: bool) {
        if value {
            self.p |= ZERO;
        } else {
            self.p &= !ZERO;
        }
    }
    pub fn set_interrupt(&mut self, value: bool) {
        if value {
            self.p |= INTERRUPT;
        } else {
            self.p &= !INTERRUPT;
        }
    }
    pub fn set_decimal(&mut self, value: bool) {
        if value {
            self.p |= DECIMAL;
        } else {
            self.p &= !DECIMAL;
        }
    }
    pub fn set_overflow(&mut self, value: bool) {
        if value {
            self.p |= OVERFLOW;
        } else {
            self.p &= !OVERFLOW;
        }
    }
    pub fn set_negative(&mut self, value: bool) {
        if value {
            self.p |= NEGATIVE;
        } else {
            self.p &= !NEGATIVE;
        }
    }

    pub fn set_zn(&mut self, result: u8) {
        self.set_zero(result == 0);
        self.set_negative(result & 0x80 != 0);
    }
    pub fn set_czvn_adc(&mut self, result: u16, memory: u8) {
        self.set_zn(result as u8);
        self.set_carry(result > 0xFF);
        self.set_overflow(((result as u8 ^ self.a) & (result as u8 ^ memory) & 0x80) != 0);
    }
    pub fn set_czvn_sbc(&mut self, result: i16, memory: u8) {
        self.set_zn(result as u8);
        self.set_carry(!(result < 0x00));
        self.set_overflow(((result as u8 ^ self.a) & (result as u8 ^ !memory) & 0x80) != 0);
    }
    pub fn set_czn_cmp(&mut self, register_value: u8, memory_value: u8) {
        let result = register_value.wrapping_sub(memory_value);
        self.set_carry(register_value >= memory_value);
        self.set_zero(register_value == memory_value);
        self.set_negative((result & 0x80) != 0);
    }
}

impl Operand {
    pub fn address(self) -> u16 {
        if let Self::Address(address) = self {
            address
        } else {
            panic!("Invalid operand type")
        }
    }
    pub fn address_value<B: bus::Bus>(self, bus: &mut B) -> u8 {
        if let Self::Address(address) = self {
            bus.read(address)
        } else {
            panic!("Invalid operand type")
        }
    }
    pub fn offset(self) -> i8 {
        if let Self::Relative(offset) = self {
            offset
        } else {
            panic!("Invalid operand type")
        }
    }
}
