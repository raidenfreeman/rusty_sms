use vm::cpu::registers::Registers;
use vm::machine::Machine;

impl Machine {
    pub(crate) fn load_into_register(&mut self, selector: fn(&mut Registers) -> &mut u8) {
        let value = self.next_byte();
        *selector(&mut self.cpu.state.registers) = value;
        self.clock(7);
    }

    pub(crate) fn load_into_register_pair(
        &mut self,
        selector: fn(&mut Registers) -> (&mut u8, &mut u8),
    ) {
        {
            let (high_val, low_val) = self.next_byte_pair();
            let (high_reg, low_reg) = selector(&mut self.cpu.state.registers);
            *high_reg = high_val;
            *low_reg = low_val;
        }
        self.clock(10);
    }

    pub(crate) fn load_into_memory(
        &mut self,
        source: fn(&Registers) -> u8,
        pointer: fn(&Registers) -> (u8, u8),
    ) {
        {
            let value = source(&self.cpu.state.registers);
            let (high_addr, low_addr) = pointer(&self.cpu.state.registers);
            let address = ((high_addr as u16) << 8) | (low_addr as u16);
            self.ram.write_u8(address, value);
        }
        self.clock(7);
    }
}
