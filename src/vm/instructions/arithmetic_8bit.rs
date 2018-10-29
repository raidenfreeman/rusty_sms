use std::collections::HashMap;
use vm::cpu::alu;
use vm::cpu::flags::Flag;
use vm::cpu::operation::Operation;
use vm::cpu::registers::Registers;
use vm::machine::Machine;

impl Machine {
    pub(crate) fn add_register(&mut self, selector: fn(&Registers) -> u8) {
        let operand = selector(&self.cpu.state.registers);
        self.operate_on_register(
            Operation::Add,
            |regs| &mut regs.a,
            operand,
            vec![
                Flag::AddSubtract,
                Flag::Carry,
                Flag::HalfCarry,
                Flag::ParityOverflow,
                Flag::Sign,
                Flag::Zero,
            ],
        );
        self.clock(4);
    }

    pub(crate) fn add_carry_register(&mut self, selector: fn(&Registers) -> u8) {
        let operand = selector(&self.cpu.state.registers);
        let carry = Flag::Carry.get_bit(&self.cpu.state.status);
        self.operate_on_register(
            Operation::Add,
            |regs| &mut regs.a,
            operand + carry,
            vec![
                Flag::AddSubtract,
                Flag::Carry,
                Flag::HalfCarry,
                Flag::ParityOverflow,
                Flag::Sign,
                Flag::Zero,
            ],
        );
        self.clock(4);
    }

    pub(crate) fn subtract_register(&mut self, selector: fn(&Registers) -> u8) {
        let operand = selector(&self.cpu.state.registers);
        self.operate_on_register(
            Operation::Subtract,
            |regs| &mut regs.a,
            operand,
            vec![
                Flag::AddSubtract,
                Flag::Carry,
                Flag::HalfCarry,
                Flag::ParityOverflow,
                Flag::Sign,
                Flag::Zero,
            ],
        );
        self.clock(4);
    }

    pub(crate) fn subtract_carry_register(&mut self, selector: fn(&Registers) -> u8) {
        let operand = selector(&self.cpu.state.registers);
        let carry = Flag::Carry.get_bit(&self.cpu.state.status);
        self.operate_on_register(
            Operation::Subtract,
            |regs| &mut regs.a,
            operand + carry,
            vec![
                Flag::AddSubtract,
                Flag::Carry,
                Flag::HalfCarry,
                Flag::ParityOverflow,
                Flag::Sign,
                Flag::Zero,
            ],
        );
        self.clock(4);
    }

    pub(crate) fn increment_register(&mut self, target: fn(&mut Registers) -> &mut u8) {
        self.operate_on_register(
            Operation::Add,
            target,
            1,
            vec![
                Flag::AddSubtract,
                Flag::ParityOverflow,
                Flag::HalfCarry,
                Flag::Zero,
                Flag::Sign,
            ],
        );
        self.clock(4);
    }

    pub(crate) fn decrement_register(&mut self, target: fn(&mut Registers) -> &mut u8) {
        self.operate_on_register(
            Operation::Subtract,
            target,
            1,
            vec![
                Flag::AddSubtract,
                Flag::ParityOverflow,
                Flag::HalfCarry,
                Flag::Zero,
                Flag::Sign,
            ],
        );
        self.clock(4);
    }

    fn operate_on_register(
        &mut self,
        operation: Operation,
        target: fn(&mut Registers) -> &mut u8,
        operand: u8,
        affected_flags: Vec<Flag>,
    ) {
        let op1 = *target(&mut self.cpu.state.registers);
        let op2 = operation.maybe_negate(operand);
        let result = alu::add_bytes(op1, op2);

        *target(&mut self.cpu.state.registers) = result.value;

        let subtraction = operation == Operation::Subtract;
        let overflow = if op1 < 0x80 && op2 < 0x80 {
            result.value > 0x7F
        } else if op1 > 0x7F && op2 > 0x7F {
            result.value < 0x80
        } else {
            false
        };

        let default_values: HashMap<Flag, bool> = [
            (Flag::Zero, result.value == 0x00),
            (Flag::Sign, result.value > 0x7F),
            (Flag::HalfCarry, result.half_carry),
            (Flag::ParityOverflow, overflow),
            (Flag::AddSubtract, subtraction),
            (Flag::Carry, result.carry),
        ]
        .iter()
        .cloned()
        .collect();

        for flag in affected_flags {
            let status = &mut self.cpu.state.status;
            match default_values.get(&flag) {
                Some(value) => flag.set(status, *value),
                None => {}
            }
        }
    }
}
