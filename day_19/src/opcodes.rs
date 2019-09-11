pub struct Program<'a> {
    pub ip_register: IpRegister,
    pub instructions: Instructions<'a>,
    pub registers: Registers,
}

impl<'a> Program<'a> {
    pub fn new(ip_register: usize, instructions: Instructions) -> Program {
        Program {
            ip_register,
            instructions,
            registers: [0, 0, 0, 0, 0, 0],
        }
    }

    pub fn exec_next(&mut self) -> bool {
        let instruction_index = self.registers[self.ip_register];
        let instruction = self.instructions.get(instruction_index);

        // Guard against out-of-bounds instruction index
        if instruction.is_none() {
            return false;
        }

        // Run the instruction and increment the register
        let registers_pre = self.registers.clone();

        let instruction = *instruction.unwrap();
        self.exec(&instruction);
        self.registers[self.ip_register] += 1;
        // println!("{} - {:?} - {:?} => {:?}", instruction_index, instruction, registers_pre, self.registers);
        true
    }

    pub fn exec(&mut self, instruction: &Instruction) {
        let registers = &mut self.registers;
        match instruction.opcode {
            "addr" => registers[instruction.c] = registers[instruction.a] + registers[instruction.b],
            "addi" => registers[instruction.c] = registers[instruction.a] + instruction.b,

            "mulr" => registers[instruction.c] = registers[instruction.a] * registers[instruction.b],
            "muli" => registers[instruction.c] = registers[instruction.a] * instruction.b,

            "banr" => registers[instruction.c] = registers[instruction.a] & registers[instruction.b],
            "bani" => registers[instruction.c] = registers[instruction.a] & instruction.b,

            "borr" => registers[instruction.c] = registers[instruction.a] | registers[instruction.b],
            "bori" => registers[instruction.c] = registers[instruction.a] | instruction.b,

            "setr" => registers[instruction.c] = registers[instruction.a],
            "seti" => registers[instruction.c] = instruction.a,

            "gtir" => registers[instruction.c] = {
                if instruction.a > registers[instruction.b] { 1 } else { 0 }
            },
            "gtri" => registers[instruction.c] = {
                if registers[instruction.a] > instruction.b { 1 } else { 0 }
            },
            "gtrr" => registers[instruction.c] = {
                if registers[instruction.a] > registers[instruction.b] { 1 } else { 0 }
            },

            "eqir" => registers[instruction.c] = {
                if instruction.a == registers[instruction.b] { 1 } else { 0 }
            },
            "eqri" => registers[instruction.c] = {
                if registers[instruction.a] == instruction.b { 1 } else { 0 }
            },
            "eqrr" => registers[instruction.c] = {
                if registers[instruction.a] == registers[instruction.b] { 1 } else { 0 }
            },
            _ => panic!("Unknown opcode"),
        }
    }
}

pub type IpRegister = usize;

pub type Instructions<'a> = Vec<Instruction<'a>>;

pub type Registers = [usize; 6];

pub type Opcode = str;

#[derive(Copy, Clone, Debug)]
pub struct Instruction<'a> {
    pub opcode: &'a Opcode,
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec() {
        let instructions = vec![
            Instruction { opcode: "seti", a: 5, b: 0, c: 1 },
            Instruction { opcode: "seti", a: 6, b: 0, c: 2 },
            Instruction { opcode: "addi", a: 0, b: 1, c: 0 },
            Instruction { opcode: "addr", a: 1, b: 2, c: 3 },
            Instruction { opcode: "setr", a: 1, b: 0, c: 0 },
            Instruction { opcode: "seti", a: 8, b: 0, c: 4 },
            Instruction { opcode: "seti", a: 9, b: 0, c: 5 },
        ];
        let mut program = Program::new(0, instructions);

        program.exec_next();
        assert_eq!(program.registers, [1, 5, 0, 0, 0, 0]);

        program.exec_next();
        assert_eq!(program.registers, [2, 5, 6, 0, 0, 0]);

        program.exec_next();
        assert_eq!(program.registers, [4, 5, 6, 0, 0, 0]);

        program.exec_next();
        assert_eq!(program.registers, [6, 5, 6, 0, 0, 0]);

        program.exec_next();
        assert_eq!(program.registers, [7, 5, 6, 0, 0, 9]);
    }
}
