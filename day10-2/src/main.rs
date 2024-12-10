use std::{collections::HashSet, fs};

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_1(puzzle_input));
}

pub fn solve_1(puzzle_input: String) -> usize {
    let input: Vec<Vec<u32>> = puzzle_input
        .lines()
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut count = 0;
    let max_row = input.len() - 1;
    let max_column = input[0].len() - 1;
    for row in 0..=max_row {
        for column in 0..=max_column {
            if input[row][column] == 0 {
                println!("{count}");
                count += find_full_hiking_tracks(row, column, max_row, max_column, &input);
            }
        }
    }

    count
}

fn find_full_hiking_tracks(
    start_row: usize,
    start_column: usize,
    max_row: usize,
    max_column: usize,
    input: &Vec<Vec<u32>>,
) -> usize {
    let mut current_height = 0;
    let mut current_positions = Vec::new();
    current_positions.push((start_row, start_column));

    while current_height < 9 {
        current_height += 1;
        let mut new_current_positions = Vec::new();
        for (x, y) in current_positions {
            let new_positions = return_neighbour_coordinates(x, y, max_row, max_column);
            for (nx, ny) in new_positions {
                if input[nx][ny] == current_height {
                    new_current_positions.push((nx, ny));
                }
            }
        }
        current_positions = new_current_positions;

        println!("{:?}", current_positions);
    }

    current_positions.len()
}

fn return_neighbour_coordinates(
    row: usize,
    column: usize,
    max_row: usize,
    max_column: usize,
) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    if row != 0 {
        out.push((row - 1, column));
    }
    if row < max_row {
        out.push((row + 1, column));
    }
    if column != 0 {
        out.push((row, column - 1));
    }
    if column < max_column {
        out.push((row, column + 1));
    }

    out
}

#[cfg(test)]
mod test {
    use crate::solve_1;

    const TEST_1: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test() {
        assert_eq!(81, solve_1(TEST_1.to_string()));
    }
}
