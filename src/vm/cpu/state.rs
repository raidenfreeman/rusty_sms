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
}
