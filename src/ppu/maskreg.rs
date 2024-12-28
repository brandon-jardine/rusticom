use bitflags::bitflags;

bitflags! {
    #[derive(Clone)]
    pub struct MaskFlags : u8 {
        const GREYSCALE     = 0b0000_0001;
        const SH_BKGD_LEFT  = 0b0000_0010;
        const SH_SPR_LEFT   = 0b0000_0100;
        const SH_BACKGROUND = 0b0000_1000;
        const SH_SPRITES    = 0b0001_0000;
        const EM_RED        = 0b0010_0000;
        const EM_GREEN      = 0b0100_0000;
        const EM_BLUE       = 0b1000_0000;
    }
}

pub struct MaskRegister {
    pub flags: MaskFlags,
    enabled: bool,
}

impl MaskRegister {
    pub fn new() -> Self {
        MaskRegister {
            flags: MaskFlags::from_bits_truncate(0),
            enabled: false,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn write(&mut self, value: u8) {
        self.flags = MaskFlags::from_bits_truncate(value);
    }
}

