#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Flag {
    Carry           = 0,
    AddSubtract     = 1,
    ParityOverflow  = 2,
    Unused1         = 3,
    HalfCarry       = 4,
    Unused2         = 5,
    Zero            = 6,
    Sign            = 7
}
