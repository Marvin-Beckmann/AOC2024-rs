use std::{collections::HashSet, fs};

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_2(puzzle_input));
}

pub fn solve_2(puzzle_input: String) -> i64 {
    let mut visited_positions: HashSet<(usize, usize, Direction)> = HashSet::new();
    let input: Vec<Vec<char>> = puzzle_input.lines().map(|x| x.chars().collect()).collect();
    let (mut current_row, mut current_column) = (0, 0);
    for row in 0..input.len() {
        for column in 0..input[0].len() {
            if input[row][column] == '^' {
                (current_row, current_column) = (row, column);
            }
        }
    }

    let direction = Direction::Top;
    visited_positions.insert((current_row, current_column, direction));

    let mut total = 0;

    for row in 0..input.len() {
        for column in 0..input[0].len() {
            if input[row][column] != '^' && input[row][column] != '#' {
                let mut modified_input = input.clone();
                modified_input[row][column] = '#';

                if gets_stuck(
                    modified_input,
                    current_row,
                    current_column,
                    direction,
                    &visited_positions,
                ) {
                    total += 1;
                }
            }
        }
    }
    total
}

pub fn gets_stuck(
    input: Vec<Vec<char>>,
    start_row: usize,
    start_column: usize,
    direction: Direction,
    visited_positions: &HashSet<(usize, usize, Direction)>,
) -> bool {
    let mut visited_positions = visited_positions.clone();
    let mut current_row = start_row;
    let mut current_column = start_column;
    let mut direction = direction;
    loop {
        let _next_pos = direction.next(current_row, current_column);
        if _next_pos.is_none() {
            return false;
        }
        let next_pos = _next_pos.unwrap();
        if let Some(row) = input.get(next_pos.0) {
            if let Some(entry) = row.get(next_pos.1) {
                match entry {
                    '#' => direction = direction.turn_90_degree(),
                    _ => {
                        if visited_positions.contains(&(next_pos.0, next_pos.1, direction)) {
                            return true;
                        }
                        visited_positions.insert((next_pos.0, next_pos.1, direction));
                        (current_row, current_column) = next_pos
                    }
                };
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
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

    pub fn next(&self, row: usize, column: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Top => {
                if row == 0 {
                    None
                } else {
                    Some((row - 1, column))
                }
            }
            Direction::Right => Some((row, column + 1)),
            Direction::Down => Some((row + 1, column)),
            Direction::Left => {
                if column == 0 {
                    None
                } else {
                    Some((row, column - 1))
                }
            }
        }
    }
}
