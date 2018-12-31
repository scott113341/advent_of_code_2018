#![feature(dbg_macro)]

mod types;

use crate::types::System;

fn main() {
    let input = include_str!("input.txt")
        .trim_end()
        .split("\n")
        .collect();

    println!("part_1: {}", first_crash(&input));
    println!("part_2: {}", last_cart(&input));
}

// Find the location of the first crash
fn first_crash(lines: &Vec<&str>) -> String {
    let mut system = System::parse(lines);

    loop {
        let crashed = system.next_tick();
        if !crashed.is_empty() {
            let coord = crashed.iter().nth(0).unwrap();
            return format!("{},{}", coord.x, coord.y);
        }
    }
}

// Find the location of the last cart (crashing carts are removed instantly)
fn last_cart(lines: &Vec<&str>) -> String {
    let mut system = System::parse(lines);

    loop {
        system.next_tick();
        if system.carts.len() == 1 {
            let last_cart_coord = system.carts.keys().nth(0).unwrap();
            return format!("{},{}", last_cart_coord.x, last_cart_coord.y);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn test_input_1() -> Vec<&'static str> {
        /*
        /->-\
        |   |  /----\
        | /-+--+-\  |
        | | |  | v  |
        \-+-/  \-+--/
          \------/
        */
        vec![
            "/->-\\",
            "|   |  /----\\",
            "| /-+--+-\\  |",
            "| | |  | v  |",
            "\\-+-/  \\-+--/",
            "\\------/ ",
        ]
    }

    pub fn test_input_2() -> Vec<&'static str> {
        /*
        />-<\
        |   |
        | /<+-\
        | | | v
        \>+</ |
          |   ^
          \<->/
        */
        vec![
            "/>-<\\  ",
            "|   |  ",
            "| /<+-\\",
            "| | | v",
            "\\>+</ |",
            "  |   ^",
            "  \\<->/",
        ]
    }

    #[test]
    fn test_first_crash() {
        assert_eq!(first_crash(&test_input_1()), "7,3".to_string());
    }

    #[test]
    fn test_last_cart() {
        assert_eq!(last_cart(&test_input_2()), "6,4".to_string());
    }
}
