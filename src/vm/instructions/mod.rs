mod arithmetic;
mod bitwise;
mod control;
mod interrupts;
mod memory;
pub mod opcodes;

use vm::cpu::flags::Flag;
use vm::instructions::opcodes::Opcode;
use vm::machine::Machine;

impl Machine {
    pub(crate) fn execute(&mut self) {
        let opcode = Opcode::from(self.next_byte());
        match opcode {
            Opcode::Nop => self.nop(),

            Opcode::IncA => self.increment_register(|regs| &mut regs.a),
            Opcode::IncB => self.increment_register(|regs| &mut regs.b),
            Opcode::IncC => self.increment_register(|regs| &mut regs.c),
            Opcode::IncD => self.increment_register(|regs| &mut regs.d),
            Opcode::IncE => self.increment_register(|regs| &mut regs.e),
            Opcode::IncH => self.increment_register(|regs| &mut regs.h),
            Opcode::IncL => self.increment_register(|regs| &mut regs.l),

            Opcode::IncBC => self.increment_register_wide(|regs| &mut regs.b, |regs| &mut regs.c),
            Opcode::IncDE => self.increment_register_wide(|regs| &mut regs.d, |regs| &mut regs.e),
            Opcode::IncHL => self.increment_register_wide(|regs| &mut regs.h, |regs| &mut regs.l),
            Opcode::IncSP => self.increment_register_wide(|regs| &mut regs.s, |regs| &mut regs.p),

            Opcode::JpXX => self.jump(|_| true),
            Opcode::JpNZXX => self.jump(|status| !Flag::Zero.get(status)),
            Opcode::JpZXX => self.jump(|status| Flag::Zero.get(status)),
            Opcode::JpNCXX => self.jump(|status| !Flag::Carry.get(status)),
            Opcode::JpCXX => self.jump(|status| Flag::Carry.get(status)),
            Opcode::JpPOXX => self.jump(|status| Flag::ParityOverflow.get(status)),
            Opcode::JpPEXX => self.jump(|status| !Flag::ParityOverflow.get(status)),
            Opcode::JpPXX => self.jump(|status| !Flag::Sign.get(status)),
            Opcode::JpMXX => self.jump(|status| Flag::Sign.get(status)),

            Opcode::AddA => self.add_register(|regs| regs.a),
            Opcode::AddB => self.add_register(|regs| regs.b),
            Opcode::AddC => self.add_register(|regs| regs.c),
            Opcode::AddD => self.add_register(|regs| regs.d),
            Opcode::AddE => self.add_register(|regs| regs.e),
            Opcode::AddH => self.add_register(|regs| regs.h),
            Opcode::AddL => self.add_register(|regs| regs.l),

            Opcode::AdcA => self.add_carry_register(|regs| regs.a),
            Opcode::AdcB => self.add_carry_register(|regs| regs.b),
            Opcode::AdcC => self.add_carry_register(|regs| regs.c),
            Opcode::AdcD => self.add_carry_register(|regs| regs.d),
            Opcode::AdcE => self.add_carry_register(|regs| regs.e),
            Opcode::AdcH => self.add_carry_register(|regs| regs.h),
            Opcode::AdcL => self.add_carry_register(|regs| regs.l),

            Opcode::SubA => self.subtract_register(|regs| regs.a),
            Opcode::SubB => self.subtract_register(|regs| regs.b),
            Opcode::SubC => self.subtract_register(|regs| regs.c),
            Opcode::SubD => self.subtract_register(|regs| regs.d),
            Opcode::SubE => self.subtract_register(|regs| regs.e),
            Opcode::SubH => self.subtract_register(|regs| regs.h),
            Opcode::SubL => self.subtract_register(|regs| regs.l),

            Opcode::SbcA => self.subtract_carry_register(|regs| regs.a),
            Opcode::SbcB => self.subtract_carry_register(|regs| regs.b),
            Opcode::SbcC => self.subtract_carry_register(|regs| regs.c),
            Opcode::SbcD => self.subtract_carry_register(|regs| regs.d),
            Opcode::SbcE => self.subtract_carry_register(|regs| regs.e),
            Opcode::SbcH => self.subtract_carry_register(|regs| regs.h),
            Opcode::SbcL => self.subtract_carry_register(|regs| regs.l),

            Opcode::AndA => self.and_register(|regs| regs.a),
            Opcode::AndB => self.and_register(|regs| regs.b),
            Opcode::AndC => self.and_register(|regs| regs.c),
            Opcode::AndD => self.and_register(|regs| regs.d),
            Opcode::AndE => self.and_register(|regs| regs.e),
            Opcode::AndH => self.and_register(|regs| regs.h),
            Opcode::AndL => self.and_register(|regs| regs.l),

            Opcode::DecA => self.decrement_register(|regs| &mut regs.a),
            Opcode::DecB => self.decrement_register(|regs| &mut regs.b),
            Opcode::DecC => self.decrement_register(|regs| &mut regs.c),
            Opcode::DecD => self.decrement_register(|regs| &mut regs.d),
            Opcode::DecE => self.decrement_register(|regs| &mut regs.e),
            Opcode::DecH => self.decrement_register(|regs| &mut regs.h),
            Opcode::DecL => self.decrement_register(|regs| &mut regs.l),

            Opcode::DecBC => self.decrement_register_wide(|regs| &mut regs.b, |regs| &mut regs.c),
            Opcode::DecDE => self.decrement_register_wide(|regs| &mut regs.d, |regs| &mut regs.e),
            Opcode::DecHL => self.decrement_register_wide(|regs| &mut regs.h, |regs| &mut regs.l),
            Opcode::DecSP => self.decrement_register_wide(|regs| &mut regs.s, |regs| &mut regs.p),

            Opcode::Halt => self.halt(),

            Opcode::LdBCXX => self.load_into_register_pair(|regs| (&mut regs.b, &mut regs.c)),
            Opcode::LdDEXX => self.load_into_register_pair(|regs| (&mut regs.d, &mut regs.e)),
            Opcode::LdHLXX => self.load_into_register_pair(|regs| (&mut regs.h, &mut regs.l)),
            Opcode::LdSPXX => self.load_into_register_pair(|regs| (&mut regs.s, &mut regs.p)),

            Opcode::LdVBCA => self.load_into_memory(|regs| regs.a, |regs| (regs.b, regs.c)),
            Opcode::LdVDEA => self.load_into_memory(|regs| regs.a, |regs| (regs.d, regs.e)),

            Opcode::OrA => self.or_register(|regs| regs.a),
            Opcode::OrB => self.or_register(|regs| regs.b),
            Opcode::OrC => self.or_register(|regs| regs.c),
            Opcode::OrD => self.or_register(|regs| regs.d),
            Opcode::OrE => self.or_register(|regs| regs.e),
            Opcode::OrH => self.or_register(|regs| regs.h),
            Opcode::OrL => self.or_register(|regs| regs.l),

            Opcode::XorA => self.xor_register(|regs| regs.a),
            Opcode::XorB => self.xor_register(|regs| regs.b),
            Opcode::XorC => self.xor_register(|regs| regs.c),
            Opcode::XorD => self.xor_register(|regs| regs.d),
            Opcode::XorE => self.xor_register(|regs| regs.e),
            Opcode::XorH => self.xor_register(|regs| regs.h),
            Opcode::XorL => self.xor_register(|regs| regs.l),

            Opcode::Exx => self.exchange_registers(),

            _ => panic!(),
        }
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

    fn next_byte_pair(&mut self) -> (u8, u8) {
        let low = self.next_byte();
        let high = self.next_byte();
        (high, low)
    }

    fn next_word(&mut self) -> u16 {
        let low = self.next_byte() as u16;
        let high = self.next_byte() as u16;
        (high << 8) | low
    }

    pub fn clock(&mut self, _tstates: u8) {
        // TODO: Something.
    }
}
