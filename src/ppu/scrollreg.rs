
/*
 * 1st write
 * 7  bit  0
 * ---- ----
 * XXXX XXXX
 * |||| ||||
 * ++++-++++- X scroll bits 7-0 (bit 8 in PPUCTRL bit 0)
 * 
 * 2nd write
 * 7  bit  0
 * ---- ----
 * YYYY YYYY
 * |||| ||||
 * ++++-++++- Y scroll bits 7-0 (bit 8 in PPUCTRL bit 1)
 */

pub struct ScrollRegister {
    x: u8,
    y: u8,
}

impl ScrollRegister {
    pub fn new() -> Self {
        ScrollRegister {
            x: 0,
            y: 0,
        }
    }

    pub fn update(&mut self, value: u8, w: bool) {
        if w {
            self.y = value;
        } else {
            self.x = value;
        }
    }

    pub fn get_pos(&self) -> (u8, u8) {
        (self.x, self.y)
    }
}
