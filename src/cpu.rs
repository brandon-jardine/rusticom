use bitflags::bitflags;
use std::collections::HashMap;

use crate::bus::Bus;
use crate::mem::Mem;
use crate::opcode;

#[cfg(test)]
mod tests;

bitflags! {
    #[derive(Clone)]
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

const STACK: u16 = 0x0100;
const STACK_RESET: u8 = 0xFD;
const STATUS_RESET: StatusFlags = StatusFlags::from_bits_truncate(0b0010_0100);

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub stack_pointer: u8,
    pub status: StatusFlags,
    pub program_counter: u16,
    pub bus: Bus,
    pub enable_decimal: bool,
    pause: bool,
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
    None,
}

impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.bus.mem_read(addr)
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.bus.mem_write(addr, data)
    }

    fn mem_read_u16(&self, pos: u16) -> u16 {
        self.bus.mem_read_u16(pos)
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        self.bus.mem_write_u16(pos, data)
    }
}

impl CPU {
    pub fn new(bus: Bus) -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            stack_pointer: STACK_RESET,
            status: STATUS_RESET,
            program_counter: 0,
            bus,
            enable_decimal: false,
            pause: false,
        }
    }

    pub fn toggle_pause(&mut self) {
        self.pause = !self.pause;
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run()
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.load_at(program, 0x8000);
    }

    pub fn load_at(&mut self, program: Vec<u8>, addr: u16) {
        for i in 0..program.len() as u16 {
            self.mem_write(addr + i, program[i as usize]);
        }
        self.mem_write_u16(0xFFFC, addr);
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.stack_pointer = STACK_RESET;
        self.status = STATUS_RESET;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn resolve_address(&self, mode: &AddressingMode, base: u16) -> u16 {
        match mode {
            AddressingMode::ZeroPage => self.mem_read(base) as u16,
            AddressingMode::Absolute => self.mem_read_u16(base),
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(base);
                let addr = pos.wrapping_add(self.register_x) as u16;
                addr
            },
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(base);
                let addr = pos.wrapping_add(self.register_y) as u16;
                addr
            },
            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(base);
                let addr = base.wrapping_add(self.register_x as u16);
                addr
            },
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(base);
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

                let addr = self.mem_read_u16(base);
                self.mem_read_u16(addr)
            },
            AddressingMode::Indirect_X => {
                let base_addr = self.mem_read(base);
                let ptr = base_addr.wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            },
            AddressingMode::Indirect_Y => {
                let base_addr = self.mem_read(base);
                let lo = self.mem_read(base_addr as u16);
                let hi = self.mem_read(base_addr.wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                deref
            },
            _ => {
                panic!("mode {:?} is not supported", mode);
            },
        }
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            _ => self.resolve_address(mode, self.program_counter),
        }
    }
    
    fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mem_value = self.mem_read(addr);

        if self.enable_decimal && self.status.contains(StatusFlags::DECIMAL_MODE) {
            self.bcd_add(mem_value);
        } else {
            self.binary_add(mem_value);
        }

        self.update_zero_and_negative_flags(self.register_a);
    }

    fn sbc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mem_value = self.mem_read(addr);

        if self.enable_decimal && self.status.contains(StatusFlags::DECIMAL_MODE) {
            self.bcd_sub(mem_value);
        } else {
            self.binary_add(!mem_value);
        }

        self.update_zero_and_negative_flags(self.register_a);
    }

    fn binary_add(&mut self, arg: u8) {
        let carry_bit = StatusFlags::CARRY.bits() & self.status.bits();
        let (carry_in, carry_a) = self.register_a.overflowing_add(carry_bit);
        let (tmp, carry_b) = carry_in.overflowing_add(arg);
        
        self.status.set(StatusFlags::CARRY, carry_a || carry_b);
        self.status.set(StatusFlags::OVERFLOW, (self.register_a ^ tmp) & (arg ^ tmp) & 0x80 != 0);
        self.register_a = tmp;
    }

    fn bcd_add(&mut self, arg: u8) {
        let carry_bit = StatusFlags::CARRY.bits() & self.status.bits();
        // abandon hope all ye who enter here
        
        let a = self.register_a as u16;
        let v = arg as u16;

        // calculate lower nibble
        let mut tmp = (a & 0x0F) + (v & 0x0F) + (carry_bit as u16);

        // correct lower nibble if out of BDC range
        if tmp > 0x09 {
            tmp = ((tmp + 0x06) & 0x0F) + 0x10;
        }

        // add in high nibbles
        tmp += (a & 0xF0).wrapping_add(v & 0xF0);
        
        // overflow is calculated before the upper nibble is corrected
        let o = (!(a ^ v) & (a ^ tmp)) & 0x80 != 0;
        self.status.set(StatusFlags::OVERFLOW, o);

        // correct high nibble if out of BDC range
        if tmp > 0x90 {
            tmp += 0x60;
        }

        self.status.set(StatusFlags::CARRY, tmp > 99);
        self.register_a = (tmp & 0xFF) as u8;
    }

    fn bcd_sub(&mut self, arg: u8) {
        let carry_bit = self.status.contains(StatusFlags::CARRY) as u16;
        let n_carry_bit = !carry_bit & 0x01;

        let a = self.register_a as u16;
        let v = arg as u16;

        // calculate lower nibble
        let mut lo = (a & 0x0F).wrapping_sub(v & 0x0F).wrapping_sub(n_carry_bit);

        // correct lower nibble if outside bcd range1
        if lo & 0x10 == 0x10 {
            lo -= 0x06;
        }

        let mut hi = (a >> 4).wrapping_sub(v >> 4).wrapping_sub((lo & 0x10 == 0x10) as u16);

        if hi & 0x10 == 0x10 {
            hi -= 0x06;
        }

        self.status.set(StatusFlags::CARRY, a.wrapping_sub(v).wrapping_sub(n_carry_bit) & 0x0F00 != 0);
        self.status.set(StatusFlags::OVERFLOW, (a.wrapping_sub(v).wrapping_sub(n_carry_bit) ^ v) & 0x80 == 0x80 && (a ^ v) & 0x80 == 0x80);

        self.register_a = ((hi << 4) | (lo & 0xF)) as u8;
    }

    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);

        self.register_a &= self.mem_read(addr);
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn asl(&mut self, mode: &AddressingMode) {

        match mode {
            AddressingMode::None => {
                let carry = 0b1000_0000 & self.register_a == 0b1000_0000;
                self.register_a <<= 1;
                self.update_zero_and_negative_flags(self.register_a);
                self.status.set(StatusFlags::CARRY, carry);
            },

            _ => {
                let addr = self.get_operand_address(mode);
                let value = self.mem_read(addr);
                let carry = 0b1000_0000 & value == 0b1000_0000;
                self.mem_write(addr, value << 1);
                self.update_zero_and_negative_flags(self.mem_read(addr));
                self.status.set(StatusFlags::CARRY, carry);
            },
        }
    }

    fn lsr(&mut self, mode: &AddressingMode) {
        match mode {
            AddressingMode::None => {
                let carry = 1 & self.register_a == 1;
                self.register_a >>= 1;
                self.update_zero_and_negative_flags(self.register_a);
                self.status.set(StatusFlags::CARRY, carry);
            },

            _ => {
                let addr = self.get_operand_address(mode);
                let value = self.mem_read(addr);
                let carry = 1 & value == 1;
                self.mem_write(addr, value >> 1);
                self.update_zero_and_negative_flags(self.mem_read(addr));
                self.status.set(StatusFlags::CARRY, carry);
            },
        }
    }

    fn branch(&mut self, condition: bool) {
        if condition {
            let jump: i8 = self.mem_read(self.program_counter) as i8;
            let jump_addr = self.program_counter
                .wrapping_add(1)
                .wrapping_add(jump as u16);

            self.program_counter = jump_addr;
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
        let result = register.wrapping_sub(value); 
        self.status.set(StatusFlags::CARRY, register >= value);
        self.status.set(StatusFlags::ZERO, value == register);
        self.status.set(StatusFlags::NEGATIVE, result & 0b1000_0000 == 0b1000_0000);
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

    fn jsr(&mut self) {
       self.stack_push_u16(self.program_counter + 2 - 1);
       self.program_counter = self.mem_read_u16(self.program_counter);
    }

    fn rts(&mut self) {
        self.program_counter = self.stack_pop_u16() + 1;
    }

    fn rti(&mut self) {
        self.status = StatusFlags::from_bits_truncate(self.stack_pop());
        self.status.remove(StatusFlags::BREAK);
        self.status.insert(StatusFlags::BREAK2);

        self.program_counter = self.stack_pop_u16();
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
        let old_carry: bool = self.status.contains(StatusFlags::CARRY);
        let carry: bool;

        match mode {
            AddressingMode::None => {
                carry = self.register_a & 0b1000_0000 == 0b1000_0000;

                self.register_a <<= 1;

                if old_carry {
                    self.register_a |= 1;
                }
                self.update_zero_and_negative_flags(self.register_a);
            },

            _ => {
                let addr = self.get_operand_address(mode);
                let mut value = self.mem_read(addr);
                carry = value & 0b1000_0000 == 0b1000_0000;

                value <<= 1;

                if old_carry {
                    value |= 1;
                }

                self.mem_write(addr, value);
                self.update_zero_and_negative_flags(value);
            },
        }

        self.status.set(StatusFlags::CARRY, carry);
    }

    fn ror(&mut self, mode: &AddressingMode) {
        let old_carry: bool = self.status.contains(StatusFlags::CARRY);
        let carry: bool;

        match mode {
            AddressingMode::None => {
                carry = self.register_a & 1 == 1;

                self.register_a >>= 1;
                
                if old_carry {
                    self.register_a |= 0b1000_0000;
                }

                self.update_zero_and_negative_flags(self.register_a);
            },

            _ => {
                let addr = self.get_operand_address(mode);
                let mut value = self.mem_read(addr);
                carry = value & 1 == 1;

                value >>= 1;

                if old_carry {
                    value |= 0b1000_0000;
                }
                
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
        self.mem_write(STACK + self.stack_pointer as u16, data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn stack_push_u16(&mut self, data: u16) {
        let [lo, hi] = data.to_le_bytes();

        self.stack_push(hi);
        self.stack_push(lo);
    }

    fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.mem_read(STACK.wrapping_add(self.stack_pointer as u16))
    }

    fn stack_pop_u16(&mut self) -> u16 {
        let lo = self.stack_pop();
        let hi = self.stack_pop();

        u16::from_le_bytes([lo, hi])
    }

    fn php(&mut self) {
        let mut flags = self.status.clone();
        flags.insert(StatusFlags::BREAK);
        flags.insert(StatusFlags::BREAK2);
        self.stack_push(flags.bits());
    }

    fn pla(&mut self) {
        self.register_a = self.stack_pop();
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn plp(&mut self) {
        self.status = StatusFlags::from_bits_truncate(self.stack_pop());
        self.status.remove(StatusFlags::BREAK);
        self.status.insert(StatusFlags::BREAK2);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.status.set(StatusFlags::ZERO, result == 0);
        self.status.set(StatusFlags::NEGATIVE, result & 0b1000_0000 != 0);
    }

    pub fn run(&mut self) {
        self.run_with_callback(|_| {});
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F) where F: FnMut(&mut CPU) {
        let ref opcodes: HashMap<u8, &'static opcode::OpCode> = *opcode::OPCODES_MAP;

        loop {
            callback(self);

            if self.pause {
                continue;
            }

            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;
            let program_counter_state = self.program_counter;

            let opcode = opcodes.get(&code).expect(&format!("OpCode {:x} is not recognized", code));

            match code {
                // OFFICIAL OPCODES

                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                    self.adc(&opcode.mode);
                },

                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                    self.and(&opcode.mode);
                },

                0x0A | 0x06 | 0x16 | 0x0E | 0x1E => {
                    self.asl(&opcode.mode);
                },

                0x90 => self.branch(!self.status.contains(StatusFlags::CARRY)),     // BCC
                0xB0 => self.branch(self.status.contains(StatusFlags::CARRY)),      // BCS
                0xF0 => self.branch(self.status.contains(StatusFlags::ZERO)),       // BEQ
                0x30 => self.branch(self.status.contains(StatusFlags::NEGATIVE)),   // BMI
                0xD0 => self.branch(!self.status.contains(StatusFlags::ZERO)),      // BNE
                0x10 => self.branch(!self.status.contains(StatusFlags::NEGATIVE)),  // BPL
                0x50 => self.branch(!self.status.contains(StatusFlags::OVERFLOW)),  // BVC
                0x70 => self.branch(self.status.contains(StatusFlags::OVERFLOW)),   // BVS

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

                0x6C => {
                    // JMP indirect
                    let mem_addr = self.mem_read_u16(self.program_counter);
                    //6502 bug mode with with page boundary:
                    //  if address $3000 contains $40, $30FF contains $80, and $3100 contains $50,
                    // the result of JMP ($30FF) will be a transfer of control to $4080 rather than $5080 as you intended
                    // i.e. the 6502 took the low byte of the address from $30FF and the high byte from $3000
                    let indirect_ref = if mem_addr & 0x00FF == 0x00FF {
                        let lo = self.mem_read(mem_addr);
                        let hi = self.mem_read(mem_addr & 0xFF00);
                        u16::from_le_bytes([lo, hi])
                    } else {
                        self.mem_read_u16(mem_addr)
                    };

                    self.program_counter = indirect_ref;
                },

                0x4C => {
                    // JMP absolute
                    self.program_counter = self.mem_read_u16(self.program_counter);
                },

                0x20 => self.jsr(),
                0x60 => self.rts(),

                0x40 => self.rti(),

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
                0x08 => self.php(), 
                0x68 => self.pla(),
                0x28 => self.plp(),
                
                0x2A | 0x26 | 0x36 | 0x2E | 0x3E => {
                    self.rol(&opcode.mode);
                },

                0x6A | 0x66 | 0x76 | 0x6E | 0x7E => {
                    self.ror(&opcode.mode);
                },

                0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => {
                    self.sbc(&opcode.mode);
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

                // UN-OFFICIAL OPCODES

                0x1A | 0x3A | 0x5A | 0x7A | 0xDA | 0xFA => {
                    // NOP
                },

                0x80 | 0x82 | 0x89 | 0xC2 | 0xE2 => {
                    // NOP
                },

                0x0C | 0x1C | 0x3C | 0x5C | 0x7C |
                0xDC | 0xFC | 0x04 | 0x44 | 0x64 |
                0x14 | 0x34 | 0x54 | 0x74 | 0xD4 | 0xF4 => {
                    // NOP
                },

                0xA3 | 0xA7 | 0xAF |
                0xB3 | 0xB7 | 0xBF => {
                    // LAX
                    self.lda(&opcode.mode);
                    self.tax();
                },

                0x83 | 0x87 | 0x8F | 0x97 => {
                    // SAX
                    let value = self.register_a & self.register_x;
                    self.mem_write(self.get_operand_address(&opcode.mode), value);
                },

                // Duplicated SBC 
                0xEB => self.sbc(&opcode.mode),

                // DCP
                0xC3 | 0xC7 | 0xCF | 0xD3 |
                0xD7 | 0xDB | 0xDF => {
                    self.dec(&opcode.mode);
                    self.cmp(&opcode.mode);
                },

                // ISC (ISB)
                0xE3 | 0xE7 | 0xEF | 0xF3 |
                0xF7 | 0xFB | 0xFF => {
                    self.inc(&opcode.mode);
                    self.sbc(&opcode.mode);
                },

                // SLO
                0x03 | 0x07 | 0x0F | 0x13 |
                0x17 | 0x1B | 0x1F => {
                    self.asl(&opcode.mode);
                    self.ora(&opcode.mode);
                },

                // RLA
                0x23 | 0x27 | 0x2F | 0x33 |
                0x37 | 0x3B | 0x3F => {
                    self.rol(&opcode.mode);
                    self.and(&opcode.mode);
                },

                // SRE
                0x43 | 0x47 | 0x4F | 0x53 |
                0x57 | 0x5B | 0x5F => {
                    self.lsr(&opcode.mode);
                    self.eor(&opcode.mode);
                },

                // RRA
                0x63 | 0x67 | 0x6F | 0x73 |
                0x77 | 0x7B | 0x7F => {
                    self.ror(&opcode.mode);
                    self.adc(&opcode.mode);
                },

                // ALR
                0x4B => {
                    self.and(&opcode.mode);
                    self.lsr(&AddressingMode::None);
                },

                // ANC
                0x0B | 0x2B => {
                    self.and(&opcode.mode);
                    self.status.set(StatusFlags::CARRY, self.status.contains(StatusFlags::NEGATIVE));
                },

                // ARR
                0x6B => {
                    self.and(&opcode.mode);
                    self.ror(&AddressingMode::None);

                    let (five, six) = (
                        self.register_a & 0b0010_0000 == 0b0010_0000,
                        self.register_a & 0b0100_0000 == 0b0100_0000,
                    );

                    self.status.set(StatusFlags::OVERFLOW, five != six);
                    self.status.set(StatusFlags::CARRY, six);
                },

                // AXS
                0xCB => {
                    let data = self.mem_read(self.get_operand_address(&opcode.mode));
                    let bitwise_and = self.register_a & self.register_x;

                    if data <= bitwise_and {
                        self.status.insert(StatusFlags::CARRY);
                    }
                    
                    let result = bitwise_and.wrapping_sub(data);
                    self.update_zero_and_negative_flags(result);

                    self.register_x = result;
                },

                // ATX
                0xAB => {
                    self.and(&opcode.mode);
                    self.register_x = self.register_a;
                },

                // AXA
                0x9F | 0x93 => {
                    // Conflicting info on what this one does
                    let addr = self.get_operand_address(&opcode.mode);
                    let value = self.register_a & self.register_x & (addr >> 8) as u8;
                    self.mem_write(addr, value);
                },

                // SXA
                0x9E => {
                    let addr = self.get_operand_address(&opcode.mode);
                    let value = self.register_x & (addr >> 8) as u8 + 1;
                    self.mem_write(addr, value);
                },

                // SYA
                0x9C => {
                    let addr = self.get_operand_address(&opcode.mode);
                    let value = self.register_y & (addr >> 8) as u8 + 1;
                    self.mem_write(addr, value);
                },

                // XAS
                0x9B => {
                    let addr = self.get_operand_address(&opcode.mode);
                    self.stack_pointer = self.register_a & self.register_x;
                    let value = self.stack_pointer & (addr >> 8) as u8 + 1;
                    self.mem_write(addr, value);
                },

                // HLT
                0x02 | 0x12 | 0x22 | 0x32 | 0x42 |
                0x52 | 0x62 | 0x72 | 0x92 | 0xB2 |
                0xD2 | 0xF2 => {
                    panic!("Illegal HLT instruction received.");
                },

                // LAR
                0xBB => {
                    let addr = self.get_operand_address(&opcode.mode);
                    let data = self.mem_read(addr) & self.stack_pointer;
                    self.register_a = data;
                    self.register_x = data;
                    self.stack_pointer = data;
                    self.update_zero_and_negative_flags(data);
                },

                // XAA
                0x8B => {
                    self.register_a = self.register_x;
                    self.register_a &= self.mem_read(self.get_operand_address(&opcode.mode));
                },

                0x00 => return, // BRK
            }

            if program_counter_state == self.program_counter {
                self.program_counter += (opcode.len - 1) as u16;
            }
        }
    }
}

