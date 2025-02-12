use itertools::Itertools;

advent_of_code::solution!(14);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    row: i64,
    col: i64,
}

impl Point {
    fn new(row: i64, col: i64) -> Self {
        Self { row, col }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Robot {
    pos: Point,
    vel: Point,
}

impl Robot {
    fn new(p_row: i64, p_col: i64, v_row: i64, v_col: i64) -> Self {
        Self {
            pos: Point::new(p_row, p_col),
            vel: Point::new(v_row, v_col),
        }
    }
}

const ROWS: i64 = 103;
const COLS: i64 = 101;
fn parse(input: &str) -> Vec<Robot> {
    let mut robots = Vec::new();
    for line in input.lines() {
        let mut nums = Vec::new();
        for num in line
            .split(|c: char| !c.is_ascii_digit() && c != '-')
            .filter(|s| !s.is_empty())
        {
            let num: i64 = num.parse().unwrap();
            nums.push(num);
        }
        // attention! because x is col and y is row this order might not be what you expect!
        let robot = Robot::new(nums[1], nums[0], nums[3], nums[2]);
        robots.push(robot);
    }
    robots
}

fn safety(robots: &[Robot]) -> usize {
    let mut sectors = [0; 4];
    let col_mid = COLS / 2;
    let row_mid = ROWS / 2;
    for &Robot { pos, .. } in robots {
        if pos.row == row_mid || pos.col == col_mid {
            continue;
        }
        let top = pos.row < row_mid;
        let left = pos.col < col_mid;
        match (top, left) {
            (true, true) => sectors[0] += 1,
            (true, false) => sectors[1] += 1,
            (false, true) => sectors[2] += 1,
            (false, false) => sectors[3] += 1,
        }
    }
    sectors.iter().product()
}


pub fn part_one(input: &str) -> Option<i64> {
    let mut robots = parse(input);
    for Robot { pos, vel } in &mut robots {
        pos.row = (pos.row + vel.row * 100).rem_euclid(ROWS);
        pos.col = (pos.col + vel.col * 100).rem_euclid(COLS);
    }
    Some(safety(&robots) as i64)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots = parse(input);
    for i in 0.. {
        for Robot { pos, vel } in &mut robots {
            pos.row = (pos.row + vel.row).rem_euclid(ROWS);
            pos.col = (pos.col + vel.col).rem_euclid(COLS);
        }
        if robots.iter().map(|robot| robot.pos).all_unique() {
            return Some(i + 1);
        }
    }
    panic!("No tree");
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
