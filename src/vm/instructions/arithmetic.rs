use std::collections::HashMap;
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

    fn operate_on_register(
        &mut self,
        operation: Operation,
        target: fn(&mut Registers) -> &mut u8,
        operand: u8,
        affected_flags: Vec<Flag>,
    ) {
        let op1 = *target(&mut self.cpu.state.registers) as u16;
        let op2 = if operation == Operation::Add {
            operand
        } else {
            !operand + 1
        } as u16;
        let result16 = op1 + op2;
        let result8 = (result16 & 0xFF) as u8;
        let result4 = (op1 & 0xF) + (op2 & 0xF);
        *target(&mut self.cpu.state.registers) = result8;

        let subtraction = operation == Operation::Subtract;
        let overflow = if op1 < 0x80 && op2 < 0x80 {
            result16 > 0x7F
        } else if op1 > 0x7F && op2 > 0x7F {
            result16 < 0x80
        } else {
            false
        };

        let default_values: HashMap<Flag, bool> = [
            (Flag::Zero, result8 == 0x00),
            (Flag::Sign, result8 > 0x7F),
            (Flag::HalfCarry, result4 > 0xF),
            (Flag::ParityOverflow, overflow),
            (Flag::AddSubtract, subtraction),
            (Flag::Carry, result16 > 0xFF),
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

    pub(crate) fn set_flag(&mut self, flag: Flag, value: fn(bool) -> bool) {
        let previous = flag.get(&self.cpu.state.status);
        flag.set(&mut self.cpu.state.status, value(previous));
        self.clock(4);
    }
}
