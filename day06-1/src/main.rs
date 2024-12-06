use std::{collections::HashSet, fs};

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_1(puzzle_input));
}

pub fn solve_1(puzzle_input: String) -> i64 {
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
    let input: Vec<Vec<char>> = puzzle_input.lines().map(|x| x.chars().collect()).collect();
    let (mut current_row, mut current_column) = (0, 0);
    for row in 0..input.len() {
        for column in 0..input[0].len() {
            if input[row][column] == '^' {
                (current_row, current_column) = (row, column);
            }
        }
    }

    visited_positions.insert((current_row, current_column));
    let mut direction = Direction::Top;

    loop {
        let next_pos = direction.next(current_row, current_column);
        if let Some(row) = input.get(next_pos.0) {
            if let Some(entry) = row.get(next_pos.1) {
                match entry {
                    '#' => direction = direction.turn_90_degree(),
                    _ => {
                        visited_positions.insert(next_pos);
                        (current_row, current_column) = next_pos
                    }
                };
            } else {
                return visited_positions.len().try_into().unwrap();
            }
        } else {
            return visited_positions.len().try_into().unwrap();
        }
    }
}

pub enum Direction {
    Top,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn turn_90_degree(&self) -> Self {
        match self {
            Direction::Top => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Top,
        }
    }

    pub fn next(&self, row: usize, column: usize) -> (usize, usize) {
        match self {
            Direction::Top => (row - 1, column),
            Direction::Right => (row, column + 1),
            Direction::Down => (row + 1, column),
            Direction::Left => (row, column - 1),
        }
    }
}
