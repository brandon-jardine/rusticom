use bitflags::bitflags;
use std::collections::HashMap;
use crate::opcode;

#[cfg(test)]
mod tests;

bitflags! {
    pub struct StatusFlags: u8 {
        const CARRY             = 0b0000_0001;
        const ZERO              = 0b0000_0010;
        const INTERRUPT_DISABLE = 0b0000_0100;
        const DECIMAL_MODE      = 0b0000_1000;
        const BREAK             = 0b0001_0000;
        const BREAK2            = 0b0010_0000;
        const OVERFLOW          = 0b0100_0000;
        const NEGATIVE          = 0b1000_0000;
    }
}

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub stack_pointer: u8,
    pub status: StatusFlags,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect,
    Indirect_X,
    Indirect_Y,
    Implied,
}

trait Mem {
    fn mem_read(&self, addr: u16) -> u8;
    fn mem_write(&mut self, addr: u16, data: u8);
    fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo = self.mem_read(pos);
        let hi = self.mem_read(pos + 1);
        u16::from_le_bytes([lo, hi])
    }
    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let [lo, hi] = data.to_le_bytes();
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }
}

impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            stack_pointer: 0xff,
            status: StatusFlags::from_bits_truncate(0),
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run()
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.stack_pointer = 0xff;
        self.status = StatusFlags::from_bits_truncate(0);

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                addr
            },
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                addr
            },
            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                addr
            },
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                addr
            },
            AddressingMode::Indirect => {
                // JMP is the only 6502 instruction to support indirection.
                // The instruction contains a 16 bit address which identifies
                // the location of the least significant byte of another 16 bit
                // memory address which is the real target of the instruction.

                // An original 6502 has does not correctly fetch the target
                // address if the indirect vector falls on a page boundary
                // (e.g. $xxFF where xx is any value from $00 to $FF). In this
                // case fetches the LSB from $xxFF as expected but takes the MSB
                // from $xx00. This is fixed in some later chips like the 65SC02
                // so for compatibility always ensure the indirect vector is not
                // at the end of the page.

                let addr = self.mem_read_u16(self.program_counter);
                self.mem_read_u16(addr)
            },
            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);
                let ptr = base.wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            },
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);
                let lo = self.mem_read(base as u16);
                let hi = self.mem_read(base.wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                deref
            },
            AddressingMode::Implied => {
                panic!("mode {:?} is not supported", mode);
            },
        }
    }
    
    fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mem_value = self.mem_read(addr);
        let carry_bit = (self.status & StatusFlags::CARRY).bits();
        if self.status.contains(StatusFlags::DECIMAL_MODE) {
            // abandon hope all ye who enter here
            
            let a = self.register_a;
            let s = mem_value;
            let c = carry_bit;

            let mut al = (a & 0b0000_1111).wrapping_add(s & 0b0000_1111).wrapping_add(c);
            let mut ah = (a >> 4).wrapping_add(s >> 4).wrapping_add((al > 0b0000_1111) as u8);

            self.status.set(StatusFlags::ZERO, (a.wrapping_add(s).wrapping_add(c)) != 0);

            if al > 9 {
                al += 6;
                // ah += 1;
            }

            self.status.set(StatusFlags::NEGATIVE, (ah & 8) != 0);
            self.status.set(StatusFlags::OVERFLOW, (((ah << 4) ^ a) & 128) != 0 && ((a ^ s) & 128) == 0);

            if ah > 9 {
                ah += 6;
            }

            self.status.set(StatusFlags::CARRY, ah > 0b0000_1111);
            self.register_a = (ah << 4) | (al & 0b0000_1111);
            // self.register_a = (ah << 4) + al;
        } else {
            let (carry_in, carry_a) = self.register_a.overflowing_add(carry_bit);
            let (tmp, carry_b) = carry_in.overflowing_add(mem_value);

            self.status.set(StatusFlags::CARRY, carry_a || carry_b);
            self.status.set(StatusFlags::OVERFLOW, (self.register_a ^ tmp) & (mem_value ^ tmp) & 0x80 != 0);
            self.register_a = tmp & 0xff;
            self.update_zero_and_negative_flags(self.register_a);
        }
    }

    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);

        self.register_a &= self.mem_read(addr);
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn asl(&mut self, mode: &AddressingMode) {
        let carry = 0b1000_0000 & self.register_a == 0b1000_0000;

        match mode {
            AddressingMode::Implied => {
                self.register_a <<= 1;
                self.update_zero_and_negative_flags(self.register_a);
            },

            _ => {
                let addr = self.get_operand_address(mode);
                let value = self.mem_read(addr);
                self.mem_write(addr, value << 1);
                self.update_zero_and_negative_flags(self.mem_read(addr));
            },
        }

        self.status.set(StatusFlags::CARRY, carry);
    }

    fn lsr(&mut self, mode: &AddressingMode) {
        let carry = 1 & self.register_a == 1;

        match mode {
            AddressingMode::Implied => {
                self.register_a >>= 1;
                self.update_zero_and_negative_flags(self.register_a);
            },

            _ => {
                let addr = self.get_operand_address(mode);
                let value = self.mem_read(addr);
                self.mem_write(addr, value >> 1);
                self.update_zero_and_negative_flags(self.mem_read(addr));
            },
        }

        self.status.set(StatusFlags::CARRY, carry);
    }

    fn bcc(&mut self, mode: &AddressingMode) {
        if !self.status.contains(StatusFlags::CARRY) {
            let offset = self.mem_read(self.get_operand_address(mode));
            self.program_counter = self.program_counter.wrapping_add(offset as u16);
        }
    }

    fn bcs(&mut self, mode: &AddressingMode) {
        if self.status.contains(StatusFlags::CARRY) {
            let offset = self.mem_read(self.get_operand_address(mode));
            self.program_counter = self.program_counter.wrapping_add(offset as u16);
        }
    }

    fn bmi(&mut self, mode: &AddressingMode) {
        if self.status.contains(StatusFlags::NEGATIVE) {
            let offset = self.mem_read(self.get_operand_address(mode));
            self.program_counter = self.program_counter.wrapping_add(offset as u16);
        }
    }

    fn bne(&mut self, mode: &AddressingMode) {
        if !self.status.contains(StatusFlags::ZERO) {
            let offset = self.mem_read(self.get_operand_address(mode));
            self.program_counter = self.program_counter.wrapping_add(offset as u16);
        }
    }

    fn bpl(&mut self, mode: &AddressingMode) {
        if !self.status.contains(StatusFlags::NEGATIVE) {
            let offset = self.mem_read(self.get_operand_address(mode));
            self.program_counter = self.program_counter.wrapping_add(offset as u16);
        }
    }

    fn bvc(&mut self, mode: &AddressingMode) {
        if !self.status.contains(StatusFlags::OVERFLOW) {
            let offset = self.mem_read(self.get_operand_address(mode));
            self.program_counter = self.program_counter.wrapping_add(offset as u16);
        }
    }

    fn bvs(&mut self, mode: &AddressingMode) {
        if self.status.contains(StatusFlags::OVERFLOW) {
            let offset = self.mem_read(self.get_operand_address(mode));
            self.program_counter = self.program_counter.wrapping_add(offset as u16);
        }
    }

    fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let b6 = value & 0b0100_0000 == 0b0100_0000;
        let b7 = value & 0b1000_0000 == 0b1000_0000;

        let zero = value & self.register_a == 0;

        self.status.set(StatusFlags::ZERO, zero);
        self.status.set(StatusFlags::OVERFLOW, b6);
        self.status.set(StatusFlags::NEGATIVE, b7);
    }

    fn compare(&mut self, register: u8, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let result = register - value;

        self.status.set(StatusFlags::NEGATIVE, result & 0b1000_0000 == 0b1000_0000);
        self.status.set(StatusFlags::ZERO, value == register);
        self.status.set(StatusFlags::CARRY, register >= value);
    }

    fn cmp(&mut self, mode: &AddressingMode) {
        self.compare(self.register_a, mode);
    }

    fn cpx(&mut self, mode: &AddressingMode) {
        self.compare(self.register_x, mode);
    }

    fn cpy(&mut self, mode: &AddressingMode) {
        self.compare(self.register_y, mode);
    }

    fn dec(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr).wrapping_sub(1);

        self.mem_write(addr, value);
        self.update_zero_and_negative_flags(value);
    }

    fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_y);
    }
    
    fn eor(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = self.register_a ^ value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn jmp(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.program_counter = addr;
    }

    fn jsr(&mut self, mode: &AddressingMode) {
        self.stack_push_u16(self.program_counter + 2);
        self.program_counter = self.get_operand_address(mode);
    }

    fn rts(&mut self) {
        self.program_counter = self.stack_pull_u16();
    }

    fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr).wrapping_add(1);

        self.mem_write(addr, value);
        self.update_zero_and_negative_flags(value);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn ldx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_x = value;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn ldy(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_y = value;
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn ora(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = self.register_a | value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn rol(&mut self, mode: &AddressingMode) {
        let carry: bool;

        match mode {
            AddressingMode::Implied => {
                carry = self.register_a & 0b1000_0000 == 0b1000_0000;

                self.register_a <<= 1;
                self.register_a |= self.status.bits & StatusFlags::CARRY.bits;
                self.update_zero_and_negative_flags(self.register_a);
            },

            _ => {
                let addr = self.get_operand_address(mode);
                let mut value = self.mem_read(addr);
                carry = value & 0b1000_0000 == 0b1000_0000;

                value <<= 1;
                value |= self.status.bits & StatusFlags::CARRY.bits;
                self.mem_write(addr, value);
                self.update_zero_and_negative_flags(value);
            },
        }

        self.status.set(StatusFlags::CARRY, carry);
    }

    fn ror(&mut self, mode: &AddressingMode) {
        let carry: bool;

        match mode {
            AddressingMode::Implied => {
                carry = self.register_a & 1 == 1;

                self.register_a >>= 1;
                
                if self.status.contains(StatusFlags::CARRY) {
                    self.register_a |= 0b1000_0000;
                }

                self.update_zero_and_negative_flags(self.register_a);
            },

            _ => {
                let addr = self.get_operand_address(mode);
                let mut value = self.mem_read(addr);
                carry = value & 1 == 1;

                value >>= 1;
                value |= self.status.bits & 0b1000_0000;
                self.mem_write(addr, value);
                self.update_zero_and_negative_flags(value);
            }
        }

        self.status.set(StatusFlags::CARRY, carry);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }

    fn sty(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_y);
    }

    fn stx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_x);
    }

    fn tay(&mut self) {
        self.register_y = self.register_a;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tsx(&mut self) {
        self.register_x = self.stack_pointer;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn txa(&mut self) {
        self.register_a = self.register_x;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn txs(&mut self) {
        self.stack_pointer = self.register_x;
    }

    fn tya(&mut self) {
        self.register_a = self.register_y;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn stack_push(&mut self, data: u8) {
        let addr = u16::from_le_bytes([self.stack_pointer, 0x01]);
        self.mem_write(addr, data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn stack_push_u16(&mut self, data: u16) {
        let [lo, hi] = data.to_le_bytes();

        self.stack_push(hi);
        self.stack_push(lo);
    }

    fn stack_pull(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let addr = u16::from_le_bytes([self.stack_pointer, 0x01]);
        self.mem_read(addr)
    }

    fn stack_pull_u16(&mut self) -> u16 {
        let lo = self.stack_pull();
        let hi = self.stack_pull();

        u16::from_le_bytes([lo, hi])
    }

    fn pla(&mut self) {
        self.register_a = self.stack_pull();
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn plp(&mut self) {
        self.status = StatusFlags::from_bits_truncate(self.stack_pull());
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.status.set(StatusFlags::ZERO, result == 0);
        self.status.set(StatusFlags::NEGATIVE, result & 0b1000_0000 != 0);
    }

    pub fn run(&mut self) {
        let ref opcodes: HashMap<u8, &'static opcode::OpCode> = *opcode::OPCODES_MAP;

        loop {
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;
            let program_counter_state = self.program_counter;

            let opcode = opcodes.get(&code).expect(&format!("OpCode {:x} is not recognized", code));

            match code {
                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                    self.adc(&opcode.mode);
                },

                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                    self.and(&opcode.mode);
                },

                0x0A | 0x06 | 0x16 | 0x0E | 0x1E => {
                    self.asl(&opcode.mode);
                },

                0x90 => self.bcc(&opcode.mode),
                0xB0 => self.bcs(&opcode.mode),
                0x30 => self.bmi(&opcode.mode),
                0xD0 => self.bne(&opcode.mode),
                0x10 => self.bpl(&opcode.mode),
                0x50 => self.bvc(&opcode.mode),
                0x70 => self.bvs(&opcode.mode),

                0x24 | 0x2C => self.bit(&opcode.mode),

                0x18 => self.status.set(StatusFlags::CARRY, false),         // CLC
                0xD8 => self.status.set(StatusFlags::DECIMAL_MODE, false),  // CLD
                0x58 => self.status.set(StatusFlags::INTERRUPT_DISABLE, false), // CLI
                0xB8 => self.status.set(StatusFlags::OVERFLOW, false), // CLV

                0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                    self.cmp(&opcode.mode);
                },

                0xE0 | 0xE4 | 0xEC => {
                    self.cpx(&opcode.mode);
                },

                0xC0 | 0xC4 | 0xCC => {
                    self.cpy(&opcode.mode);
                },

                0xC6 | 0xD6 | 0xCE | 0xDE => {
                    self.dec(&opcode.mode);
                },

                0xCA => self.dex(),
                0x88 => self.dey(),

                0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => {
                    self.eor(&opcode.mode);
                },

                0x4C | 0x6C => self.jmp(&opcode.mode),
                0x20 => self.jsr(&opcode.mode),
                0x60 => self.rts(),

                0xE6 | 0xF6 | 0xEE | 0xFE => {
                    self.inc(&opcode.mode);
                },

                0xE8 => self.inx(),
                0xC8 => self.iny(),

                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    self.lda(&opcode.mode);
                },

                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
                    self.ldx(&opcode.mode);
                },

                0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
                    self.ldy(&opcode.mode);
                },

                0x4A | 0x46 | 0x56 | 0x4E | 0x5E => {
                    self.lsr(&opcode.mode);
                },

                0xEA => {
                    // NOP
                },

                0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => {
                    self.ora(&opcode.mode);
                },

                0x48 => self.stack_push(self.register_a), // PHA
                0x08 => self.stack_push(self.status.bits()), // PHP
                0x68 => self.pla(),
                0x28 => self.plp(),
                
                0x2A | 0x26 | 0x36 | 0x2E | 0x3E => {
                    self.rol(&opcode.mode);
                },

                0x6A | 0x66 | 0x76 | 0x6E | 0x7E => {
                    self.ror(&opcode.mode);
                },

                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    self.sta(&opcode.mode);
                },

                0x84 | 0x94 | 0x8C => {
                    self.sty(&opcode.mode);
                },

                0x86 | 0x96 | 0x8E => {
                    self.stx(&opcode.mode);
                },

                0x38 => self.status.set(StatusFlags::CARRY, true),
                0xF8 => self.status.set(StatusFlags::DECIMAL_MODE, true),
                0x78 => self.status.set(StatusFlags::INTERRUPT_DISABLE, true),

                0xAA => self.tax(),
                0xA8 => self.tay(),
                0xBA => self.tsx(),
                0x8A => self.txa(),
                0x9A => self.txs(),
                0x98 => self.tya(),

                0x00 => return, // BRK
                _ => todo!(),
            }

            if program_counter_state == self.program_counter {
                self.program_counter += (opcode.len - 1) as u16;
            }
        }
    }
}

