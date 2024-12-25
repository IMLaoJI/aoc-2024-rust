use itertools::*;
use std::ops::Sub;
advent_of_code::solution!(2);

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn is_safe(report: &[i32]) -> bool {
    let dir = (report[0] - report[1]).signum();
    for (a, b) in report.iter().tuple_windows() {
        let diff = a - b;
        if diff == 0 || diff.abs() > 3 || diff.signum() != dir {
            return false;
        }
    }
    true
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;
    for line in input.lines() {
        let vec = parse_line(line);
        if (is_safe(&vec)) {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_one_iter(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(parse_line)
            .filter(|report| is_safe(report))
            .count() as u32,
    )
}

fn is_safe_p2(report: &[i32]) -> bool {
    for idx in 0..report.len() {
        let mut report = report.to_vec();
        report.remove(idx);
        if is_safe(&report) {
            return true;
        }
    }
    false
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;
    for line in input.lines() {
        let report = parse_line(line);
        if is_safe_p2(&report) {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two_iter(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(parse_line)
            .filter(|report| is_safe_p2(&report))
            .count() as u32,
    )
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
