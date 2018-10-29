use vm::cpu::registers::Registers;
use vm::machine::Machine;

impl Machine {
    pub(crate) fn increment_register_wide(
        &mut self,
        target_first: fn(&mut Registers) -> &mut u8,
        target_second: fn(&mut Registers) -> &mut u8,
    ) {
        let op1 = *target_first(&mut self.cpu.state.registers) as u16;
        let op2 = *target_second(&mut self.cpu.state.registers) as u16;

        let inc = ((op1 << 0x08) | op2) + 0x01;
        let result_left = (inc >> 0x08) as u8;
        let result_right = inc as u8;

        *target_first(&mut self.cpu.state.registers) = result_left;
        *target_second(&mut self.cpu.state.registers) = result_right;

        self.clock(6);
    }

    pub(crate) fn decrement_register_wide(
        &mut self,
        target_first: fn(&mut Registers) -> &mut u8,
        target_second: fn(&mut Registers) -> &mut u8,
    ) {
        let op1 = *target_first(&mut self.cpu.state.registers) as u16;
        let op2 = *target_second(&mut self.cpu.state.registers) as u16;

        let inc = ((op1 << 0x08) | op2) - 0x01;
        let result_left = (inc >> 0x08) as u8;
        let result_right = inc as u8;

        *target_first(&mut self.cpu.state.registers) = result_left;
        *target_second(&mut self.cpu.state.registers) = result_right;

        self.clock(6);
    }
}
