use crate::cpu::Mem;

const RAM_START: u16 = 0x0000;
const RAM_END: u16 = 0x1FFF;
const RAM_MASK: u16 = 0b0000_0111_1111_1111;

const PPU_START: u16 = 0x2000;
const PPU_END: u16 = 0x3FFF;
const PPU_MASK: u16 = 0b0010_0000_0000_0111;

const ROM_START: u16 = 0x8000;
const ROM_END: u16 = 0xFFFF;
const ROM_MASK: u16 = 0b0111_1111_1111_1111;

pub struct Bus {
    cpu_vram: [u8; 2048],
    prg_rom: [u8; 0x7FFF],
}

impl Mem for Bus {
    fn mem_read(&self, addr: u16) -> u8 {
        match addr {
            RAM_START ..= RAM_END => {
                let mask_apply = addr & RAM_MASK;
                self.cpu_vram[mask_apply as usize]
            }

            PPU_START ..= PPU_END => {
                // let mask_apply = addr & PPU_MASK;
                todo!("PPU not implemented")
            }

            ROM_START ..= ROM_END => {
                let mask_apply = addr & ROM_MASK;
                self.prg_rom[mask_apply as usize]
            }

            _ => {
                println!("Ignoring mem read at {}", addr);
                0x00
            }
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM_START ..= RAM_END => {
                let mask_apply = addr & RAM_MASK;
                self.cpu_vram[mask_apply as usize] = data;
            }

            PPU_START ..= PPU_END => {
                todo!("PPU not implemented")
            }

            ROM_START ..= ROM_END => {
                let mask_apply = addr & ROM_MASK;
                self.prg_rom[mask_apply as usize] = data;
            }

            _ => {
                println!("Ignoring mem write at {}", addr);
            }
        }
    }
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            cpu_vram: [0; 2048],
            prg_rom: [0; 0x7FFF],
        }
    }
}

