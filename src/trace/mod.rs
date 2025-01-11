use std::collections::HashMap;

use crate::cpu::{AddressingMode, CPU};
use crate::mem::Mem;
use crate::opcode;

#[cfg(test)]
mod tests;

pub fn trace(cpu: &mut CPU) -> String {
    let ref opcodes: HashMap<u8, &'static opcode::OpCode> = *opcode::OPCODES_MAP;
    
    let instr_byte_one: u8 = cpu.mem_read(cpu.program_counter);
    let instr_byte_two: u8 = cpu.mem_read(cpu.program_counter + 1);
    let instr_byte_three: u8 = cpu.mem_read(cpu.program_counter + 2);
    let u16_addr = u16::from_le_bytes([instr_byte_two, instr_byte_three]);


    let opcode = opcodes.get(&instr_byte_one).expect(&format!("OpCode {:#02X} is not recognized", instr_byte_one));

    let opcode_hex = match opcode.len {
        1 => format!("{:02X}      ", instr_byte_one),
        2 => format!("{:02X} {:02X}   ", instr_byte_one, instr_byte_two),
        _ => format!("{:02X} {:02X} {:02X}", instr_byte_one, instr_byte_two, instr_byte_three),
    };

    let (mem_addr, stored_value) = match opcode.mode {
        AddressingMode::Immediate | AddressingMode::None => (0, 0),
        _ => {
            let (addr, _) = cpu.resolve_address(&opcode.mode, cpu.program_counter + 1);
            (addr, cpu.mem_read(addr))
        }
    };
    
    let opcode_args = match opcode.mode {
        // secial cases for 'None' AddressingMode
        AddressingMode::None => {
            match opcode.len {
                1 => {
                    match opcode.code {
                        0x0A | 0x4A | 0x2A | 0x6A => String::from("A                          "),
                        _ => String::from("                           "),
                    }
                },
                2 => {
                    // Branches fall here
                    // We need some shenanigans here to make the arithmetic work
                    let addr = (cpu.program_counter + 2)
                        .wrapping_add_signed((instr_byte_two as i8) as i16);
                    format!("${:04X}                      ", addr)
                },
                3 => { 
                    let mem_addr = cpu.mem_read_u16(cpu.program_counter + 1);
                    
                    if opcode.code == 0x6C {
                        // JMP Indirect
                        let indirect_ref = if mem_addr & 0x00FF == 0x00FF {
                            let lo = cpu.mem_read(mem_addr);
                            let hi = cpu.mem_read(mem_addr & 0xFF00);
                            u16::from_le_bytes([lo, hi])
                        } else {
                            cpu.mem_read_u16(mem_addr)
                        };

                        format!("(${:04X}) = {:04X}             ", mem_addr, indirect_ref)
                    } else {
                        format!("${:04X}                      ", mem_addr)
                    }
                },
                _ => String::from("")
            }
        },

        // length 1 modes
        AddressingMode::Implied     => String::from("                           "),

        // length 2 modes
        AddressingMode::Immediate   => format!("#${:02X}                       ", cpu.mem_read(cpu.program_counter + 1)),
        AddressingMode::ZeroPage    => format!("${:02X} = {:02X}                   ", mem_addr, stored_value),
        AddressingMode::ZeroPage_X  => format!("${:02X},X @ {:02X} = {:02X}            ", instr_byte_two, mem_addr, stored_value),
        AddressingMode::ZeroPage_Y  => format!("${:02X},Y @ {:02X} = {:02X}            ", instr_byte_two, mem_addr, stored_value),
        AddressingMode::Indirect_X  => {
            let target = instr_byte_two.wrapping_add(cpu.register_x);
            format!("(${:02X},X) @ {:02X} = {:04X} = {:02X}   ", instr_byte_two,target, mem_addr, stored_value)
        },
        AddressingMode::Indirect_Y  => {
            // I am not sure why this works with wrapping_sub
            let target = mem_addr.wrapping_sub(cpu.register_y as u16);
            format!("(${:02X}),Y = {:04X} @ {:04X} = {:02X} ", instr_byte_two, target, mem_addr, stored_value)
        },

        // length 3 modes
        AddressingMode::Absolute    => format!("${:04X} = {:02X}                 ", mem_addr, stored_value),
        AddressingMode::Absolute_X  => format!("${:04X},X @ {:04X} = {:02X}        ", u16_addr, mem_addr, stored_value),
        AddressingMode::Absolute_Y  => format!("${:04X},Y @ {:04X} = {:02X}        ", u16_addr, mem_addr, stored_value),

        _ => {
            panic!("mode {:?} is not supported", opcode.mode);
        }
    };

    format!(
        "{:04X}  {} {}{} {} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
        cpu.program_counter,
        opcode_hex,
        if opcode.undocumented {
            "*"
        } else {
            " "
        },
        opcode.mnemonic,
        opcode_args,
        cpu.register_a,
        cpu.register_x,
        cpu.register_y,
        cpu.status.bits(),
        cpu.stack_pointer,
    )
    .to_ascii_uppercase()
}

