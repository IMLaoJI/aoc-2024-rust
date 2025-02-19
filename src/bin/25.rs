use std::collections::{HashMap, HashSet};

advent_of_code::solution!(25);

fn parse(input: &str) -> (HashSet<[u8; 5]>, HashSet<[u8; 5]>) {
    let mut locks = HashSet::new();
    let mut keys = HashSet::new();
    for block in input.split("\r\n\r\n") {
        let set = if block.starts_with('.') {
            &mut keys
        } else {
            &mut locks
        };
        let mut heights = [0; 5];
        for line in block.lines() {
            for (idx, c) in line.chars().enumerate() {
                if c == '#' {
                    heights[idx] += 1;
                }
            }
        }
        set.insert(heights);
    }
    (locks, keys)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (locks, keys) = parse(input);
    let mut sum = 0;
    for lock in locks {
        for key in &keys {
            if lock.iter().zip(key).all(|(l, k)| l + k <= 7) {
                sum += 1;
            }
        }
    }
    Some(sum)
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
