use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_1(puzzle_input));
}

pub fn solve_1(puzzle_input: String) -> i64 {
    puzzle_input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|elem| elem.parse::<i64>().unwrap())
                .collect()
        })
        .fold(0, |acc: i64, level| {
            if is_safe(level, Mode::Undef) {
                acc + 1
            } else {
                acc
            }
        })
}

fn is_safe(level: Vec<i64>, mode: Mode) -> bool {
    if let (Some(x), Some(y)) = (level.get(0), level.get(1)) {
        if (x - y).abs() > 3 {
            return false;
        }
        let new_level = level[1..level.len()].to_vec();
        match mode {
            Mode::Asc => {
                if y > x {
                    return is_safe(new_level, Mode::Asc);
                } else {
                    return false;
                }
            }
            Mode::Desc => {
                if y < x {
                    return is_safe(new_level, Mode::Desc);
                } else {
                    return false;
                }
            }
            Mode::Undef => {
                if x < y {
                    return is_safe(new_level, Mode::Asc);
                } else if x > y {
                    return is_safe(new_level, Mode::Desc);
                } else {
                    return false;
                    //return is_safe(new_level, Mode::Undef);
                }
            }
        }
    } else {
        true
    }
}

pub enum Mode {
    Asc,
    Desc,
    Undef,
}
