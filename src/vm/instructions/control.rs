use vm::machine::Machine;

impl Machine {
    pub(crate) fn jump(&mut self, predicate: fn(&u8) -> bool) {
        let dest = self.next_word();

        if predicate(&self.cpu.state.status) {
            self.cpu.goto(dest);
        }

        self.clock(10);
    }
}
