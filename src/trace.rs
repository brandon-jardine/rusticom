use std::collections::HashMap;

use crate::cpu::CPU;
use crate::mem::Mem;
use crate::opcode;

#[cfg(test)]
mod tests;

pub fn trace(cpu: &CPU) -> String {
    let ref opcodes: HashMap<u8, &'static opcode::OpCode> = *opcode::OPCODES_MAP;
    
    let instr_byte_one: u8 = cpu.mem_read(cpu.program_counter);
    let instr_byte_two: u8 = cpu.mem_read(cpu.program_counter + 1);
    let instr_byte_three: u8 = cpu.mem_read(cpu.program_counter + 2);

    let opcode = opcodes.get(&instr_byte_one).expect(&format!("OpCode {:x} is not recognized", instr_byte_one));

    let opcode_hex = match opcode.len {
        1 => format!("{:02X}      ", instr_byte_one),
        2 => format!("{:02X} {:02X}   ", instr_byte_one, instr_byte_two),
        _ => format!("{:02X} {:02X} {:02X}", instr_byte_one, instr_byte_two, instr_byte_three),
    };

    format!(
        "{pc:04X}  {opcode_hex}  {opcode_asm}",
        pc=cpu.program_counter,
        opcode_hex=opcode_hex,
        opcode_asm=opcode.mnemonic,
    )
}

