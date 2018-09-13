use vm::cpu::flags::Flag;
use vm::cpu::registers::Registers;

pub struct State {
    pub registers: Registers,
    pub alt_registers: Registers,
    pub program_counter: u16,
    pub stack_pointer: u16,
    pub status: u8,
}

impl State {
    pub fn new() -> State {
        State {
            registers: Registers::new(),
            alt_registers: Registers::new(),
            program_counter: 0,
            stack_pointer: 0,
            status: 0,
        }
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) -> () {
        let binary_value = if value { 1 } else { 0 } as u8;
        let mask = binary_value << (flag as u8);
        if value {
            self.status |= mask;
        } else {
            self.status &= !mask;
        };
    }
}
