use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_1(puzzle_input));
}

pub fn solve_1(puzzle_input: String) -> usize {
    let mut file_system = Vec::new();
    for (i, char) in puzzle_input.chars().enumerate() {
        let disk_space = char.to_digit(10).unwrap();
        for _ in 0..disk_space {
            if i % 2 == 0 {
                file_system.push(Some(i / 2));
            } else {
                file_system.push(None);
            }
        }
    }

    let mut fs_ = FileSystem { file_system };
    let mut range = Some((0, fs_.file_system.len()));
    while range.is_some() {
        range = fs_.switch_last_to_free(Some(range.unwrap().0), Some(range.unwrap().1));
    }

    // calculate checksum
    fs_.file_system
        .iter()
        .enumerate()
        .map(|(i, val)| i * val.unwrap_or(0))
        .sum()
}

pub struct FileSystem {
    file_system: Vec<Option<usize>>,
}

impl FileSystem {
    /// The parameters start and end just describe a potential range in which to operate,
    /// otherwise the search for the next element will start from the start respectively the end.
    fn switch_last_to_free(
        &mut self,
        start: Option<usize>,
        end: Option<usize>,
    ) -> Option<(usize, usize)> {
        let start = start.unwrap_or(0);
        let end = end.unwrap_or(self.file_system.len());

        let first_free_space = self
            .file_system
            .iter()
            .enumerate()
            .find(|(_, x)| x.is_none())
            .unwrap()
            .0;
        let last_occupied_space = self
            .file_system
            .iter()
            .enumerate()
            .rev()
            .find(|(_, x)| x.is_some())
            .unwrap()
            .0;

        if first_free_space > last_occupied_space {
            None
        } else {
            self.file_system.swap(first_free_space, last_occupied_space);

            Some((first_free_space, last_occupied_space))
        }
    }
}
