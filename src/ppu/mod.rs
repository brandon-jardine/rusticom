use crate::rom::Mirroring;
use self::addrreg::AddressRegister;
use self::ctrlreg::{ControlFlags, ControlRegister};
use self::maskreg::{MaskFlags, MaskRegister};
use self::statusreg::StatusRegister;

pub mod addrreg;
pub mod ctrlreg;
pub mod maskreg;
pub mod statusreg;

pub struct PPU {
    pub chr_rom: Vec<u8>,
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    pub oam_data: [u8; 256],
    pub mirroring: Mirroring,
    pub addr: AddressRegister,
    pub ctrl: ControlRegister,
    pub mask: MaskRegister,
    pub status: StatusRegister,
    cycles: usize,
    scanline: u16,
    internal_data_buf: u8,
}

impl PPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        PPU {
            chr_rom,
            mirroring,
            vram: [0; 2048],
            oam_data: [0; 256],
            palette_table: [0; 32],
            cycles: 0,
            scanline: 0,
            internal_data_buf: 0,
            addr: AddressRegister::new(),
            ctrl: ControlRegister::new(),
            mask: MaskRegister::new(),
            status: StatusRegister::new(),
        }
    }

    pub fn write_to_ppu_addr(&mut self, value: u8) {
        self.addr.update(value);
    }

    pub fn write_to_ppu_ctrl(&mut self, value: u8) {
        self.ctrl.flags = ControlFlags::from_bits_truncate(value);
    }

    pub fn write_to_ppu_mask(&mut self, value: u8) {
        self.mask.flags = MaskFlags::from_bits_truncate(value);
    }

    fn inc_vram_addr(&mut self) {
        self.addr.increment(self.ctrl.vram_addr_inc());
    }

    pub fn read_data(&mut self) -> u8 {
        let addr = self.addr.get_addr();
        self.inc_vram_addr();

        match addr {
            0x0000..=0x1FFF => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.chr_rom[addr as usize];
                result
            },
            0x2000..=0x2FFF => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.vram[self.mirror_vram_addr(addr) as usize];
                result
            }
            0x3000..=0x3EFF => panic!("addr space 0x3000..0x3EFF is not expected to be used by PPU, requested = {}", addr),
            0x3F00..=0x3FFF => {
                self.palette_table[(addr - 0x3F00) as usize]
            },
            _ => panic!("Unexpected PPU access to mirrored space {}", addr),
        }
    }

    pub fn write_data(&mut self, value: u8) {
        let addr = self.addr.get_addr();
        
        match addr {
            0x0000..=0x1FFF => println!("attempt to write to chr rom space {}", addr),
            0x2000..=0x2FFF => self.vram[self.mirror_vram_addr(addr) as usize] = value,
            0x3000..=0x3EFF => unimplemented!("write to illegal PPU area {}", addr),
            0x3F00..=0x3FFF => self.palette_table[(addr - 0x3F00) as usize] = value,
            _ => panic!("Unexpected PPU access to mirrored space {}", addr),
        }

        self.inc_vram_addr();
    }

    fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let mirrored_vram = addr & 0b1011_1111_1111_1111;
        let vram_index = mirrored_vram - 0x2000;
        let name_table = vram_index / 0x0400;

        match (&self.mirroring, name_table) {
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => vram_index - 0x0800,
            (Mirroring::Horizontal, 2) => vram_index - 0x0400,
            (Mirroring::Horizontal, 1) => vram_index - 0x0400,
            (Mirroring::Horizontal, 3) => vram_index - 0x0800,
            _ => vram_index,
        }
    }

    pub fn tick(&mut self, cycles: u8) -> bool {
        self.cycles += cycles as usize;
        if self.cycles >= 341 {
            self.cycles -= 341;
            self.scanline += 1;

            if self.scanline == 241 {
                if self.ctrl.flags.contains(ControlFlags::GENERATE_NMI) {
                    self.status.set_vblank_status(true);
                    todo!("trigger NMI interrupt")
                }
            }

            if self.scanline >= 262 {
                self.scanline = 0;
                self.status.reset_vblank_status();
                return true;
            }
        }
        return false;
    }
}
