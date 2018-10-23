use vm::cpu::registers::Registers;
use vm::machine::Machine;

impl Machine {
    pub(crate) fn push_to_stack(&mut self, selector: fn(&Registers) -> (u8, u8)) {
        let (op1, op2) = selector(&self.cpu.state.registers);
        let sp = Registers::u8s_to_u16(self.cpu.state.registers.s, self.cpu.state.registers.p);
        self.ram.write_u8(sp - 1, op1);
        self.ram.write_u8(sp - 2, op2);
        let (s, p) = Registers::u16_to_u8s(sp - 2);
        self.cpu.state.registers.s = s;
        self.cpu.state.registers.p = p;
        self.clock(11);
    }

    pub(crate) fn pop_from_stack(&mut self, selector: fn(&mut Registers) -> (&mut u8, &mut u8)) {
        let sp = Registers::u8s_to_u16(self.cpu.state.registers.s, self.cpu.state.registers.p);
        let high_val = self.ram.read_u8(sp);
        let low_val = self.ram.read_u8(sp + 1);
        {
            let (high_reg, low_reg) = selector(&mut self.cpu.state.registers);
            *high_reg = high_val;
            *low_reg = low_val;
        }
        let (s, p) = Registers::u16_to_u8s(sp + 2);
        self.cpu.state.registers.s = s;
        self.cpu.state.registers.p = p;
        self.clock(10);
    }
}
