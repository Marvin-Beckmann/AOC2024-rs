use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_2(puzzle_input));
}

pub fn solve_2(puzzle_input: String) -> i64 {
    puzzle_input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|elem| elem.parse::<i64>().unwrap())
                .collect()
        })
        .fold(0, |acc: i64, level| {
            if is_safe(level, Mode::Undef, 0) {
                acc + 1
            } else {
                acc
            }
        })
}

fn is_safe(level: Vec<i64>, mode: Mode, err_count: i64) -> bool {
    if err_count > 1 {
        return false;
    }
    // determine mode
    let mode = match mode {
        Mode::Undef => {
            if (0..4)
                .map(|i| level[i] < level[i + 1])
                .fold(0, |acc, x| if x { acc + 1 } else { acc })
                >= 3
            {
                Mode::Asc
            } else {
                Mode::Desc
            }
        }
        _ => mode,
    };
    for i in 0..(level.len() - 1) {
        let mut level_err_i = level.clone();
        level_err_i.remove(i);
        let mut level_err_i_1 = level.clone();
        level_err_i_1.remove(i + 1);
        if (level[i] - level[i + 1]).abs() > 3 {
            return is_safe(level_err_i, mode, err_count + 1)
                || is_safe(level_err_i_1, mode, err_count + 1);
        }
        let (x, y) = (level.get(i).unwrap(), level.get(i + 1).unwrap());
        match mode {
            Mode::Asc => {
                if x >= y {
                    return is_safe(level_err_i, mode, err_count + 1)
                        || is_safe(level_err_i_1, mode, err_count + 1);
                }
            }
            Mode::Desc => {
                if y >= x {
                    return is_safe(level_err_i, mode, err_count + 1)
                        || is_safe(level_err_i_1, mode, err_count + 1);
                }
            }
            _ => panic!(),
        };
    }
    true
}

#[derive(Clone, Copy)]
pub enum Mode {
    Asc,
    Desc,
    Undef,
}
