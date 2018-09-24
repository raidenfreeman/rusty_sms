#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Flag {
    Carry = 1,
    AddSubtract = 2,
    ParityOverflow = 4,
    Unused1 = 8,
    HalfCarry = 16,
    Unused2 = 32,
    Zero = 64,
    Sign = 128,
}

impl Flag {
    pub fn set(self, register: &mut u8, value: bool) {
        let mask = self as u8;
        if value {
            *register |= mask;
        } else {
            *register &= !mask;
        };
    }

    pub fn get(self, register: &u8) -> bool {
        self.get_bit(register) > 0
    }

    pub fn get_bit(self, register: &u8) -> u8 {
        let mask = self as u8;
        *register & mask
}
