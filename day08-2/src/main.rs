use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_1(puzzle_input));
}

pub fn solve_1(puzzle_input: String) -> usize {
    let mut locations_antinodes = HashSet::new();
    //let mut locations_antenna = HashSet::new();
    let mut frequencies: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    let map: Vec<Vec<char>> = puzzle_input.lines().map(|x| x.chars().collect()).collect();

    let max_row = map.len() - 1;
    let max_column = map[0].len() - 1;

    for row in 0..=max_row {
        for column in 0..=max_column {
            if map[row][column] != '.' {
                //locations_antenna.insert((row.try_into().unwrap(), column.try_into().unwrap()));
                if let Some(list) = frequencies.get_mut(&map[row][column]) {
                    list.push((row.try_into().unwrap(), column.try_into().unwrap()));
                } else {
                    frequencies.insert(
                        map[row][column],
                        vec![(row.try_into().unwrap(), column.try_into().unwrap())],
                    );
                }
            }
        }
    }

    for key in frequencies.keys() {
        let all_positions = compute_all_positions(frequencies.get(key).unwrap().to_vec());
        for pos in all_positions {
            if pos.0 >= 0
                && pos.0 <= max_row.try_into().unwrap()
                && pos.1 >= 0
                && pos.1 <= max_column.try_into().unwrap()
            // && !locations_antenna.contains(&pos)
            {
                locations_antinodes.insert(pos);
            }
        }
    }

    locations_antinodes.len()
}

pub fn compute_all_positions(all_positions: Vec<(i64, i64)>) -> HashSet<(i64, i64)> {
    let mut positions = HashSet::new();

    for pos_1 in &all_positions {
        for pos_2 in &all_positions {
            let difference = (pos_1.0 - pos_2.0, pos_1.1 - pos_2.1);

            if difference != (0, 0) {
                // in any direction you can go at most 50 times before going out of bounds
                // as 50 is the maximal number of rows and columns in our example
                for i in 0..50 {
                    positions.insert((pos_1.0 + i * difference.0, pos_1.1 + i * difference.1));
                    positions.insert((pos_2.0 - i * difference.0, pos_2.1 - i * difference.1));
                }
            }
        }
    }
    positions
}
