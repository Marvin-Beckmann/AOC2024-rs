use std::{collections::HashMap, fs};

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_2(puzzle_input));
}

pub fn solve_2(puzzle_input: String) -> usize {
    let mut hash = HashMap::new();
    puzzle_input
        .split_whitespace()
        .map(|x| apply_transformation_rules(x.to_owned(), 0, &mut hash))
        .sum()
}

fn apply_transformation_rules(
    string: String,
    depth: usize,
    already_computed: &mut HashMap<(String, usize), usize>,
) -> usize {
    if !already_computed.contains_key(&(string.clone(), depth)) {
        let mut value = 0;

        if depth == 75 {
            value = 1;
        } else if &string == "0" {
            value = apply_transformation_rules("1".to_owned(), depth + 1, already_computed);
        } else if &string.len() % 2 == 0 {
            value = apply_transformation_rules(
                string.clone()[0..(string.len()) / 2].to_string(),
                depth + 1,
                already_computed,
            ) + apply_transformation_rules(
                string[(string.len()) / 2..(string.len() - 1)]
                    .trim_start_matches("0")
                    .to_owned()
                    + &string.clone()[(string.len() - 1)..],
                depth + 1,
                already_computed,
            );
        } else {
            let int_value = string.parse::<usize>().unwrap();
            let new_value = int_value * 2024;

            value = apply_transformation_rules(new_value.to_string(), depth + 1, already_computed);
        }
        already_computed.insert((string.clone(), depth), value);
    }
    already_computed.get(&(string, depth)).unwrap().clone()
}
