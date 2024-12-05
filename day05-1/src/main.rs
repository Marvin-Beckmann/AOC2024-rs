use std::{collections::HashMap, fs};

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_1(puzzle_input, 1176, 1177));
}

pub fn solve_1(puzzle_input: String, last_rule: usize, first_instr: usize) -> i64 {
    let mut total = 0;
    let input: Vec<&str> = puzzle_input.lines().collect();

    let rules = &input[0..last_rule];
    let instructions = &input[first_instr..];

    let mut rules_map: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in rules.iter() {
        println!("{line}");
        let (first, second) = line.split_once("|").unwrap();
        if let Some(before) = rules_map.get_mut(second) {
            before.push(first);
        } else {
            rules_map.insert(second, vec![first]);
        }
    }

    for instruction in instructions.iter() {
        let page_order: Vec<&str> = instruction.split(",").collect();
        // check if valid instruction
        let mut valid = false;
        for j in 0..page_order.len() {
            valid = false;
            if let Some(before) = rules_map.get(page_order[j]) {
                if before.iter().all(|previous_page| {
                    if page_order.contains(previous_page) {
                        page_order[0..j].contains(previous_page)
                    } else {
                        true
                    }
                }) {
                    valid = true
                } else {
                    break;
                }
            } else {
                valid = true
            }
        }
        if valid {
            let len = (page_order.len() - 1) / 2;
            println!("{instruction}, {}", page_order[len].parse::<i64>().unwrap());
            total += page_order[len].parse::<i64>().unwrap()
        }
    }
    total
}

#[cfg(test)]
mod test {
    use crate::solve_1;

    #[test]
    fn example() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(143, solve_1(input.to_string(), 21, 22));
    }
}
