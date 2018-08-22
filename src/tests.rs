#[cfg(test)]
mod tests {
    use vm::machine::Machine;
    use vm::opcodes::Opcode;
    use program::Program;

    #[test]
    fn nothing() { }

    #[test]
    fn load_simple_program() {
        let mut vm = Machine::new();
        let mut p = Program::new();
        p.add(Opcode::IncA);
        p.add(Opcode::DecA);
        p.add(Opcode::Halt);
        vm.load(&p);

        assert_eq!(vm.ram.read_u8(0), Opcode::IncA as u8);
        assert_eq!(vm.ram.read_u8(1), Opcode::DecA as u8);
        assert_eq!(vm.ram.read_u8(2), Opcode::Halt as u8);
    }

    #[test]
    fn load_and_run_simple_program() {
        let mut vm = Machine::new();
        let mut p = Program::new();
        p.add(Opcode::IncA);
        p.add(Opcode::IncA);
        p.add(Opcode::Halt);
        vm.load(&p);
        vm.start();

        let a = vm.cpu.get_register(|regs| regs.a);
        assert_eq!(a, 2);
    }
}