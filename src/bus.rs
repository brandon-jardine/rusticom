use crate::mem::Mem;
use crate::ppu::PPU;
use crate::rom::Rom;

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
    cycles: usize,
    prg_rom: Vec<u8>,
    ppu: PPU,
    pub allow_rom_writes: bool,
}

impl Mem for Bus {
    fn mem_read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM_START ..= RAM_END => {
                let mask_apply = addr & RAM_MASK;
                self.cpu_vram[mask_apply as usize]
            },

            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                panic!("Attempt to read from write-only PPU address {:x}", addr);
            },

            0x2002 => self.ppu.read_status(),

            0x2004 => self.ppu.read_oam_data(),

            0x2007 => self.ppu.read_data(),

            0x2008 ..= PPU_END => {
                let mask_apply = addr & PPU_MASK;
                self.mem_read(mask_apply)
            },

            ROM_START ..= ROM_END => {
                let mut mask_apply = addr & ROM_MASK;

                if self.prg_rom.len() == 0x4000 && addr >= 0x4000 {
                    mask_apply = mask_apply % 0x4000;
                }

                self.prg_rom[mask_apply as usize]
            },

            _ => {
                println!("Ignoring mem read at {}", addr);
                0x00
            },
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM_START ..= RAM_END => {
                let mask_apply = addr & RAM_MASK;
                self.cpu_vram[mask_apply as usize] = data;
            },

            0x2000 => self.ppu.write_to_ppu_ctrl(data),
            0x2001 => self.ppu.write_to_ppu_mask(data),

            0x2003 => self.ppu.oam_addr = data,

            0x2005 => self.ppu.write_to_ppu_scroll(data),
            0x2006 => self.ppu.write_to_ppu_addr(data),
            0x2007 => self.ppu.write_data(data),

            0x2008 ..= PPU_END => {
                let mask_apply = addr & PPU_MASK;
                self.mem_write(mask_apply, data);
            },

            0x4014 => {
                let oam_page: u16 = (data as u16) << 8;
                let idx = oam_page as usize;

                let oam_slice = &self.cpu_vram[idx..(idx+256)];
                self.ppu.oam_data.copy_from_slice(oam_slice);

                //self.tick(513);
                todo!("should take 513 or 514 cycles");
            }

            ROM_START ..= ROM_END => {
                if self.allow_rom_writes {
                    let mask_apply = addr & ROM_MASK;
                    self.prg_rom[mask_apply as usize] = data;
                } else {
                    panic!("Attempted to write to ROM!");
                }
            },

            _ => {
                println!("Ignoring mem write at {}", addr);
            },
        }
    }
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        let ppu = PPU::new(rom.chr_rom, rom.screen_mirroring);

        Bus {
            cpu_vram: [0; 2048],
            cycles: 0,
            prg_rom: rom.prg_rom,
            ppu,
            allow_rom_writes: false,
        }
    }

    pub fn poll_nmi_status() -> bool {
        todo!("Get nmi status from ppu");
    }

    pub fn tick(&mut self, cycles: u8) {
        self.cycles += cycles as usize;
        self.ppu.tick(cycles * 3);
    }
}

