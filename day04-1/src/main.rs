use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_1(puzzle_input));
}

pub fn solve_1(puzzle_input: String) -> i64 {
    let mut total = 0;
    let matrix: Vec<Vec<char>> = puzzle_input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let row_max = matrix.len();
    let col_max = matrix[0].len();

    for row in 0..row_max {
        for column in 0..col_max {
            if matrix[row][column] == 'X' {
                let (row, column): (i64, i64) =
                    (row.try_into().unwrap(), column.try_into().unwrap());
                for (dir_row, dir_column) in [
                    (1, 0),
                    (1, 1),
                    (0, 1),
                    (-1, 1),
                    (-1, 0),
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                ] {
                    let in_bounds = (row + 3 * dir_row) < row_max.try_into().unwrap()
                        && (row + 3 * dir_row) >= 0
                        && (column + 3 * dir_column) < col_max.try_into().unwrap()
                        && (column + 3 * dir_column) >= 0;
                    if in_bounds {
                        let m = matrix[TryInto::<usize>::try_into(row + dir_row).unwrap()]
                            [TryInto::<usize>::try_into(column + dir_column).unwrap()]
                            == 'M';
                        let a = matrix[TryInto::<usize>::try_into(row + 2 * dir_row).unwrap()]
                            [TryInto::<usize>::try_into(column + 2 * dir_column).unwrap()]
                            == 'A';
                        let s = matrix[TryInto::<usize>::try_into(row + 3 * dir_row).unwrap()]
                            [TryInto::<usize>::try_into(column + 3 * dir_column).unwrap()]
                            == 'S';
                        if m && a && s {
                            total += 1
                        }
                    }
                }
            }
        }
    }

    total
}
