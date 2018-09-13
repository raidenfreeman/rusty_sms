use vm::opcodes::Opcode;

pub struct Program {
    bin: Vec<u8>,
}

impl Program {
    pub fn new() -> Program {
        Program { bin: Vec::new() }
    }

    pub fn raw(&self) -> &Vec<u8> {
        &self.bin
    }

    pub fn add(&mut self, opcode: Opcode) {
        self.add_vector(opcode, Vec::new());
    }

    pub fn add_param(&mut self, opcode: Opcode, parameter: u8) {
        let mut params = Vec::new();
        params.push(parameter);
        self.add_vector(opcode, params);
    }

    pub fn add_params(&mut self, opcode: Opcode, parameter_1: u8, parameter_2: u8) {
        let mut params = Vec::new();
        params.push(parameter_1);
        params.push(parameter_2);
        self.add_vector(opcode, params);
    }

    pub fn add_vector(&mut self, opcode: Opcode, mut parameters: Vec<u8>) {
        self.bin.push(opcode as u8);
        self.bin.append(&mut parameters);
    }
}
