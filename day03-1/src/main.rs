use regex::Regex;
use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_1(puzzle_input));
}

pub fn solve_1(puzzle_input: String) -> i64 {
    let regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    regex
        .find_iter(&puzzle_input)
        .fold(0, |acc, mulops| acc + compute(mulops.as_str()))
}

pub fn compute(mulops: &str) -> i64 {
    let stripped = &mulops[4..(mulops.len() - 1)];
    let (x, y) = stripped.split_once(",").unwrap();
    x.parse::<i64>().unwrap() * y.parse::<i64>().unwrap()
}
