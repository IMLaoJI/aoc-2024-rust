advent_of_code::solution!(9);

#[derive(Debug,Clone, Copy, PartialEq, Eq)]
enum Block {
    Empty,
    File(usize),
}

fn parse(input: &str) -> Vec<(usize, Block)> {
    let mut file_system = Vec::new();
    let mut file_id = 0;

    for (i, b) in input.bytes().enumerate() {
        let block = if i % 2 == 0 {
            let file = Block::File(file_id);
            file_id += 1;
            file
        } else {
            Block::Empty
        };
        file_system.push(((b - b'0') as usize, block));
    }

    file_system
}

pub fn part_one(input: &str) -> Option<u32> {
    let vec = parse(input)
        .iter()
        .flat_map(|&(size, item)| (0..size).map(move |_| item))
        .collect::<Vec<_>>();
    println!("{:?}", vec);
    todo!()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
