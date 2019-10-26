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

        if instruction_index == 4 && self.registers[2] * self.registers[3] == self.registers[5] {
            let r = self.registers;
            println!("{:10}  {:10}  {:10}  {:10}  {:10}  {:10}", r[0], r[1], r[2], r[3], r[4], r[5]);
        }

        let instruction = *instruction.unwrap();
        self.exec(&instruction);
        self.registers[self.ip_register] += 1;
        true
    }

    pub fn exec_next_optimized(&mut self) -> bool {
        let instruction_index = self.registers[self.ip_register];
        let instruction = self.instructions.get(instruction_index);

        // Guard against out-of-bounds instruction index
        if instruction.is_none() {
            return false;
        }

        // This is a result of inspecting instructions 3-11 and figuring out what they do: find the
        // divisors of 10551339 (stored in r[5]) and sum them up in r[0]. Once that's done, it sets
        // r[4] (the instruction pointer) to 11. I figured this out by looking at what the value of
        // all registers were during normal ::exec_next execution when "eqrr 1 5 1" was true.
        if instruction_index == 3 {
            let r = &mut self.registers;

            // Set r[2] to our target
            r[2] = r[5];

            // Decrement r[2] down to 1
            while r[2] >= 1 {
                // If r[2] is a divisor of target, add it to r[0] and save multiplier to r[3]
                if r[5] % r[2] == 0 {
                    r[3] = r[5] / r[2];
                    r[0] += r[3];
                    println!("{:10}  {:10}  {:10}  {:10}  {:10}  {:10}", r[0], r[1], r[2], r[3], r[4], r[5]);
                }

                r[2] -= 1;
            }

            r[self.ip_register] = 11;
        } else {
            let instruction = *instruction.unwrap();
            self.exec(&instruction);
        }

        self.registers[self.ip_register] += 1;

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
