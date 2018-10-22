use vm::cpu::registers::Registers;
use vm::machine::Machine;

impl Machine {
    pub(crate) fn push_to_stack(&mut self, selector: fn(&Registers) -> (u8, u8)) {
        let (op1, op2) = selector(&self.cpu.state.registers);
        let sp_high = self.cpu.state.registers.s as u16;
        let sp_low = self.cpu.state.registers.p as u16;
        let sp = sp_high << 8 | sp_low;
        self.ram.write_u8(sp - 1, op1);
        self.ram.write_u8(sp - 2, op2);
        self.cpu.state.registers.s = (((sp - 2) | 0xFF00) >> 8) as u8;
        self.cpu.state.registers.p = ((sp - 2) | 0x00FF) as u8;
        self.clock(11);
    }
}
