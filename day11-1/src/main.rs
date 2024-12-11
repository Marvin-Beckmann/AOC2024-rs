use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_1(puzzle_input));
}

pub fn solve_1(puzzle_input: String) -> usize {
    let mut input = puzzle_input;

    for i in 0..25 {
        input = input
            .split_whitespace()
            .fold("".to_string(), |acc, next_stone| {
                acc + " " + &apply_transformation_rules(next_stone)
            });
        println!("{input}");
    }

    input.split_whitespace().count()
}

fn apply_transformation_rules(string: &str) -> String {
    if string == "0" {
        "1".to_string()
    } else if string.len() % 2 == 0 {
        string[0..(string.len()) / 2].to_owned()
            + " "
            + string[(string.len()) / 2..(string.len() - 1)].trim_start_matches("0")
            + &string[(string.len() - 1)..]
    } else {
        let int_value = string.parse::<usize>().unwrap();
        let new_value = int_value * 2024;
        new_value.to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::solve_1;

    #[test]
    fn test() {
        let input = "125 17";
        solve_1(input.to_owned());
    }
}
