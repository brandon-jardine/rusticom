const ADDR_MIRROR_MASK: u16 = 0b0011_1111_1111_1111;

pub struct AddressRegister {
    hi_value: u8,
    lo_value: u8,
    hi_ptr: bool,
}

impl AddressRegister {
    pub fn new() -> Self {
        AddressRegister {
            hi_value: 0,
            lo_value: 0,
            hi_ptr: true,
        }
    }

    fn set(&mut self, addr: u16) {
    self.hi_value = (addr >> 8) as u8;
        self.lo_value = (addr & 0xFF) as u8;
    }

    pub fn value(&self) -> (u8, u8) {
        (self.hi_value, self.lo_value)
    }

    pub fn update(&mut self, addr_part: u8) {
        if self.hi_ptr {
            self.hi_value = addr_part;
        } else {
            self.lo_value = addr_part;
        }

        if self.get_addr() > 0x3FFF {
            self.set(self.get_addr() & ADDR_MIRROR_MASK);
        }

        self.hi_ptr = !self.hi_ptr;
    }

    pub fn increment(&mut self, inc: u8) {
        let (lo, overflow) = self.lo_value.overflowing_add(inc);
        self.lo_value = lo;

        if overflow {
            self.hi_value = self.hi_value.wrapping_add(1);
        }

        if self.get_addr() > 0x3FFF {
            self.set(self.get_addr() & ADDR_MIRROR_MASK);
        }
    }

    pub fn reset(&mut self) {
        self.hi_ptr = true;
    }

    pub fn get_addr(&self) -> u16 {
        ((self.hi_value as u16) << 8) | self.lo_value as u16
    }
}

