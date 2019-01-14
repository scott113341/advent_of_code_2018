use std::collections::HashSet;

const OPCODES: &'static [&'static Opcode] = &[
    "addr",
    "addi",
    "mulr",
    "muli",
    "banr",
    "bani",
    "borr",
    "bori",
    "setr",
    "seti",
    "gtir",
    "gtri",
    "gtrr",
    "eqir",
    "eqri",
    "eqrr",
];

pub type Registers = [usize; 4];

pub type Opcode = str;

#[derive(Debug)]
pub struct Instruction<'a> {
    pub opcode: &'a Opcode,
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

pub fn exec(instruction: &Instruction, registers: &mut Registers) {
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

pub fn possible_instructions(before: &Registers, after: &Registers, a: usize, b: usize, c: usize) -> HashSet<&'static Opcode> {
    OPCODES
        .iter()
        .cloned()
        .filter(|opcode| {
            let instruction = Instruction { opcode, a, b, c };
            let mut registers = before.clone();
            exec(&instruction, &mut registers);
            registers == *after
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addr() {
        let mut registers = [1, 2, 3, 4];
        let instruction = Instruction { opcode: "addr", a: 0, b: 1, c: 3 };
        exec(&instruction, &mut registers);
        assert_eq!(registers, [1, 2, 3, 3]);
    }

    #[test]
    fn test_possible_instructions() {
        let before = [3, 2, 1, 1];
        let after = [3, 2, 2, 1];

        let possible = possible_instructions(&before, &after, 2, 1, 2);

        assert_eq!(possible, [
            "mulr",
            "addi",
            "seti",
        ].iter().cloned().collect::<HashSet<&Opcode>>());
    }
}
