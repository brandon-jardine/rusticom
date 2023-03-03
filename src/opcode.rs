use crate::cpu::AddressingMode;
use std::collections::HashMap;

pub struct OpCode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl OpCode {
    pub fn new(code: u8, mnemonic: &'static str, len: u8, cycles: u8, mode: AddressingMode) -> Self {
        OpCode {
            code,
            mnemonic,
            len,
            cycles,
            mode,
        }
    }
}

lazy_static! {
    pub static ref CPU_OP_CODES: Vec<OpCode> = vec![
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::Implied),
        OpCode::new(0xe8, "INX", 1, 2, AddressingMode::Implied),

        OpCode::new(0xa9, "LDA", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa5, "LDA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb5, "LDA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xad, "LDA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbd, "LDA", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_X),
        OpCode::new(0xb9, "LDA", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_Y),
        OpCode::new(0xa1, "LDA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0xb1, "LDA", 2, 5 /* +1 if page crossed */, AddressingMode::Indirect_Y),

        OpCode::new(0xa2, "LDX", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa6, "LDX", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb6, "LDX", 2, 4, AddressingMode::ZeroPage_Y),
        OpCode::new(0xae, "LDX", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbe, "LDX", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_Y),

        OpCode::new(0xa0, "LDY", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa4, "LDY", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb4, "LDY", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xac, "LDY", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbc, "LDY", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_X),

        OpCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x8d, "STA", 2, 4, AddressingMode::Absolute),
        OpCode::new(0x9d, "STA", 3, 5, AddressingMode::Absolute_X),
        OpCode::new(0x99, "STA", 3, 5, AddressingMode::Absolute_Y),
        OpCode::new(0x81, "STA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x91, "STA", 2, 6, AddressingMode::Indirect_Y),

        OpCode::new(0x84, "STY", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x94, "STY", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x8C, "STY", 3, 4, AddressingMode::Absolute),

        OpCode::new(0x86, "STY", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x96, "STY", 2, 4, AddressingMode::ZeroPage_Y),
        OpCode::new(0x8E, "STY", 3, 4, AddressingMode::Absolute),

        OpCode::new(0x38, "SEC", 1, 2, AddressingMode::Implied),
        OpCode::new(0xF8, "SED", 1, 2, AddressingMode::Implied),
        OpCode::new(0x78, "SEI", 1, 2, AddressingMode::Implied),

        OpCode::new(0xaa, "TAX", 1, 2, AddressingMode::Implied),
        OpCode::new(0xA8, "TAY", 1, 2, AddressingMode::Implied),
        OpCode::new(0xBA, "TSX", 1, 2, AddressingMode::Implied),
        OpCode::new(0x8A, "TXA", 1, 2, AddressingMode::Implied),
        OpCode::new(0x9A, "TXS", 1, 2, AddressingMode::Implied),
        OpCode::new(0x98, "TYA", 1, 2, AddressingMode::Implied),
    ];

    pub static ref OPCODES_MAP: HashMap<u8, &'static OpCode> = {
        let mut map =  HashMap::new();
        for cpuop in &*CPU_OP_CODES {
            map.insert(cpuop.code, cpuop);
        }
        map
    };
}
