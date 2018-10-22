use vm::machine::Machine;

impl Machine {
    pub(crate) fn jump(&mut self, predicate: fn(&mut u8) -> bool) {
        let dest = self.next_word();

        if predicate(&mut self.cpu.state.status) {
            self.cpu.goto(dest);
        }

        self.clock(10);
    }
}
