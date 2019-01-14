#![feature(dbg_macro)]

mod opcodes;

extern crate lazy_static;
extern crate regex;

use crate::opcodes::{Registers, Opcode, Instruction, possible_instructions, exec};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

lazy_static! {
    static ref BEFORE_RE: Regex = Regex::new(r"Before:\s+\[(\d+), (\d+), (\d+), (\d+)]").unwrap();
    static ref INSTRUCTION_RE: Regex = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();
    static ref AFTER_RE: Regex = Regex::new(r"After:\s+\[(\d+), (\d+), (\d+), (\d+)]").unwrap();
}

fn main() {
    let examples = include_str!("part_1_input.txt").trim();
    let program = include_str!("part_2_input.txt").trim();

    println!("part_1: {}", part_1(examples));
    println!("part_2: {}", part_2(examples, program));
}

fn part_1(lines: &str) -> usize {
    let mut samples_3_opcodes = 0;

    for example in lines.split("\n\n") {
        let (before, after, _code, a, b, c) = parse_example(example);
        let possible = possible_instructions(&before, &after, a, b, c);
        if possible.len() >= 3 {
            samples_3_opcodes += 1;
        }
    }

    samples_3_opcodes
}

fn part_2(examples: &str, program: &str) -> usize {
    let mut poss_opcodes: HashMap<usize, HashSet<&Opcode>> = HashMap::new();
    let mut solved_opcodes: HashMap<&Opcode, usize> = HashMap::new();
    let mut solved_codes: HashMap<usize, &Opcode> = HashMap::new();

    // Go through each example, only retaining the possible opcodes (str) for each code (int)
    for example in examples.split("\n\n") {
        let (before, after, code, a, b, c) = parse_example(example);
        let possible = possible_instructions(&before, &after, a, b, c);
        poss_opcodes
            .entry(code)
            .or_insert(possible.clone())
            .retain(|&opcode| possible.contains(opcode));
    }

    // Solve each opcode by iteratively removing solved opcodes from each set of possibilities
    while solved_opcodes.len() < poss_opcodes.len() {
        for (code, opcodes) in poss_opcodes.iter_mut() {
            opcodes.retain(|&opcode| !solved_opcodes.contains_key(opcode));
            if opcodes.len() == 1 {
                let opcode = opcodes.iter().nth(0).unwrap();
                solved_opcodes.insert(opcode, *code);
                solved_codes.insert(*code, opcode);
            }
        }
    }

    // Run the program
    let mut registers: Registers = [0, 0, 0, 0];
    for line in program.split("\n") {
        let instruction_cap = INSTRUCTION_RE.captures(&line).unwrap();
        let code: usize = instruction_cap[1].parse().unwrap();
        let opcode = solved_codes.get(&code).unwrap();
        let instruction = Instruction {
            opcode,
            a: instruction_cap[2].parse().unwrap(),
            b: instruction_cap[3].parse().unwrap(),
            c: instruction_cap[4].parse().unwrap(),
        };
        exec(&instruction, &mut registers);
    }

    registers[0]
}

fn parse_example(example: &str) -> (Registers, Registers, usize, usize, usize, usize) {
    let before_cap = BEFORE_RE.captures(&example).unwrap();
    let after_cap = AFTER_RE.captures(&example).unwrap();
    let instruction_cap = INSTRUCTION_RE.captures(&example).unwrap();

    let before = [
        before_cap[1].parse().unwrap(),
        before_cap[2].parse().unwrap(),
        before_cap[3].parse().unwrap(),
        before_cap[4].parse().unwrap(),
    ];

    let after = [
        after_cap[1].parse().unwrap(),
        after_cap[2].parse().unwrap(),
        after_cap[3].parse().unwrap(),
        after_cap[4].parse().unwrap(),
    ];

    let code = instruction_cap[1].parse().unwrap();
    let a = instruction_cap[2].parse().unwrap();
    let b = instruction_cap[3].parse().unwrap();
    let c = instruction_cap[4].parse().unwrap();

    (before, after, code, a, b, c)
}
