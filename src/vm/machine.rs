use program::Program;
use std::collections::HashMap;
use vm::cpu::flags::Flag;
use vm::cpu::operation::Operation;
use vm::cpu::processor::Processor;
use vm::cpu::registers::Registers;
use vm::cpu::state::State;
use vm::opcodes::Opcode;
use vm::ram::memory::Memory;

pub struct Machine {
    pub cpu: Processor,
    pub ram: Memory,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            cpu: Processor::new(),
            ram: Memory::new(),
        }
    }

    pub fn load_at(&mut self, program: &Program, start_address: u16) {
        let mut address = start_address;
        for value in program.raw() {
            self.ram.write_u8(address, *value);
            address += 1;
        }
    }

    pub fn load(&mut self, program: &Program) {
        self.load_at(program, 0)
    }

    fn next_byte(&mut self) -> u8 {
        let pc = self.cpu.state.program_counter;
        let val = self.ram.read_u8(pc);
        let (result, overflow) = pc.overflowing_add(1);
        if overflow {
            self.cpu.halt();
        } else {
            self.cpu.state.program_counter = result;
        }
        val
    }

    fn next_word(&mut self) -> u16 {
        let low = self.next_byte() as u16;
        let high = self.next_byte() as u16;
        (high << 8) | low
    }

    pub fn start_at(&mut self, address: u16) {
        self.cpu.halt();
        self.cpu.goto(address);
        self.cpu.unhalt();
        while !self.cpu.is_halted() {
            self.execute();
        }
    }

    pub fn start(&mut self) {
        self.start_at(0);
    }

    fn execute(&mut self) {
        let opcode = Opcode::from(self.next_byte());
        match opcode {
            Opcode::Nop => self.clock(4),

            Opcode::IncA => self.increment_register(|regs| &mut regs.a),
            Opcode::IncB => self.increment_register(|regs| &mut regs.b),
            Opcode::IncC => self.increment_register(|regs| &mut regs.c),
            Opcode::IncD => self.increment_register(|regs| &mut regs.d),
            Opcode::IncE => self.increment_register(|regs| &mut regs.e),
            Opcode::IncH => self.increment_register(|regs| &mut regs.h),
            Opcode::IncL => self.increment_register(|regs| &mut regs.l),

            Opcode::AddA => self.add_register(|regs| &mut regs.a),
            Opcode::AddB => self.add_register(|regs| &mut regs.b),
            Opcode::AddC => self.add_register(|regs| &mut regs.c),
            Opcode::AddD => self.add_register(|regs| &mut regs.d),
            Opcode::AddE => self.add_register(|regs| &mut regs.e),
            Opcode::AddH => self.add_register(|regs| &mut regs.h),
            Opcode::AddL => self.add_register(|regs| &mut regs.l),

            Opcode::AdcA => self.add_carry_register(|regs| &mut regs.a),
            Opcode::AdcB => self.add_carry_register(|regs| &mut regs.b),
            Opcode::AdcC => self.add_carry_register(|regs| &mut regs.c),
            Opcode::AdcD => self.add_carry_register(|regs| &mut regs.d),
            Opcode::AdcE => self.add_carry_register(|regs| &mut regs.e),
            Opcode::AdcH => self.add_carry_register(|regs| &mut regs.h),
            Opcode::AdcL => self.add_carry_register(|regs| &mut regs.l),

            Opcode::DecA => self.decrement_register(|regs| &mut regs.a),
            Opcode::DecB => self.decrement_register(|regs| &mut regs.b),
            Opcode::DecC => self.decrement_register(|regs| &mut regs.c),
            Opcode::DecD => self.decrement_register(|regs| &mut regs.d),
            Opcode::DecE => self.decrement_register(|regs| &mut regs.e),
            Opcode::DecH => self.decrement_register(|regs| &mut regs.h),
            Opcode::DecL => self.decrement_register(|regs| &mut regs.l),

            Opcode::LdBCXX => self.load_into_register(|regs| (&mut regs.b, &mut regs.c)),
            Opcode::LdDEXX => self.load_into_register(|regs| (&mut regs.d, &mut regs.e)),
            Opcode::LdHLXX => self.load_into_register(|regs| (&mut regs.h, &mut regs.l)),
            Opcode::LdSPXX => {
                self.load_into_double_register(|state: &mut State| &mut state.stack_pointer)
            }

            Opcode::LdVBCA => self.store_into_memory(|regs| &regs.a, |regs| (&regs.b, &regs.c)),
            Opcode::LdVDEA => self.store_into_memory(|regs| &regs.a, |regs| (&regs.d, &regs.e)),

            Opcode::Halt => self.cpu.halt(),

            _ => panic!(),
        }
    }

    fn add_register(&mut self, selector: fn(&mut Registers) -> &mut u8) {
        let operand = *selector(&mut self.cpu.state.registers);
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

    fn add_carry_register(&mut self, selector: fn(&mut Registers) -> &mut u8) {
        let operand = *selector(&mut self.cpu.state.registers);
        let carry = if Flag::Carry.get(&self.cpu.state.status) { 1 } else { 0 }; 
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

    // fn logical_and(&mut self, selector: fn(&mut Registers)-> &mut u8, operation: fn(u8, u8) -> u8) -> () {
    //     // get the data
    //     let register =  selector(&mut self.cpu.state.registers);
    //     let accumulator = &mut self.cpu.state.registers.a;
    //     //
    //     let value = *accumulator & *register;
    //     ::std::mem::replace(accumulator, value);
    //     *accumulator = value;
    //     // set flags
    //     self.cpu.state.set_flag(Flag::Sign, value >= 0x80);
    //     self.cpu.state.set_flag(Flag::Zero, value == 0x00);
    //     self.cpu.state.set_flag(Flag::ParityOverflow, value > 0xFF);
    //     self.cpu.state.set_flag(Flag::HalfCarry, (value >= 0x10)&&(*accumulator < 0x10) );
    //     self.cpu.state.set_flag(Flag::AddSubtract, false);
    //     // advance clock
    //     self.clock(4);
    // }

    fn increment_register(&mut self, target: fn(&mut Registers) -> &mut u8) {
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

    fn decrement_register(&mut self, target: fn(&mut Registers) -> &mut u8) {
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
        ].iter()
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

    fn and_register(&mut self, selector: fn(&Registers) -> &u8) {
        self.bitwise_operation(selector, |a, b| a & b, true);
    }

    fn bitwise_operation(&mut self, operand: fn(&Registers) -> &u8, operation: fn(u8, u8) -> u8, half_carry_value: bool) {
        let op1 = self.cpu.state.registers.a;
        let op2 = *operand(&self.cpu.state.registers);
        let result = operation(op1, op2);
        let parity = (0..8).fold(0, |acc, b| { acc + (result >> b) & 1 }) % 2 == 0;

        let status = &mut self.cpu.state.status;
        Flag::ParityOverflow.set(status, parity);
        Flag::Carry.set(status, false);
        Flag::HalfCarry.set(status, half_carry_value);
        Flag::AddSubtract.set(status, false);
        Flag::Zero.set(status, result == 0x00);
        Flag::Sign.set(status, result > 0x7F);
    }

    fn load_into_register(&mut self, selector: fn(&mut Registers) -> (&mut u8, &mut u8)) {
        {
            let address = self.next_word();
            let (low_val, high_val) = (self.ram.read_u8(address), self.ram.read_u8(address + 1));
            let (high_reg, low_reg) = selector(&mut self.cpu.state.registers);
            *high_reg = high_val;
            *low_reg = low_val;
        }
        self.clock(10);
    }

    fn load_into_double_register(&mut self, selector: fn(&mut State) -> &mut u16) {
        {
            let address = self.next_word();
            let val = self.ram.read_u16(address);
            let reg = selector(&mut self.cpu.state);
            *reg = val;
        }
        self.clock(10);
    }

    fn store_into_memory(
        &mut self,
        source: fn(&Registers) -> &u8,
        pointer: fn(&Registers) -> (&u8, &u8),
    ) {
        let value = *source(&self.cpu.state.registers);
        let (high_reg, low_reg) = pointer(&self.cpu.state.registers);
        let address = ((*high_reg as u16) << 8) | (*low_reg as u16);
        self.ram.write_u8(address, value);
    }

    pub fn clock(&mut self, _tstates: u8) {
        // TODO: Something.
    }
}
