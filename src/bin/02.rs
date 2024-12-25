use std::ops::Sub;

advent_of_code::solution!(2);

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .split('\n')
        .map(|line| {
            line.trim_end()
                .split(" ")
                .filter_map(|line| line.parse::<i32>().ok())
                .collect()
        })
        .collect()
}

fn is_save(v: Vec<i32>) -> bool {
    let v_zip: Vec<_> = v.iter().zip(v.iter().skip(1)).collect();
    v_zip
        .iter()
        .all(|(v1   , v2)| v2.sub(*v1) >= 1 && v2.sub(*v1) <= 3)
        || v_zip
            .iter()
            .all(|(v1, v2)| v2.sub(*v1) <= -1 && v2.sub(*v1) >= -3)
}
pub fn part_one(input: &str) -> Option<u32> {
    let res = parse(input).iter().filter(|v| is_save(v.to_vec())).count();
    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = parse(input)
        .iter()
        .filter(|v| {
            v.iter().enumerate().any(|(i, _)| {
                let new_v = v
                    .iter()
                    .enumerate()
                    .filter(|(idx, _)| !idx.eq(&i))
                    .map(|(_, &value)| value)
                    .collect();
                is_save(new_v)
            })
        })
        .count();
    Some(res as u32)
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
