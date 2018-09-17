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
    fn increment() {
        let mut vm = run_program(
            |regs| regs.a = 0x7E,
            vec![Opcode::IncA],
        );
        assert_eq!(vm.cpu.get_register(|regs| regs.a), 0x7F);
        assert!(!Flag::ParityOverflow.get(&vm.cpu.state.status));
        assert!(!Flag::Sign.get(&vm.cpu.state.status));
        assert!(!Flag::Carry.get(&vm.cpu.state.status));

        vm.start_at(0);
        assert_eq!(vm.cpu.get_register(|regs| regs.a), 0x80);
        assert!(Flag::ParityOverflow.get(&vm.cpu.state.status));
        assert!(Flag::Sign.get(&vm.cpu.state.status));
        assert!(!Flag::Carry.get(&vm.cpu.state.status));
        
        vm.start_at(0);
        assert_eq!(vm.cpu.get_register(|regs| regs.a), 0x81);
        assert!(!Flag::ParityOverflow.get(&vm.cpu.state.status));
        assert!(Flag::Sign.get(&vm.cpu.state.status));
        assert!(!Flag::Carry.get(&vm.cpu.state.status));

        vm.cpu.state.registers.a = 0xFF;
        vm.start_at(0);
        assert_eq!(vm.cpu.get_register(|regs| regs.a), 0x00);
        assert!(!Flag::ParityOverflow.get(&vm.cpu.state.status));
        assert!(!Flag::Sign.get(&vm.cpu.state.status));
        assert!(!Flag::Carry.get(&vm.cpu.state.status));
    }

    #[test]
    fn add() {
        let mut vm = run_program(
            |regs| {
                regs.a = 0x7E;
                regs.b = 0x01;
            },
            vec![Opcode::AddB],
        );

        assert_eq!(vm.cpu.get_register(|regs| regs.a), 0x7F);
        assert!(!Flag::ParityOverflow.get(&vm.cpu.state.status));
        assert!(!Flag::Sign.get(&vm.cpu.state.status));
        assert!(!Flag::Carry.get(&vm.cpu.state.status));

        vm.start_at(0);
        assert_eq!(vm.cpu.get_register(|regs| regs.a), 0x80);
        assert!(Flag::ParityOverflow.get(&vm.cpu.state.status));
        assert!(Flag::Sign.get(&vm.cpu.state.status));
        assert!(!Flag::Carry.get(&vm.cpu.state.status));
        
        vm.start_at(0);
        assert_eq!(vm.cpu.get_register(|regs| regs.a), 0x81);
        assert!(!Flag::ParityOverflow.get(&vm.cpu.state.status));
        assert!(Flag::Sign.get(&vm.cpu.state.status));
        assert!(!Flag::Carry.get(&vm.cpu.state.status));

        vm.cpu.state.registers.a = 0xFF;
        vm.start_at(0);
        assert_eq!(vm.cpu.get_register(|regs| regs.a), 0x00);
        assert!(!Flag::ParityOverflow.get(&vm.cpu.state.status));
        assert!(!Flag::Sign.get(&vm.cpu.state.status));
        assert!(Flag::Carry.get(&vm.cpu.state.status));
    }
}
