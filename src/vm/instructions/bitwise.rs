use vm::cpu::flags::Flag;
use vm::cpu::registers::Registers;
use vm::machine::Machine;

impl Machine {
    pub(crate) fn and_register(&mut self, selector: fn(&Registers) -> u8) {
        self.bitwise_operation(selector, |a, b| a & b, true);
        self.clock(4);
    }

    pub(crate) fn or_register(&mut self, selector: fn(&Registers) -> u8) {
        self.bitwise_operation(selector, |a, b| a | b, false);
        self.clock(4);
    }

    pub(crate) fn xor_register(&mut self, selector: fn(&Registers) -> u8) {
        self.bitwise_operation(selector, |a, b| a ^ b, false);
        self.clock(4);
    }

    pub(crate) fn bitwise_operation(
        &mut self,
        operand: fn(&Registers) -> u8,
        operation: fn(u8, u8) -> u8,
        half_carry_value: bool,
    ) {
        let op1 = self.cpu.state.registers.a;
        let op2 = operand(&self.cpu.state.registers);
        let result = operation(op1, op2);
        let parity = (0..8).fold(0, |acc, b| acc + (result >> b) & 1) % 2 == 0;

        let status = &mut self.cpu.state.status;
        Flag::ParityOverflow.set(status, parity);
        Flag::Carry.set(status, false);
        Flag::HalfCarry.set(status, half_carry_value);
        Flag::AddSubtract.set(status, false);
        Flag::Zero.set(status, result == 0x00);
        Flag::Sign.set(status, result > 0x7F);
    }

    pub(crate) fn rotate_accumulator_left(&mut self) {
        let old_value = self.cpu.state.registers.a as u16;
        let old_carry = Flag::Carry.get_bit(&mut self.cpu.state.status) as u16;
        let new_value = (old_value << 1) + old_carry;
        let new_carry = new_value & 0x100 != 0;
        Flag::Carry.set(&mut self.cpu.state.status, new_carry);
        Flag::HalfCarry.set(&mut self.cpu.state.status, false);
        Flag::AddSubtract.set(&mut self.cpu.state.status, false);
        self.clock(4);
    }
}
