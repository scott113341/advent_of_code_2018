#![feature(dbg_macro)]

mod opcodes;

use crate::opcodes::{Instruction, Instructions, IpRegister, Program};

fn main() {
    let input = include_str!("input.txt")
        .trim()
        .split("\n")
        .collect();

    println!("part_1: {}", register_0_result(&input));
    println!("part_2: {}", register_0_result_different_start(&input));
}

fn register_0_result(input: &Vec<&str>) -> usize {
    let (ip_register, instructions) = parse_input(input);
    let mut program = Program::new(ip_register, instructions);

    // Return the 0th register until exec_next returns false (program terminates)
    loop {
        if program.exec_next() == false {
            return program.registers[0];
        }
    }
}

// A new background process immediately spins up in its place. It appears identical, but on closer
// inspection, you notice that this time, register 0 started with the value 1.
fn register_0_result_different_start(input: &Vec<&str>) -> usize {
    let (ip_register, instructions) = parse_input(input);
    let mut program = Program::new(ip_register, instructions);

    // Mutate the 0th register
    program.registers[0] = 1;

    // Return the 0th register until exec_next returns false (program terminates)
    loop {
        if program.exec_next() == false {
            return program.registers[0];
        }
    }
}

fn parse_input<'a>(input: &Vec<&'a str>) -> (IpRegister, Instructions<'a>) {
    let (ip_register_slice, instructions_slice) = input.split_at(1);

    let ip_register = ip_register_slice[0].split(' ').last().unwrap().parse().unwrap();

    let instructions = instructions_slice
        .iter()
        .map(|i| {
            let mut instruction = i.split(' ').into_iter();
            Instruction {
                opcode: instruction.next().unwrap(),
                a: instruction.next().unwrap().parse().unwrap(),
                b: instruction.next().unwrap().parse().unwrap(),
                c: instruction.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    (ip_register, instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_0_result() {
        let input = vec![
            "#ip 0",
            "seti 5 0 1",
            "seti 6 0 2",
            "addi 0 1 0",
            "addr 1 2 3",
            "setr 1 0 0",
            "seti 8 0 4",
            "seti 9 0 5",
        ];

        assert_eq!(register_0_result(&input), 7);
    }
}
