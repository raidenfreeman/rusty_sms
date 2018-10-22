use vm::cpu::registers::Registers;
use vm::cpu::state::State;
use vm::machine::Machine;

impl Machine {
    pub(crate) fn load_into_register(
        &mut self,
        selector: fn(&mut Registers) -> (&mut u8, &mut u8),
    ) {
        let address = self.next_word();
        let (low_val, high_val) = (self.ram.read_u8(address), self.ram.read_u8(address + 1));
        {
            let (high_reg, low_reg) = selector(&mut self.cpu.state.registers);
            *high_reg = high_val;
            *low_reg = low_val;
        }
        self.clock(10);
    }

    pub(crate) fn load_into_double_register(&mut self, selector: fn(&mut State) -> &mut u16) {
        let address = self.next_word();
        let val = self.ram.read_u16(address);
        {
            let reg = selector(&mut self.cpu.state);
            *reg = val;
        }
        self.clock(10);
    }

    pub(crate) fn load_into_memory(
        &mut self,
        source: fn(&Registers) -> &u8,
        pointer: fn(&Registers) -> (&u8, &u8),
    ) {
        let value = *source(&self.cpu.state.registers);
        let (high_reg, low_reg) = pointer(&self.cpu.state.registers);
        let address = ((*high_reg as u16) << 8) | (*low_reg as u16);
        self.ram.write_u8(address, value);
    }
}
