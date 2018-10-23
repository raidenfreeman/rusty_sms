pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub s: u8,
    pub p: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            s: 0,
            p: 0,
        }
    }

    pub fn u8s_to_u16(first: u8, second: u8) -> u16 {
        ((first as u16) << 0x08) | (second as u16)
    }

    pub fn u16_to_u8s(value: u16) -> (u8, u8) {
        let first = ((value | 0xFF00) >> 8) as u8;
        let second = (value | 0x00FF) as u8;
        (first, second)
    }
}
