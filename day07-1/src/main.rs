use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_1(puzzle_input));
}

pub fn solve_1(puzzle_input: String) -> usize {
    puzzle_input
        .lines()
        .map(|x| {
            let (target, remainder) = x.split_once(": ").unwrap();
            let target = target.parse::<usize>().unwrap();
            let others: Vec<usize> = remainder
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            match compute_all_combinations(others[0], &others[1..]).contains(&target) {
                true => target,
                false => 0,
            }
        })
        .sum()
}

pub fn compute_all_combinations(carry_over: usize, others: &[usize]) -> Vec<usize> {
    let mut sol = Vec::new();

    let sol_1 = carry_over + others[0];
    let sol_2 = carry_over * others[0];

    if others.len() > 1 {
        sol.append(&mut compute_all_combinations(sol_1, &others[1..]));
        sol.append(&mut compute_all_combinations(sol_2, &others[1..]));
    } else {
        sol.push(sol_1);
        sol.push(sol_2);
    }

    sol
}
