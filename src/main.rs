#![allow(unused)]
use aoc_23::days::day1;
use aoc_23::input_parser::read_input;

fn main() {
    let input = read_input();
    println!("{:?}", input);
    println!("{:?}", day1::run_part1(input));
}
