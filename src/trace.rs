use std::collections::HashMap;

use crate::cpu::{Mem, CPU};
use crate::opcode;

#[cfg(test)]
mod tests;

pub fn trace(cpu: &CPU) -> String {
    let ref opcodes: HashMap<u8, &'static opcode::OpCode> = *opcode::OPCODES_MAP;
    
    let instr_byte_one: u8 = cpu.mem_read(cpu.program_counter);

    let opcode = opcodes.get(&instr_byte_one).expect(&format!("OpCode {:x} is not recognized", instr_byte_one));

    let mut instr_byte_two: u8 = 0;
    let mut instr_byte_three: u8 = 0;

    if opcode.len > 1 {
        instr_byte_two = cpu.mem_read(cpu.program_counter + 1);
    }
    if opcode.len > 2 {
        instr_byte_three = cpu.mem_read(cpu.program_counter + 2);
    }

    format!(
        "{pc:04X}  {instr_one:02X} {instr_two:02X} {instr_three:02X} {opcode_asm}",
        pc=cpu.program_counter,
        instr_one=instr_byte_one,
        instr_two=instr_byte_two,
        instr_three=instr_byte_three,
        opcode_asm=opcode.mnemonic,
    )
}

