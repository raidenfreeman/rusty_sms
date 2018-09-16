#[cfg(test)]
mod tests {
    use program::Program;
    use vm::cpu::flags::Flag;
    use vm::cpu::registers::Registers;
    use vm::machine::Machine;
    use vm::opcodes::Opcode;

    fn run_program(regs: fn(&mut Registers), stream: Vec<Opcode>) -> Machine {
        let mut vm = Machine::new();
        let mut p = Program::new();
        p.add_vector(stream.iter().map(|i| *i as u8).collect());
        vm.load(&p);
        regs(&mut vm.cpu.state.registers);
        vm.start();
        vm
    }

    fn run_program_from_default_state(stream: Vec<Opcode>) -> Machine {
        run_program(|_| {}, stream)
    }

    #[test]
    fn increment_without_overflow() {
        let vm = run_program(|regs| regs.a = 0x7E, vec![Opcode::IncA, Opcode::Halt]);

        let a = vm.cpu.get_register(|regs| regs.a);
        assert_eq!(a, 0x7F);
    }

    #[test]
    fn increment_with_overflow() {
        let vm = run_program(|regs| regs.a = 0x7F, vec![Opcode::IncA, Opcode::Halt]);

        let overflow = Flag::ParityOverflow.get(&vm.cpu.state.status);
        assert_eq!(overflow, true);
    }

    #[test]
    fn add_without_overflow() {
        let vm = run_program(|regs| { regs.a = 0x7E; regs.b = 0x01; }, vec![Opcode::AddB]);

        let a = vm.cpu.get_register(|regs| regs.a);
        assert_eq!(a, 0x7F);               
    }

    #[test]
    fn add_with_overflow() {
        let vm = run_program(|regs| { regs.a = 0x7E; regs.b = 0x02; }, vec![Opcode::AddB]);

        let a = vm.cpu.get_register(|regs| regs.a);
        assert_eq!(a, 0x80);

        let overflow = Flag::ParityOverflow.get(&vm.cpu.state.status);
        assert_eq!(overflow, true);               
    }
}
