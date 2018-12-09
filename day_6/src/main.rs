#![feature(dbg_macro)]

mod grid;
mod part_1;
mod part_2;

fn main() {
    let pois = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| grid::Poi::parse(&s))
        .collect();

    println!("part_1: {}", part_1::part_1(&pois));
    println!("part_2: {}", part_2::part_2(&pois, 10_000));
}
