pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8
}

impl Registers {
    pub fn new() -> Registers { 
        Registers { a: 0, b: 0, c: 0, d: 0, e: 0, f: 0, h: 0, l: 0 }
    }
}

