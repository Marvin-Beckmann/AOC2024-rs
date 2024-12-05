use std::{collections::HashMap, fs};

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_2(puzzle_input, 1176, 1177));
}

pub fn solve_2(puzzle_input: String, last_rule: usize, first_instr: usize) -> i64 {
    let mut total = 0;
    let input: Vec<&str> = puzzle_input.lines().collect();

    let rules = &input[0..last_rule];
    let instructions: Vec<&str> = input[first_instr..].to_vec();

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

    for instruction in instructions {
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
        if !valid {
            let new_order = order_row(&page_order, &rules_map);

            let len = (page_order.len() - 1) / 2;
            total += new_order[len].parse::<i64>().unwrap()
        }
    }
    total
}

fn order_row(instruction: &Vec<&str>, rules_map: &HashMap<&str, Vec<&str>>) -> Vec<String> {
    let mut new_order: Vec<String> = Vec::new();

    let mut to_add = instruction.clone();
    while !to_add.is_empty() {
        let mut next_add = None;
        let mut j: usize = 0;
        while next_add.is_none() {
            if let Some(before) = rules_map.get(to_add[j]) {
                if before.iter().all(|previous_page| {
                    if instruction.contains(previous_page) {
                        new_order.contains(&(*previous_page).to_owned())
                    } else {
                        true
                    }
                }) {
                    next_add = Some(j)
                }
            }
            j += 1
        }
        new_order.push(to_add[next_add.unwrap()].to_owned());
        to_add.remove(j - 1);
    }
    new_order
}
