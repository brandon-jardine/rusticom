use bitflags::bitflags;

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
        if self.flags.contains(ControlFlags::VRAM_ADDR_INC) {
            32
        } else {
            1
        }
    }
}

