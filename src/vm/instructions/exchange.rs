use std::mem;
use vm::machine::Machine;

impl Machine {
    pub(crate) fn exchange_all_registers_with_shadow(&mut self) {
        {
            let reg = &mut self.cpu.state.registers;
            let alt = &mut self.cpu.state.alt_registers;
            mem::swap(&mut reg.b, &mut alt.b);
            mem::swap(&mut reg.c, &mut alt.c);
            mem::swap(&mut reg.d, &mut alt.d);
            mem::swap(&mut reg.e, &mut alt.e);
            mem::swap(&mut reg.h, &mut alt.h);
            mem::swap(&mut reg.l, &mut alt.l);
        }
        self.clock(4);
    }

    pub(crate) fn exchange_accumulator_and_flags_with_shadow(&mut self) {
        {
            let reg = &mut self.cpu.state.registers;
            let alt = &mut self.cpu.state.alt_registers;
            mem::swap(&mut reg.a, &mut alt.a);
            mem::swap(&mut reg.f, &mut alt.f);
        }
        self.clock(4);
    }

    pub(crate) fn exhange_de_with_hl(&mut self) {
        {
            let reg = &mut self.cpu.state.registers;
            mem::swap(&mut reg.d, &mut reg.h);
            mem::swap(&mut reg.e, &mut reg.l);
        }
        self.clock(4);
    }

    pub(crate) fn exchage_memory_from_sp_with_hl(&mut self) {
        {
            let reg = &mut self.cpu.state.registers;
            let low_address = (reg.s as u16) << 8 | reg.p as u16;
            let high_address = low_address + 1;
            self.ram.write_u8(low_address, reg.l);
            self.ram.write_u8(high_address, reg.h);
        }
        self.clock(19);
    }
}
