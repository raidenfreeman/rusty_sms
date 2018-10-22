use vm::machine::Machine;
use std::mem;

impl Machine {    
    pub(crate) fn exchange_registers(&mut self) {
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
}