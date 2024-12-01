use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_1(puzzle_input));
}

pub fn solve_1(puzzle_input: String) -> i64 {
    let total_string: Vec<Vec<&str>> = puzzle_input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();
    let mut left: Vec<i64> = total_string
        .iter()
        .map(|line| line.first().unwrap().parse::<i64>().unwrap())
        .collect();
    let mut right: Vec<i64> = total_string
        .iter()
        .map(|line| line.last().unwrap().parse::<i64>().unwrap())
        .collect();

    left.sort();
    right.sort();

    let mut total = 0;
    for i in 0..left.len() {
        total = total + (left[i] - right[i]).abs();
    }
    total
}
