use crate::cpu::AddressingMode;
use std::collections::HashMap;

pub struct OpCode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
    pub undocumented: bool,
}

impl OpCode {
    pub fn new(code: u8, mnemonic: &'static str, len: u8, cycles: u8, mode: AddressingMode) -> Self {
        OpCode {
            code,
            mnemonic,
            len,
            cycles,
            mode,
            undocumented: false,
        }
    }

    pub fn new_undoc(code: u8, mnemonic: &'static str, len: u8, cycles: u8, mode: AddressingMode) -> Self {
        OpCode {
            undocumented: true,
            ..OpCode::new(code, mnemonic, len, cycles, mode)
        }
    }
}

lazy_static! {
    pub static ref CPU_OP_CODES: Vec<OpCode> = vec![
        // OFFICIAL OPCODES

        OpCode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x65, "ADC", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x75, "ADC", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x6D, "ADC", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x7D, "ADC", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_X),
        OpCode::new(0x79, "ADC", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_Y),
        OpCode::new(0x61, "ADC", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x71, "ADC", 2, 5 /* +1 if page crossed */, AddressingMode::Indirect_Y),

        OpCode::new(0x29, "AND", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x25, "AND", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x35, "AND", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x2D, "AND", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x3D, "AND", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_X),
        OpCode::new(0x39, "AND", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_Y),
        OpCode::new(0x21, "AND", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x31, "AND", 2, 5 /* +1 if page crossed */, AddressingMode::Indirect_Y),

        OpCode::new(0x0A, "ASL", 1, 2, AddressingMode::None),
        OpCode::new(0x06, "ASL", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x16, "ASL", 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new(0x0E, "ASL", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x1E, "ASL", 3, 7, AddressingMode::Absolute_X),

        OpCode::new(0x90, "BCC", 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::None),
        OpCode::new(0xB0, "BCS", 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::None),
        OpCode::new(0xF0, "BEQ", 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::None),
        OpCode::new(0x30, "BMI", 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::None),
        OpCode::new(0xD0, "BNE", 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::None),
        OpCode::new(0x10, "BPL", 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::None),
        OpCode::new(0x50, "BVC", 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::None),
        OpCode::new(0x70, "BVS", 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::None),

        OpCode::new(0x24, "BIT", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x2C, "BIT", 3, 4, AddressingMode::Absolute),

        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::None),

        OpCode::new(0x18, "CLC", 1, 2, AddressingMode::None),
        OpCode::new(0xD8, "CLD", 1, 2, AddressingMode::None),
        OpCode::new(0x58, "CLI", 1, 2, AddressingMode::None),
        OpCode::new(0xB8, "CLV", 1, 2, AddressingMode::None),

        OpCode::new(0xC9, "CMP", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xC5, "CMP", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xD5, "CMP", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xCD, "CMP", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xDD, "CMP", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_X),
        OpCode::new(0xD9, "CMP", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_Y),
        OpCode::new(0xC1, "CMP", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0xD1, "CMP", 2, 5 /* +1 if page crossed */, AddressingMode::Indirect_Y),

        OpCode::new(0xE0, "CPX", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xE4, "CPX", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xEC, "CPX", 3, 4, AddressingMode::Absolute),

        OpCode::new(0xC0, "CPY", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xC4, "CPY", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xCC, "CPY", 3, 4, AddressingMode::Absolute),

        OpCode::new(0xC6, "DEC", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0xD6, "DEC", 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new(0xCE, "DEC", 3, 6, AddressingMode::Absolute),
        OpCode::new(0xDE, "DEC", 3, 7, AddressingMode::Absolute_X),

        OpCode::new(0xCA, "DEX", 1, 2, AddressingMode::None),
        OpCode::new(0x88, "DEY", 1, 2, AddressingMode::None),

        OpCode::new(0x49, "EOR", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x45, "EOR", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x55, "EOR", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x4D, "EOR", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x5D, "EOR", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_X),
        OpCode::new(0x59, "EOR", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_Y),
        OpCode::new(0x41, "EOR", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x51, "EOR", 2, 5 /* +1 if page crossed */, AddressingMode::Indirect_Y),

        OpCode::new(0x4C, "JMP", 3, 3, AddressingMode::None),
        OpCode::new(0x6C, "JMP", 3, 5, AddressingMode::None),
        
        OpCode::new(0x20, "JSR", 3, 6, AddressingMode::None),
        OpCode::new(0x60, "RTS", 1, 6, AddressingMode::None),
        OpCode::new(0x40, "RTI", 1, 6, AddressingMode::None),

        OpCode::new(0xE6, "INC", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0xF6, "INC", 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new(0xEE, "INC", 3, 6, AddressingMode::Absolute),
        OpCode::new(0xFE, "INC", 3, 7, AddressingMode::Absolute_X),

        OpCode::new(0xE8, "INX", 1, 2, AddressingMode::None),
        OpCode::new(0xC8, "INY", 1, 2, AddressingMode::None),

        OpCode::new(0xA9, "LDA", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xA5, "LDA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xB5, "LDA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xAD, "LDA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xBD, "LDA", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_X),
        OpCode::new(0xB9, "LDA", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_Y),
        OpCode::new(0xA1, "LDA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0xB1, "LDA", 2, 5 /* +1 if page crossed */, AddressingMode::Indirect_Y),

        OpCode::new(0xA2, "LDX", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xA6, "LDX", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xB6, "LDX", 2, 4, AddressingMode::ZeroPage_Y),
        OpCode::new(0xAE, "LDX", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xBE, "LDX", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_Y),

        OpCode::new(0xA0, "LDY", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xA4, "LDY", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xB4, "LDY", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xAC, "LDY", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xBC, "LDY", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_X),

        OpCode::new(0x4A, "LSR", 1, 2, AddressingMode::None),
        OpCode::new(0x46, "LSR", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x56, "LSR", 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new(0x4E, "LSR", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x5E, "LSR", 3, 7, AddressingMode::Absolute_X),

        OpCode::new(0xEA, "NOP", 1, 2, AddressingMode::None),

        OpCode::new(0x09, "ORA", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x05, "ORA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x15, "ORA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x0D, "ORA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x1D, "ORA", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_X),
        OpCode::new(0x19, "ORA", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_Y),
        OpCode::new(0x01, "ORA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x11, "ORA", 2, 5 /* +1 if page crossed */, AddressingMode::Indirect_Y),

        OpCode::new(0x48, "PHA", 1, 3, AddressingMode::None),
        OpCode::new(0x08, "PHP", 1, 3, AddressingMode::None),
        OpCode::new(0x68, "PLA", 1, 4, AddressingMode::None),
        OpCode::new(0x28, "PLP", 1, 4, AddressingMode::None),

        OpCode::new(0x2A, "ROL", 1, 2, AddressingMode::None),
        OpCode::new(0x26, "ROL", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x36, "ROL", 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new(0x2E, "ROL", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x3E, "ROL", 3, 7, AddressingMode::Absolute_X),

        OpCode::new(0x6A, "ROR", 1, 2, AddressingMode::None),
        OpCode::new(0x66, "ROR", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x76, "ROR", 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new(0x6E, "ROR", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x7E, "ROR", 3, 7, AddressingMode::Absolute_X),

        OpCode::new(0xE9, "SBC", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xE5, "SBC", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xF5, "SBC", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xED, "SBC", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xFD, "SBC", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_X),
        OpCode::new(0xF9, "SBC", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_Y),
        OpCode::new(0xE1, "SBC", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0xF1, "SBC", 2, 5 /* +1 if page crossed */, AddressingMode::Indirect_Y),

        OpCode::new(0x38, "SEC", 1, 2, AddressingMode::None),
        OpCode::new(0xF8, "SED", 1, 2, AddressingMode::None),
        OpCode::new(0x78, "SEI", 1, 2, AddressingMode::None),

        OpCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x8D, "STA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x9D, "STA", 3, 5, AddressingMode::Absolute_X),
        OpCode::new(0x99, "STA", 3, 5, AddressingMode::Absolute_Y),
        OpCode::new(0x81, "STA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x91, "STA", 2, 6, AddressingMode::Indirect_Y),

        OpCode::new(0x86, "STX", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x96, "STX", 2, 4, AddressingMode::ZeroPage_Y),
        OpCode::new(0x8E, "STX", 3, 4, AddressingMode::Absolute),

        OpCode::new(0x84, "STY", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x94, "STY", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x8C, "STY", 3, 4, AddressingMode::Absolute),

        OpCode::new(0xAA, "TAX", 1, 2, AddressingMode::None),
        OpCode::new(0xA8, "TAY", 1, 2, AddressingMode::None),
        OpCode::new(0xBA, "TSX", 1, 2, AddressingMode::None),
        OpCode::new(0x8A, "TXA", 1, 2, AddressingMode::None),
        OpCode::new(0x9A, "TXS", 1, 2, AddressingMode::None),
        OpCode::new(0x98, "TYA", 1, 2, AddressingMode::None),

        // UN-OFFICIAL NES OPCODES

        // Unofficial NOPs
        OpCode::new_undoc(0x1A, "NOP", 1, 2, AddressingMode::None),
        OpCode::new_undoc(0x3A, "NOP", 1, 2, AddressingMode::None),
        OpCode::new_undoc(0x5A, "NOP", 1, 2, AddressingMode::None),
        OpCode::new_undoc(0x7A, "NOP", 1, 2, AddressingMode::None),
        OpCode::new_undoc(0xDA, "NOP", 1, 2, AddressingMode::None),
        OpCode::new_undoc(0xFA, "NOP", 1, 2, AddressingMode::None),

        // Unofficial SKB
        // Reads an immediate byte and skip/discard it
        OpCode::new_undoc(0x80, "NOP", 2, 2, AddressingMode::Immediate),
        OpCode::new_undoc(0x82, "NOP", 2, 2, AddressingMode::Immediate),
        OpCode::new_undoc(0x89, "NOP", 2, 2, AddressingMode::Immediate),
        OpCode::new_undoc(0xC2, "NOP", 2, 2, AddressingMode::Immediate),
        OpCode::new_undoc(0xE2, "NOP", 2, 2, AddressingMode::Immediate),

        // Unofficial IGN
        // Reads a byte from memory and ignores it
        OpCode::new_undoc(0x0C, "NOP", 3, 4, AddressingMode::Absolute),
        OpCode::new_undoc(0x1C, "NOP", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_X),
        OpCode::new_undoc(0x3C, "NOP", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_X),
        OpCode::new_undoc(0x5C, "NOP", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_X),
        OpCode::new_undoc(0x7C, "NOP", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_X),
        OpCode::new_undoc(0xDC, "NOP", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_X),
        OpCode::new_undoc(0xFC, "NOP", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_X),
        OpCode::new_undoc(0x04, "NOP", 2, 3, AddressingMode::ZeroPage),
        OpCode::new_undoc(0x44, "NOP", 2, 3, AddressingMode::ZeroPage),
        OpCode::new_undoc(0x64, "NOP", 2, 3, AddressingMode::ZeroPage),
        OpCode::new_undoc(0x14, "NOP", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new_undoc(0x34, "NOP", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new_undoc(0x54, "NOP", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new_undoc(0x74, "NOP", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new_undoc(0xD4, "NOP", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new_undoc(0xF4, "NOP", 2, 4, AddressingMode::ZeroPage_X),

        // Unofficial LAX
        // Shortcut for LDA then TAX
        OpCode::new_undoc(0xA3, "LAX", 2, 6, AddressingMode::Indirect_X),
        OpCode::new_undoc(0xA7, "LAX", 2, 3, AddressingMode::ZeroPage),
        OpCode::new_undoc(0xAF, "LAX", 3, 4, AddressingMode::Absolute),
        OpCode::new_undoc(0xB3, "LAX", 2, 5, AddressingMode::Indirect_Y),
        OpCode::new_undoc(0xB7, "LAX", 2, 4, AddressingMode::ZeroPage_Y),
        OpCode::new_undoc(0xBF, "LAX", 3, 4, AddressingMode::Absolute_Y),

        // Unofficial SAX
        // Store AND of A and X (no flags affected)
        OpCode::new_undoc(0x83, "SAX", 2, 6, AddressingMode::Indirect_X),
        OpCode::new_undoc(0x87, "SAX", 2, 3, AddressingMode::ZeroPage),
        OpCode::new_undoc(0x8F, "SAX", 3, 4, AddressingMode::Absolute),
        OpCode::new_undoc(0x97, "SAX", 2, 4, AddressingMode::ZeroPage_Y),

        // Duplicate SDB immediate
        OpCode::new_undoc(0xEB, "SBC", 2, 2, AddressingMode::Immediate),

        // Unofficial DCP
        // Equiv. of DEC then CMP
        OpCode::new_undoc(0xC3, "DCP", 2, 8, AddressingMode::Indirect_X),
        OpCode::new_undoc(0xC7, "DCP", 2, 5, AddressingMode::ZeroPage),
        OpCode::new_undoc(0xCF, "DCP", 3, 6, AddressingMode::Absolute),
        OpCode::new_undoc(0xD3, "DCP", 2, 8, AddressingMode::Indirect_Y),
        OpCode::new_undoc(0xD7, "DCP", 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new_undoc(0xDB, "DCP", 3, 7, AddressingMode::Absolute_Y),
        OpCode::new_undoc(0xDF, "DCP", 3, 7, AddressingMode::Absolute_X),

        // Unofficial ISB (or ISC)
        // Equiv. of INC then SBC
        OpCode::new_undoc(0xE3, "ISB", 2, 8, AddressingMode::Indirect_X),
        OpCode::new_undoc(0xE7, "ISB", 2, 5, AddressingMode::ZeroPage),
        OpCode::new_undoc(0xEF, "ISB", 3, 6, AddressingMode::Absolute),
        OpCode::new_undoc(0xF3, "ISB", 2, 8, AddressingMode::Indirect_Y),
        OpCode::new_undoc(0xF7, "ISB", 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new_undoc(0xFB, "ISB", 3, 7, AddressingMode::Absolute_Y),
        OpCode::new_undoc(0xFF, "ISB", 3, 7, AddressingMode::Absolute_X),

        // Unofficial SLO
        // Equiv. of ASL then ORA
        OpCode::new_undoc(0x03, "SLO", 2, 8, AddressingMode::Indirect_X),
        OpCode::new_undoc(0x07, "SLO", 2, 5, AddressingMode::ZeroPage),
        OpCode::new_undoc(0x0F, "SLO", 3, 6, AddressingMode::Absolute),
        OpCode::new_undoc(0x13, "SLO", 2, 8, AddressingMode::Indirect_Y),
        OpCode::new_undoc(0x17, "SLO", 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new_undoc(0x1B, "SLO", 3, 7, AddressingMode::Absolute_Y),
        OpCode::new_undoc(0x1F, "SLO", 3, 7, AddressingMode::Absolute_X),

        // Unofficial RLA
        // Equiv. of ROL then AND
        OpCode::new_undoc(0x23, "RLA", 2, 8, AddressingMode::Indirect_X),
        OpCode::new_undoc(0x27, "RLA", 2, 5, AddressingMode::ZeroPage),
        OpCode::new_undoc(0x2F, "RLA", 3, 6, AddressingMode::Absolute),
        OpCode::new_undoc(0x33, "RLA", 2, 8, AddressingMode::Indirect_Y),
        OpCode::new_undoc(0x37, "RLA", 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new_undoc(0x3B, "RLA", 3, 7, AddressingMode::Absolute_Y),
        OpCode::new_undoc(0x3F, "RLA", 3, 7, AddressingMode::Absolute_X),

        // Unofficial SRE
        // Equiv. of LSR then EOR
        OpCode::new_undoc(0x43, "SRE", 2, 8, AddressingMode::Indirect_X),
        OpCode::new_undoc(0x47, "SRE", 2, 5, AddressingMode::ZeroPage),
        OpCode::new_undoc(0x4F, "SRE", 3, 6, AddressingMode::Absolute),
        OpCode::new_undoc(0x53, "SRE", 2, 8, AddressingMode::Indirect_Y),
        OpCode::new_undoc(0x57, "SRE", 2, 6, AddressingMode::ZeroPage_X),
        OpCode::new_undoc(0x5B, "SRE", 3, 7, AddressingMode::Absolute_Y),
        OpCode::new_undoc(0x5F, "SRE", 3, 7, AddressingMode::Absolute_X),

        // Unofficial RRA 
        // Equiv. of ROR then ADC
        OpCode::new_undoc(0x63, "RRA", 2, 8, AddressingMode::Indirect_X),
        OpCode::new_undoc(0x67, "RRA", 2, 5, AddressingMode::ZeroPage),
        OpCode::new_undoc(0x6F, "RRA", 3, 6, AddressingMode::Absolute),
        OpCode::new_undoc(0x73, "RRA", 2, 8, AddressingMode::Indirect_Y),
        OpCode::new_undoc(0x77, "RRA", 2, 8, AddressingMode::ZeroPage_X),
        OpCode::new_undoc(0x7B, "RRA", 3, 7, AddressingMode::Absolute_Y),
        OpCode::new_undoc(0x7F, "RRA", 3, 7, AddressingMode::Absolute_X),

        // Unofficial ALR
        // AND immediate, then LSR accumulator
        OpCode::new_undoc(0x4B, "ALR", 2, 2, AddressingMode::Immediate),

        // Unofficial ANC
        // AND immediate, set carry flag if N flag is set
        OpCode::new_undoc(0x0B, "ANC", 2, 2, AddressingMode::Immediate),
        OpCode::new_undoc(0x2B, "ANC", 2, 2, AddressingMode::Immediate),

        // Unofficial ARR
        // AND, then ROR, then set based on bits 5 and 6 of result:
        // check bit 5 and 6:
        // If both bits are 1: set C, clear V.
        // If both bits are 0: clear C and V.
        // If only bit 5 is 1: set V, clear C.
        // If only bit 6 is 1: set C and V.
        OpCode::new_undoc(0x6B, "ARR", 2, 2, AddressingMode::Immediate),

        // Unofficial AXS
        // Sets X to (A AND X), then subtract from X register (no borrow)
        OpCode::new_undoc(0xCB, "AXS", 2, 2, AddressingMode::Immediate),

        // Unofficial ATX
        // AND value with acc, then transfer acc to X
        OpCode::new_undoc(0xAB, "ATX", 2, 2, AddressingMode::Immediate),

        // Unofficial AXA
        // AND X with acc then AND with hi byte of target address (conflicting info on this one)
        // store in memory
        OpCode::new_undoc(0x9F, "AXA", 3, 5, AddressingMode::Absolute_Y),
        OpCode::new_undoc(0x93, "AXA", 2, 6, AddressingMode::Indirect_Y),

        // Unofficial HLT (KIL/JAM)
        // Locks up the cpu
        OpCode::new_undoc(0x02, "HLT", 1, 1, AddressingMode::None),
        OpCode::new_undoc(0x12, "HLT", 1, 1, AddressingMode::None),
        OpCode::new_undoc(0x22, "HLT", 1, 1, AddressingMode::None),
        OpCode::new_undoc(0x32, "HLT", 1, 1, AddressingMode::None),
        OpCode::new_undoc(0x42, "HLT", 1, 1, AddressingMode::None),
        OpCode::new_undoc(0x52, "HLT", 1, 1, AddressingMode::None),
        OpCode::new_undoc(0x62, "HLT", 1, 1, AddressingMode::None),
        OpCode::new_undoc(0x72, "HLT", 1, 1, AddressingMode::None),
        OpCode::new_undoc(0x92, "HLT", 1, 1, AddressingMode::None),
        OpCode::new_undoc(0xB2, "HLT", 1, 1, AddressingMode::None),
        OpCode::new_undoc(0xD2, "HLT", 1, 1, AddressingMode::None),
        OpCode::new_undoc(0xF2, "HLT", 1, 1, AddressingMode::None),

        // Unofficial LAR
        // AND memory with SP, transfer result to acc, X, and SP
        OpCode::new_undoc(0xBB, "LAR", 3, 4, AddressingMode::Absolute_Y),
    ];

    pub static ref OPCODES_MAP: HashMap<u8, &'static OpCode> = {
        let mut map =  HashMap::new();
        for cpuop in &*CPU_OP_CODES {
            map.insert(cpuop.code, cpuop);
        }
        map
    };
}

