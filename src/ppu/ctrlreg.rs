use bitflags::bitflags;

/*
 * 7  bit  0
 * ---- ----
 * VPHB SINN
 * |||| ||||
 * |||| ||++- Base nametable address
 * |||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
 * |||| |+--- VRAM address increment per CPU read/write of PPUDATA
 * |||| |     (0: add 1, going across; 1: add 32, going down)
 * |||| +---- Sprite pattern table address for 8x8 sprites
 * ||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
 * |||+------ Background pattern table address (0: $0000; 1: $1000)
 * ||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels – see PPU OAM#Byte 1)
 * |+-------- PPU master/slave select
 * |          (0: read backdrop from EXT pins; 1: output color on EXT pins)
 * +--------- Vblank NMI enable (0: off, 1: on)
 */

bitflags! {
    #[derive(Clone)]
    pub struct ControlFlags : u8 {
        const NAMETABLE1        = 0b0000_0001;
        const NAMETABLE2        = 0b0000_0010;
        const VRAM_ADDR_INC     = 0b0000_0100;
        const SPRITE_PTRN_ADDR  = 0b0000_1000;
        const BGRND_PTRN_ADDR   = 0b0001_0000;
        const SPRITE_SIZE       = 0b0010_0000;
        const MASTER_SLAVE      = 0b0100_0000;
        const GENERATE_NMI      = 0b1000_0000;
    }
}

pub struct ControlRegister {
    pub flags: ControlFlags,
}

impl ControlRegister {
    pub fn new() -> Self {
        ControlRegister {
            flags: ControlFlags::from_bits_truncate(0),
        }
    }

    pub fn vram_addr_inc(&self) -> u8 {
        match self.flags.contains(ControlFlags::VRAM_ADDR_INC) {
            true => 32,
            _ => 1,
        }
    }
}

