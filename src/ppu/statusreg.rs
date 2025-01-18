use bitflags::bitflags;

/*
 * 7  bit  0
 * ---- ----
 * VSOx xxxx
 * |||| ||||
 * |||+-++++- (PPU open bus or 2C05 PPU identifier)
 * ||+------- Sprite overflow flag
 * |+-------- Sprite 0 hit flag
 * +--------- Vblank flag, cleared on read. Unreliable; see below.
 */

bitflags! {
    #[derive(Clone)]
    pub struct StatusRegister : u8 {
        const UNUSED0   = 0b0000_0001;
        const UNUSED1   = 0b0000_0010;
        const UNUSED2   = 0b0000_0100;
        const UNUSED3   = 0b0000_1000;
        const UNUSED4   = 0b0001_0000;
        const SPRITE_OVERFLOW   = 0b0010_0000;
        const SPRITE_ZERO_HIT   = 0b0100_0000;
        const VBLANK_FLAG       = 0b1000_0000;
    }
}

impl StatusRegister {
    pub fn new() -> Self {
        StatusRegister::from_bits_truncate(0)
    }

    pub fn set_vblank_status(&mut self, vblank: bool) {
        self.set(StatusRegister::VBLANK_FLAG, vblank);
    }

    pub fn reset_vblank_status(&mut self) {
        self.set(StatusRegister::VBLANK_FLAG, false);
    }
}

