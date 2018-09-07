use vm::cpu::processor::Processor;
use vm::opcodes::Opcode;
use vm::ram::memory::Memory;
use vm::cpu::registers::Registers;
use vm::cpu::flags::Flag;
use vm::cpu::state::State;
use program::Program;

pub struct Machine {
    pub cpu: Processor,
    pub ram: Memory
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            cpu: Processor::new(),
            ram: Memory::new(),
         }
    }

    pub fn load_at(&mut self, program: &Program, start_address: u16) -> () {
        let mut address = start_address;
        for value in program.raw() {
            self.ram.write_u8(address, *value);
            address += 1;
        }
    }
    
    pub fn load(&mut self, program: &Program) -> () { self.load_at(program, 0) }

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

    pub fn start_at(&mut self, address: u16) -> () {   
        self.cpu.halt();
        self.cpu.goto(address); 
        self.cpu.unhalt();
        while !self.cpu.is_halted() {
            self.execute();
        }
    }

    pub fn start(&mut self) { self.start_at(0); }

    fn execute(&mut self) -> () {
        let opcode = Opcode::from(self.next_byte());
        match opcode {
            /* Examples */
            Opcode::Nop         => { self.clock(4); },
            
            Opcode::IncA        => { self.increment_register(|regs| { &mut regs.a }) },
            Opcode::IncB        => { self.increment_register(|regs| { &mut regs.b }) },
            Opcode::IncC        => { self.increment_register(|regs| { &mut regs.c }) },
            Opcode::IncD        => { self.increment_register(|regs| { &mut regs.d }) },
            Opcode::IncE        => { self.increment_register(|regs| { &mut regs.e }) },
            Opcode::IncH        => { self.increment_register(|regs| { &mut regs.h }) },
            Opcode::IncL        => { self.increment_register(|regs| { &mut regs.l }) },

            Opcode::DecA        => { self.decrement_register(|regs| { &mut regs.a }) },
            Opcode::DecB        => { self.decrement_register(|regs| { &mut regs.b }) },
            Opcode::DecC        => { self.decrement_register(|regs| { &mut regs.c }) },
            Opcode::DecD        => { self.decrement_register(|regs| { &mut regs.d }) },
            Opcode::DecE        => { self.decrement_register(|regs| { &mut regs.e }) },
            Opcode::DecH        => { self.decrement_register(|regs| { &mut regs.h }) },
            Opcode::DecL        => { self.decrement_register(|regs| { &mut regs.l }) },

            Opcode::LdBCXX      => { self.load_into_register(|regs| { (&mut regs.b, &mut regs.c) }) },
            Opcode::LdDEXX      => { self.load_into_register(|regs| { (&mut regs.d, &mut regs.e) }) },
            Opcode::LdHLXX      => { self.load_into_register(|regs| { (&mut regs.h, &mut regs.l) }) },
            Opcode::LdSPXX      => { self.load_into_double_register(|state: &mut State| { &mut state.stack_pointer }) },

            Opcode::LdVBCA      => { self.store_into_memory(|regs| { &regs.a }, |regs| { (&regs.b, &regs.c) } )}
            Opcode::LdVDEA      => { self.store_into_memory(|regs| { &regs.a }, |regs| { (&regs.d, &regs.e) } )}

            Opcode::Halt        => { self.cpu.halt(); }

            _                   => panic!()
        }
    }

    fn increment_register(&mut self, selector: fn(&mut Registers) -> &mut u8) -> () {
        self.operate_on_register(selector,
            |value| { value.wrapping_add(1) }, 
            |_, previous| { previous == 0x7F },
            |result, previous| { (result & 0b0001_0000) > 0 && (previous & 0b0001_0000) == 0 });
    } 

    fn decrement_register(&mut self, selector: fn(&mut Registers) -> &mut u8) -> () {
        self.operate_on_register(selector,
            |value| { value.wrapping_sub(1) }, 
            |_, previous| { previous == 0x00 },
            |result, previous| { (result & 0b0001_0000) > 0 && (previous & 0b0001_0000) == 0 });
    }

    fn operate_on_register(&mut self, 
            selector: fn(&mut Registers) -> &mut u8,
            operation: fn(u8) -> u8,
            parity_overflow_check: fn(u8, u8) -> bool,
            half_carry_check: fn(u8, u8) -> bool) -> () {
        let (result, previous) = {           
            let register = selector(&mut self.cpu.state.registers);
            let value = *register;
            let result = operation(value);
            ::std::mem::replace(register, result);
            (result, value)
        };
        self.cpu.state.set_flag(Flag::Sign, result >= 0x80);
        self.cpu.state.set_flag(Flag::Zero, result == 0x00);
        self.cpu.state.set_flag(Flag::HalfCarry, half_carry_check(result, previous));
        self.cpu.state.set_flag(Flag::ParityOverflow, parity_overflow_check(result, previous));
        self.cpu.state.set_flag(Flag::AddSubtract, false);
        self.clock(4);
    }

    fn load_into_register(&mut self, selector: fn(&mut Registers) -> (&mut u8, &mut u8)) {
        {
            let address = self.next_word();
            let (low_val, high_val) = (self.ram.read_u8(address), self.ram.read_u8(address + 1));
            let (high_reg, low_reg) = selector(&mut self.cpu.state.registers);
            ::std::mem::replace(high_reg, high_val);
            ::std::mem::replace(low_reg, low_val);
        }
        self.clock(10);
    }

    fn load_into_double_register(&mut self, selector: fn(&mut State) -> &mut u16) {
        {
            let address = self.next_word();
            let val = self.ram.read_u16(address);
            let reg = selector(&mut self.cpu.state);
            ::std::mem::replace(reg, val);
        }
        self.clock(10);
    }

    fn store_into_memory(&mut self, source: fn(&Registers) -> &u8, pointer: fn(&Registers) -> (&u8, &u8)) {
        let value = *source(&self.cpu.state.registers);
        let (high_reg, low_reg) = pointer(&self.cpu.state.registers);
        let address = ((*high_reg as u16) << 8) | (*low_reg as u16);
        self.ram.write_u8(address, value);
    }

    pub fn clock(&mut self, _tstates: u8) -> () {
        // TODO: Something.
    }
}