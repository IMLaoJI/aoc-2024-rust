use std::collections::HashMap;

advent_of_code::solution!(19);

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let ( towels,  pattern) = input.split_once("\r\n\r\n").unwrap();
    let towels = towels.split(", ").collect();
    let patterns = pattern.lines().collect();
    (towels, patterns)
}

fn count<'a>(pattern: &'a str, towels: &[&str], cache: &mut HashMap<&'a str, usize>) -> usize {
    if pattern.is_empty() {
        return 1;
    }
    if let Some(&num) = cache.get(pattern) {
        return num;
    }
    let towels_num = towels
        .iter()
        .filter(|&towel| pattern.starts_with(towel))
        .map(|towel| count(&pattern[towel.len()..], towels, cache))
        .sum();
    cache.insert(pattern, towels_num);
    towels_num
}

pub fn part_one(input: &str) -> Option<u32> {
    let (towels, patterns) = parse(input);
    let mut cache = HashMap::new();
    Some(
        patterns
            .iter()
            .filter(|pattern| count(pattern, &towels, &mut cache) != 0)
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (towels, patterns) = parse(input);
    let mut cache = HashMap::new();
    Some(
        patterns
            .iter()
            .map(|pattern| count(pattern, &towels, &mut cache))
            .sum(),
    )
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
        assert_eq!(result, Some(569808947758890));
    }
}
