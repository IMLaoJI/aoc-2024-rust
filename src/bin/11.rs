use std::collections::HashMap;

advent_of_code::solution!(11);

fn parse(input: &str) -> HashMap<u64, u64> {
    let mut stones: HashMap<u64, u64> = HashMap::new();
    for num in input.split_ascii_whitespace() {
        let num = num.parse().unwrap();
        *stones.entry(num).or_default() += 1;
    }
    stones
}

fn blink(stones: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new = HashMap::new();
    for (stone, amount) in stones {
        if *stone == 0 {
            *new.entry(1).or_default() += amount;
        } else {
            let digits = stone.ilog10() + 1;
            if digits % 2 == 0 {
                let magnitude = 10u64.pow(digits / 2);
                *new.entry(stone % magnitude).or_default() += amount;
                *new.entry(stone / magnitude).or_default() += amount;
            } else {
                *new.entry(stone * 2024).or_default() += amount;
            }
        }
    }
    new
}

fn both(input: &str) -> (u64, u64) {
    let mut stones = parse(input);
    let mut p1 = 0;
    for i in (0..75) {
        if i == 25 {
            p1 = stones.values().sum();
        }
        stones = blink(&stones);
    }

    (p1, stones.values().sum())
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(both(input).0)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(both(input).1)
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
