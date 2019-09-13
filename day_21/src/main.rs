mod opcodes;

use crate::opcodes::*;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt")
        .trim()
        .split("\n")
        .collect();

    println!("part_1: {}", part_1(&input, 1797184));
    println!("part_2: {}", part_2(&input));
}

// The instruction that uses Register 0 for realz is:
//   #ip 28: "eqrr 1 0 5"
//
// By adding a debug statement to that instruction, I saw that the registers always look like:
//   [ OUR_VALUE, 1797184, 1, 28, 1, 1 ]
//
// We're comparing reg[0] to reg[1], so initializing reg[0] with 1797184 might work... and it does!
// If this didn't work, then the program would never halt.
fn part_1(lines: &Vec<&str>, magic_number: usize) -> usize {
    let (ip_register, instructions) = parse_input(lines);
    let mut program = Program::new(ip_register, instructions);

    // Mutate the 0th register
    program.registers[0] = magic_number;

    // Return the 0th register until exec_next returns false (program terminates)
    loop {
        if program.exec_next() == false {
            return magic_number;
        }
    }
}

// This checks for repeats in Register 1 when running the "eqrr 1 0 5" instruction, which compares
// Register 0 to Register 1. When the first repeat occurs, we return the PREVIOUS value of Register
// 1 as our answer, which gives the most instructions executed.
fn part_2(lines: &Vec<&str>) -> usize {
    let (ip_register, instructions) = parse_input(lines);
    let mut program = Program::new(ip_register, instructions);

    let mut seen_r1_values = HashSet::new();
    let mut previous_r1_value = 0;

    loop {
        while program.exec_next() {
            if program.registers[program.ip_register] == 28 {
                let next_eqrr_comparison = program.registers[1];

                if seen_r1_values.contains(&next_eqrr_comparison) {
                    return previous_r1_value;
                } else {
                    seen_r1_values.insert(next_eqrr_comparison);
                    previous_r1_value = next_eqrr_comparison;
                }
            }
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
