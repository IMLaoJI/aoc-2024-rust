
advent_of_code::solution!(7);

fn parse(input: &str) -> impl Iterator<Item = (u64, Vec<u64>)> + '_ {
    input.lines().map(|line| {
        let (target, numbers) = line.split_once(": ").unwrap();
        (
            target.parse().unwrap(),
            numbers
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        )
    })
}

fn is_reachable(goal: u64, nums: &[u64], concat: bool) -> bool {
    if nums.len() == 1 {
        return goal == nums[0];
    }
    let (&last, rest) = nums.split_last().unwrap();
    if goal % last == 0 && is_reachable(goal / last, rest, concat) {
        return true;
    }
    if goal > last && is_reachable(goal - last, rest, concat) {
        return true;
    }
    if concat {
        let last_len = last.ilog10() + 1;
        let magnitude = 10u64.pow(last_len);
        let goal_len = goal.ilog10() + 1;
        let ending = goal % magnitude;
        if goal_len > last_len && last == ending && is_reachable(goal / magnitude, rest, concat)
        {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .filter(|(goal, nums)| is_reachable(*goal, nums, false))
            .map(|(goal, _)| goal)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .filter(|(goal, nums)| is_reachable(*goal, nums, true))
            .map(|(goal, _)| goal)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
