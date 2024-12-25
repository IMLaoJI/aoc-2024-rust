use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let nums: Vec<u32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        left.push(nums[0]);
        right.push(nums[1]);
    }

    (left, right)
}
pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse(input);
    left.sort();
    right.sort();
    let mut sum = 0;
    for (l, r) in left.iter().zip(right) {
        sum +=l.abs_diff(r);
    }
    Some(sum)
}

pub fn part_one_iter(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse(input);
    left.sort_unstable();
    right.sort_unstable();

    let sum = left.iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum();
    Some(sum)
}

// pub fn part_two(input: &str) -> Option<u32> {
//     let (left, right) = parse(input);
//
//     let mut sum = 0;
//
//     for l in &left {
//         let product = l * right.iter().filter(|&r| l == r).count() as u32;
//         sum += product;
//     }
//
//     Some(sum)
// }

pub fn part_two_iter(input: &str) -> Option<u32> {
    let (first_elements, second_elements): (Vec<u32>, Vec<u32>) = parse(input.trim());
    Some(first_elements.iter().fold(0, |acc, b| {
        acc + b * second_elements.iter().filter(|item| *item == b).count() as u32
    }))
}


pub fn part_two(input: &str) -> Option<u32> {
   let (left,right) = parse(input);
    let counts: HashMap<u32, u32> = right.into_iter().fold(HashMap::new(), |mut acc, r| {
        *acc.entry(r).or_default() += 1;
        acc
    });

    let res = left.into_iter()
        .map(|l| l * counts.get(&l).copied().unwrap_or_default())
        .sum();
    Some(res)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_iter(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
