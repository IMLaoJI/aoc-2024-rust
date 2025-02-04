advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

pub fn part_one(input: &str) -> Option<usize> {
    let mut filesystem = parse(input)
        .iter()
        .flat_map(|&(size, item)| (0..size).map(move |_| item))
        .collect::<Vec<_>>();
    let mut i = filesystem.len() - 1;
    while i > 0 {
        if filesystem[i] == Block::Empty {
            i -= 1;
            continue;
        }
        let empty_pos = filesystem[0..i].iter().position(|&x| x == Block::Empty);
        if let Some(pos) = empty_pos {
            filesystem.swap(i, pos);
        }
        i -= 1;
    }
    Some(
        filesystem
            .iter()
            .enumerate()
            .filter_map(|(idx, item)| match item {
                Block::Empty => None,
                Block::File(file_id) => Some(idx * file_id),
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut filesystem = parse(input);
    let mut i = filesystem.len() - 1;
    while i > 0 {
        let (curr_size, curr_item) = filesystem[i];
        if curr_item == Block::Empty {
            i -= 1;
            continue;
        }
        let empty_pos = filesystem[0..i]
            .iter()
            .position(|&(size, item)| item == Block::Empty && size >= curr_size);
        if let Some(pos) = empty_pos {
            let empty_size = filesystem[pos].0;
            filesystem[pos] = (curr_size, curr_item);
            filesystem[i] = (curr_size, Block::Empty);
            if empty_size > curr_size {
                let remaining_empty = empty_size - curr_size;
                filesystem.insert(pos + 1, (remaining_empty, Block::Empty));
            }
        }
        i -= 1;
    }

    Some(
        filesystem
            .iter()
            .flat_map(|&(size, item)| (0..size).map(move |_| item))
            .enumerate()
            .flat_map(|(idx, item)| match item {
                Block::Empty => None,
                Block::File(file_id) => Some(idx * file_id),
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
