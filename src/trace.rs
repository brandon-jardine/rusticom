use std::collections::HashMap;

use crate::cpu::{AddressingMode, CPU};
use crate::mem::Mem;
use crate::opcode;

#[cfg(test)]
mod tests;

pub fn trace(cpu: &CPU) -> String {
    let ref opcodes: HashMap<u8, &'static opcode::OpCode> = *opcode::OPCODES_MAP;
    
    let instr_byte_one: u8 = cpu.mem_read(cpu.program_counter);
    let instr_byte_two: u8 = cpu.mem_read(cpu.program_counter + 1);
    let instr_byte_three: u8 = cpu.mem_read(cpu.program_counter + 2);

    let opcode = opcodes.get(&instr_byte_one).expect(&format!("OpCode {:#02X} is not recognized", instr_byte_one));

    let opcode_hex = match opcode.len {
        1 => format!("{:02X}      ", instr_byte_one),
        2 => format!("{:02X} {:02X}   ", instr_byte_one, instr_byte_two),
        _ => format!("{:02X} {:02X} {:02X}", instr_byte_one, instr_byte_two, instr_byte_three),
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
                    let addr = (cpu.program_counter + 2).wrapping_add(instr_byte_two.into());
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
        AddressingMode::Implied     => format!("                           "),

        // length 2 modes
        AddressingMode::Immediate   => format!("#${:02X}                       ", cpu.mem_read(cpu.program_counter + 1)),
        AddressingMode::ZeroPage    => format!("${:02X} = {:02X}                   ", instr_byte_two, cpu.mem_read(instr_byte_two.into())),
        AddressingMode::ZeroPage_X  => {
            let addr = instr_byte_two.wrapping_add(cpu.register_x);
            let value = cpu.mem_read(addr.into());

            format!("${:02X},X @ {:02X} = {:02X}            ", instr_byte_two, addr, value)
        },
        AddressingMode::ZeroPage_Y  => {
            let addr = instr_byte_two.wrapping_add(cpu.register_y);
            let value = cpu.mem_read(addr.into());

            format!("${:02X},Y @ {:02X} = {:02X}            ", instr_byte_two, addr, value)
        },
        AddressingMode::Indirect_X  => {
            let addr: u8 = cpu.register_x.wrapping_add(instr_byte_two);
            let lo = cpu.mem_read(addr as u16);
            let hi = cpu.mem_read(addr.wrapping_add(1) as u16);
            let target = (hi as u16) << 8 | (lo as u16);

            let value = cpu.mem_read(target);

            format!("(${:02X},X) @ {:02X} = {:04X} = {:02X}   ", instr_byte_two, addr, target, value)
        },
        AddressingMode::Indirect_Y  => {
           let addr: u16 = cpu.mem_read_u16(instr_byte_two.into()).wrapping_add(cpu.register_y.into()); 
           let value = cpu.mem_read(addr);

           format!("(${:02X}),Y = {:04X} @ {:04X} = {:02X} ", instr_byte_two, addr, addr, value)
        },

        // length 3 modes
        AddressingMode::Indirect    => {
            let addr: u16 = u16::from_le_bytes([instr_byte_two, instr_byte_three]);
            let target: u16 = cpu.mem_read_u16(addr);

            format!("(${:04X} = {:04X})             ", addr, target)
        },
        AddressingMode::Absolute    => {
            let addr: u16 = u16::from_le_bytes([instr_byte_two, instr_byte_three]);
            let value: u8 = cpu.mem_read(addr);
            format!("${:04X} = {:02X}                 ", addr, value)
        },
        AddressingMode::Absolute_X  => {
            let addr: u16 = u16::from_le_bytes([instr_byte_two, instr_byte_three])
                .wrapping_add(cpu.register_x.into());
            let target: u16 = cpu.mem_read_u16(addr);

            let value = cpu.mem_read(target);

            format!("${:04X},X @ {:04X} = {:02X}", addr, target, value)
        },
        AddressingMode::Absolute_Y  => {
            let addr: u16 = u16::from_le_bytes([instr_byte_two, instr_byte_three])
                .wrapping_add(cpu.register_y.into());
            let target: u16 = cpu.mem_read_u16(addr);

            let value = cpu.mem_read(target);

            format!("${:04X},Y @ {:04X} = {:02X}", addr, target, value)
        },
    };

    format!(
        "{:04X}  {}  {} {} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
        cpu.program_counter,
        opcode_hex,
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

