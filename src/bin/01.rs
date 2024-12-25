advent_of_code::solution!(1);

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let mut nums = line.split_whitespace().map(|s| s.parse::<u32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip()
}
pub fn part_one(input: &str) -> Option<u32> {
    let (mut first_elements, mut second_elements): (Vec<u32>, Vec<u32>) = parse(input.trim());
    first_elements.sort_unstable();
    second_elements.sort_unstable();
    Some(
        first_elements
            .iter()
            .zip(second_elements.iter())
            .map(|(a, b)| a.abs_diff(*b))
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (first_elements, second_elements): (Vec<u32>, Vec<u32>) = parse(input.trim());
    Some(first_elements.iter().fold(0, |acc, b| {
        acc + b * second_elements.iter().filter(|item| *item == b).count() as u32
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
