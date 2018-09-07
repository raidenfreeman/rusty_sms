#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Opcode {
    Nop         = 0x00,
    LdBCXX      = 0x01,
    LdVBCA      = 0x02,
    IncBC       = 0x03,
    IncB        = 0x04,
    DecB        = 0x05,
    IncC        = 0x0c,
    DecC        = 0x0d,
    LdDEXX      = 0x11, 
    LdVDEA      = 0x12,        
    IncD        = 0x14,
    DecD        = 0x15,
    IncE        = 0x1c,
    DecE        = 0x1d,
    LdHLXX      = 0x21,
    LdVXXHL     = 0x22,
    IncH        = 0x24,
    DecH        = 0x25,
    IncL        = 0x2c,
    DecL        = 0x2d,
    LdSPXX      = 0x31,
    LdVXXA      = 0x32,
    IncA        = 0x3c,
    DecA        = 0x3d,
    Halt        = 0x76
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        unsafe {
            ::std::mem::transmute_copy::<u8, Opcode>(&value)
        }
    }
}
