use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_2(puzzle_input));
}

pub fn solve_2(puzzle_input: String) -> i64 {
    let mut total = 0;
    let matrix: Vec<Vec<char>> = puzzle_input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let row_max = matrix.len();
    let col_max = matrix[0].len();

    for row in 0..row_max {
        for column in 0..col_max {
            if matrix[row][column] == 'A' {
                let (row, column): (i64, i64) =
                    (row.try_into().unwrap(), column.try_into().unwrap());

                let in_bounds = (row + 1) < row_max.try_into().unwrap()
                    && (row - 1) >= 0
                    && (column + 1) < col_max.try_into().unwrap()
                    && (column - 1) >= 0;
                if in_bounds {
                    let lt_rb = (
                        matrix[TryInto::<usize>::try_into(row - 1).unwrap()]
                            [TryInto::<usize>::try_into(column - 1).unwrap()],
                        matrix[TryInto::<usize>::try_into(row + 1).unwrap()]
                            [TryInto::<usize>::try_into(column + 1).unwrap()],
                    );
                    let lb_rt = (
                        matrix[TryInto::<usize>::try_into(row + 1).unwrap()]
                            [TryInto::<usize>::try_into(column - 1).unwrap()],
                        matrix[TryInto::<usize>::try_into(row - 1).unwrap()]
                            [TryInto::<usize>::try_into(column + 1).unwrap()],
                    );
                    if lt_rb.0 != lt_rb.1
                        && lb_rt.0 != lb_rt.1
                        && ['M', 'S'].contains(&lt_rb.0)
                        && ['M', 'S'].contains(&lt_rb.1)
                        && ['M', 'S'].contains(&lb_rt.0)
                        && ['M', 'S'].contains(&lb_rt.1)
                    {
                        total += 1
                    }
                }
            }
        }
    }

    total
}
