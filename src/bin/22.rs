use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut num = line.parse().unwrap();
                for _ in 0..2000 {
                    num = solve(num);
                }
                num
            })
            .sum(),
    )
}

fn mix(a: i64, b: i64) -> i64 {
    a ^ b
}

fn prune(a: i64) -> i64 {
    a.rem_euclid(16777216)
}

fn solve(mut num: i64) -> i64 {
    num = prune(mix(num * 64, num));
    num = prune(mix(num / 32, num));
    num = prune(mix(num * 2048, num));
    num
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut profitmap = HashMap::new();
    for line in input.lines() {
        let mut num = line.parse().unwrap();
        let mut prices = [0; 2000];
        for price in prices.iter_mut() {
            num = solve(num);
            *price = num.rem_euclid(10);
        }

        let mut seen = HashSet::new();
        for (a, b, c, d, e) in prices.iter().tuple_windows() {
            let diffs = ((b - a), (c - b), (d - c), (e - d));
            if seen.insert(diffs) {
                *profitmap.entry(diffs).or_default() += *e;
            }
        }
    }
    Some(*profitmap.values().max().unwrap())
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
