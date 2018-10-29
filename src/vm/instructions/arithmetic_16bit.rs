use vm::cpu::flags::Flag;
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

    fn operate_on_register_pair(&mut self, selector: fn(&Registers) -> (u8, u8)) {
        let op1 = {
            let high = self.cpu.state.registers.h;
            let low = self.cpu.state.registers.l;
            Registers::u8s_to_u16(high, low)
        };
        let op2 = {
            let (high, low) = selector(&self.cpu.state.registers);
            Registers::u8s_to_u16(high, low)
        };
        let result = (op1 as u32) + (op2 as u32);
        let carry = (result & 0x10000) != 0;
        Flag::AddSubtract.set(&mut self.cpu.state.status, false);
        Flag::Carry.set(&mut self.cpu.state.status, carry);
        self.clock(11);
    }
}
