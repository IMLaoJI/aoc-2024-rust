#![feature(is_sorted)]

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, pages) = input.split_once("\r\n\r\n").unwrap();

    let mut orderings: HashMap<u32, HashSet<u32>> = HashMap::new();
    for rule in rules.lines() {
        let (n1, n2) = rule.split_once('|').unwrap();
        orderings
            .entry(n2.parse().unwrap())
            .or_default()
            .insert(n1.parse().unwrap());
    }

    let mut updates: Vec<Vec<u32>> = vec![];
    for page in pages.lines() {
        let mut update = vec![];
        for num in page.split(',') {
            update.push(num.parse().unwrap());
        }
        updates.push(update);
    }

    let mut sum = 0;

    for update in updates {
        if update.is_sorted_by(|a, b| orderings.contains_key(b) && orderings[b].contains(a)) {
            sum += update[update.len() / 2];
        }
    }
    Some(sum)
}

pub fn part_one_another(input: &str) -> Option<u32> {
    let (rules, pages) = input.split_once("\r\n\r\n").unwrap();
    let mut orderings: HashMap<(u32, u32), Ordering> = HashMap::new();
    for rule in rules.lines() {
        let (n1, n2) = rule
            .split_once('|')
            .map(|(n1, n2)| (n1.parse::<u32>().unwrap(), n2.parse::<u32>().unwrap()))
            .unwrap();
        orderings.insert((n1, n2), Ordering::Less);
        orderings.insert((n2, n1), Ordering::Greater);
    }
    let mut sum = 0;
    for page in pages.lines() {
        let update = page
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u32>>();
        if update.is_sorted_by(|&a, &b| orderings.get(&(a, b)) == Some(&Ordering::Less)) {
            sum += update[update.len() / 2];
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, pages) = input.split_once("\r\n\r\n").unwrap();
    let mut orderings: HashMap<(u32, u32), Ordering> = HashMap::new();
    for rule in rules.lines() {
        let (n1, n2) = rule
            .split_once('|')
            .map(|(n1, n2)| (n1.parse::<u32>().unwrap(), n2.parse::<u32>().unwrap()))
            .unwrap();
        orderings.insert((n1, n2), Ordering::Less);
        orderings.insert((n2, n1), Ordering::Greater);
    }
    let mut sum = 0;
    for page in pages.lines() {
        let mut update = page
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u32>>();
        if !update.is_sorted_by(|&a, &b| orderings.get(&(a, b)) == Some(&Ordering::Less)) {
            update.sort_by(|&a, &b| *orderings.get(&(a, b)).unwrap_or(&Ordering::Equal));
            sum += update[update.len() / 2];
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_another(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
