use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_2(puzzle_input));
}

pub fn solve_2(puzzle_input: String) -> usize {
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
    let mut end = Some(fs_.file_system.len());
    while end.is_some() {
        end = fs_.switch_last_to_free_blocks(end);
    }

    // calculate checksum
    let mut id = 0;
    let mut sum = 0;
    for val in fs_.file_system.iter() {
        println!("{:?}", val);
        if val.is_some() {
            sum += val.unwrap() * id;
            id += 1;
        } else {
            // not correct imo
            id += 1;
        }
    }
    sum
}

pub struct FileSystem {
    file_system: Vec<Option<usize>>,
}

impl FileSystem {
    /// The parameters start and end just describe a potential range in which to operate,
    /// otherwise the search for the next element will start from the start respectively the end.
    fn switch_last_to_free_blocks(&mut self, end: Option<usize>) -> Option<usize> {
        // let start = start.unwrap_or(0);
        let end = end.unwrap_or(self.file_system.len());
        if end == 1 {
            return None;
        }

        let last_occupied_space = self.file_system[0..end]
            .iter()
            .enumerate()
            .rev()
            .find(|(_, x)| x.is_some())
            .unwrap()
            .0;
        let mut block_size = 1;
        while last_occupied_space > block_size
            && self.file_system[last_occupied_space]
                == self.file_system[last_occupied_space - block_size]
        {
            block_size += 1
        }

        let first_free_space = (0..last_occupied_space).find(|i| {
            (0..block_size).all(|x| self.file_system.get(i + x).is_some_and(|x| x.is_none()))
        });

        if first_free_space.is_none() {
            return self.switch_last_to_free_blocks(Some(last_occupied_space - block_size + 1));
        }

        for x in 0..block_size {
            self.file_system
                .swap(first_free_space.unwrap() + x, last_occupied_space - x);
        }

        Some(last_occupied_space)
    }
}

#[cfg(test)]
mod test {
    use crate::solve_2;

    const TEST_1: &str = "2333133121414131402";

    #[test]
    fn test() {
        println!("{}", solve_2(TEST_1.to_string()));
    }
}
