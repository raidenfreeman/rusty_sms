#[cfg(test)]
mod tests {
    use program::Program;
    use vm::cpu::flags::Flag;
    use vm::cpu::registers::Registers;
    use vm::machine::Machine;
    use vm::instructions::opcodes::Opcode;

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
        let mut vm = run_program(|regs| regs.a = 0x7E, vec![Opcode::IncA, Opcode::Halt]);
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
            vec![Opcode::AddB, Opcode::Halt],
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

    #[test]
    fn increment_wide() {
        let mut vm = run_program(
            |regs| {
                regs.b = 0x00;
                regs.c = 0xFE;
            },
            vec![Opcode::IncBC, Opcode::Halt],
        );
        assert_eq!(vm.cpu.get_register(|regs| regs.b), 0x00);
        assert_eq!(vm.cpu.get_register(|regs| regs.c), 0xFF);

        vm.start_at(0);
        assert_eq!(vm.cpu.get_register(|regs| regs.b), 0x01);
        assert_eq!(vm.cpu.get_register(|regs| regs.c), 0x00);
    }

    #[test]
    fn decrement_wide() {
        let mut vm = run_program(
            |regs| {
                regs.b = 0x01;
                regs.c = 0x00;
            },
            vec![Opcode::DecBC, Opcode::Halt],
        );
        assert_eq!(vm.cpu.get_register(|regs| regs.b), 0x00);
        assert_eq!(vm.cpu.get_register(|regs| regs.c), 0xFF);

        vm.start_at(0);
        assert_eq!(vm.cpu.get_register(|regs| regs.b), 0x00);
        assert_eq!(vm.cpu.get_register(|regs| regs.c), 0xFE);
    }

    fn jump_test_flag(opcode: Opcode, param: u16, flag: Flag, flag_value: bool, expected: u16) {
        let mut vm = Machine::new();
        let mut p = Program::new();
        p.add_param_word(opcode, param);
        p.add(Opcode::Halt);
        p.add(Opcode::Halt);
        vm.load(&p);
        flag.set(&mut vm.cpu.state.status, flag_value);
        vm.start();
        assert_eq!(vm.cpu.state.program_counter, expected);
    }

    #[test]
    fn jump() {
        jump_test_flag(Opcode::JpXX, 0x04, Flag::Unused1, true, 0x05);
        jump_test_flag(Opcode::JpNZXX, 0x04, Flag::Zero, false, 0x05);
        jump_test_flag(Opcode::JpNZXX, 0x04, Flag::Zero, true, 0x04);
        jump_test_flag(Opcode::JpZXX, 0x04, Flag::Zero, true, 0x05);
        jump_test_flag(Opcode::JpZXX, 0x04, Flag::Zero, false, 0x04);
        jump_test_flag(Opcode::JpNCXX, 0x04, Flag::Carry, false, 0x05);
        jump_test_flag(Opcode::JpNCXX, 0x04, Flag::Carry, true, 0x04);
        jump_test_flag(Opcode::JpCXX, 0x04, Flag::Carry, true, 0x05);
        jump_test_flag(Opcode::JpCXX, 0x04, Flag::Carry, false, 0x04);
        jump_test_flag(Opcode::JpPOXX, 0x04, Flag::ParityOverflow, true, 0x05);
        jump_test_flag(Opcode::JpPOXX, 0x04, Flag::ParityOverflow, false, 0x04);
        jump_test_flag(Opcode::JpPEXX, 0x04, Flag::ParityOverflow, false, 0x05);
        jump_test_flag(Opcode::JpPEXX, 0x04, Flag::ParityOverflow, true, 0x04);
        jump_test_flag(Opcode::JpPXX, 0x04, Flag::Sign, false, 0x05);
        jump_test_flag(Opcode::JpPXX, 0x04, Flag::Sign, true, 0x04);
        jump_test_flag(Opcode::JpMXX, 0x04, Flag::Sign, true, 0x05);
        jump_test_flag(Opcode::JpMXX, 0x04, Flag::Sign, false, 0x04);
    }
}
