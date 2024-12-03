use regex::Regex;
use std::fs;

pub const MUL_INSTR: &str = r"mul\([0-9]{1,3},[0-9]{1,3}\)";
pub const DO_INSTR: &str = r"do\(\)";
pub const DONT_INSTR: &str = r"don't\(\)";

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_2(puzzle_input));
}

pub fn solve_2(puzzle_input: String) -> i64 {
    let regex = Regex::new(&format!("{MUL_INSTR}|{DO_INSTR}|{DONT_INSTR}")).unwrap();
    let do_regex = Regex::new(DO_INSTR).unwrap();
    let dont_regex = Regex::new(DONT_INSTR).unwrap();
    let mul_regex = Regex::new(MUL_INSTR).unwrap();
    let mut operations_enabled = true;
    regex.find_iter(&puzzle_input).fold(0, |acc, ops| {
        acc + {
            if operations_enabled & mul_regex.is_match(ops.as_str()) {
                compute(ops.as_str())
            } else if do_regex.is_match(ops.as_str()) {
                operations_enabled = true;
                0
            } else if dont_regex.is_match(ops.as_str()) {
                operations_enabled = false;
                0
            } else {
                0
            }
        }
    })
}

pub fn compute(mulops: &str) -> i64 {
    let stripped = &mulops[4..(mulops.len() - 1)];
    let (x, y) = stripped.split_once(",").unwrap();
    x.parse::<i64>().unwrap() * y.parse::<i64>().unwrap()
}
