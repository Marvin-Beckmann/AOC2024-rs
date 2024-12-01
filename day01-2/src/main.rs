use std::{collections::HashMap, fs};

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_2(puzzle_input));
}

pub fn solve_2(puzzle_input: String) -> i64 {
    let total_string: Vec<Vec<&str>> = puzzle_input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();
    let left: Vec<i64> = total_string
        .iter()
        .map(|line| line.first().unwrap().parse::<i64>().unwrap())
        .collect();
    let right: Vec<i64> = total_string
        .iter()
        .map(|line| line.last().unwrap().parse::<i64>().unwrap())
        .collect();

    let multiplicities = multiplicities_right(right);

    left.iter().fold(0, |acc, value| {
        acc + value * multiplicities.get(value).or(Some(&0)).unwrap()
    })
}

fn multiplicities_right(right: Vec<i64>) -> HashMap<i64, i64> {
    let mut out = HashMap::new();

    for value in right.iter() {
        if let Some(entry) = out.get_mut(value) {
            *entry += 1
        } else {
            out.insert(value.to_owned(), 1);
        }
    }
    out
}
