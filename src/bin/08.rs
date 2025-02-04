use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

#[derive(Debug, PartialEq, Eq, Hash,Clone, Copy)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    fn is_in_bounds(&self, rows: i32, cols: i32) -> bool {
        (self.row >= 0 && self.row < rows) && (self.col >= 0 && self.col < cols)
    }

    fn dist(&self, other: &Self) -> Self {
        Point::new(self.row - other.row, self.col - other.col)
    }

    fn add(&self, other: &Self) -> Self {
        Point::new(self.row + other.row, self.col + other.col)
    }
}

fn parse(input: &str) -> (i32, i32, HashMap<char, Vec<Point>>) {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();

    input.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, ch)| {
            if ch != '.' {
                antennas
                    .entry(ch)
                    .or_default()
                    .push(Point::new(row as i32, col as i32))
            }
        })
    });
    (rows as i32, cols as i32, antennas)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rows, cols, antennas) = parse(input);
    let mut antinodes = HashSet::new();
    antennas.values().for_each(|points| {
        for p1 in points {
            for p2 in points {
                if p1 == p2 {
                    continue;
                }
                let dist = p1.dist(p2);
                let new = p1.add(&dist);
                if new.is_in_bounds(rows, cols) {
                    antinodes.insert(new);
                }
            }
        }
    });
    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rows, cols, antennas) = parse(input);
    let mut antinodes = HashSet::new();
    for points in antennas.values() {
        for p1 in points {
            for p2 in points {
                if p1 == p2 {
                    continue;
                }
                let dist = p1.dist(p2);
                let mut new = *p1;
                while new.is_in_bounds(rows, cols) {
                    antinodes.insert(new.clone());
                    new = new.add(&dist);
                }
            }
        }
    }
    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
