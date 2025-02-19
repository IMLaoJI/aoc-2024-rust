use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(18);

const ROWS: i32 = 71;
const COLS: i32 = 71;
const SIZE: usize = 1024;

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn new(row: i32, col: i32) -> Point {
        Self { row, col }
    }
    fn add(self, other: &Point) -> Point {
        Self::new(self.row + other.row, self.col + other.col)
    }
    fn dirs() -> [Point; 4] {
        [
            // up
            Self::new(-1, 0),
            // right
            Self::new(0, 1),
            // down
            Self::new(1, 0),
            // left
            Self::new(0, -1),
        ]
    }

    fn is_in_bound(&self, rows: i32, cols: i32) -> bool {
        (self.row >= 0 && self.row < rows) && (self.col >= 0 && self.col < cols)
    }

    fn neighbour(&self, rows: i32, cols: i32) -> Vec<Point> {
        let mut neighbours = Vec::new();
        for dir in Self::dirs() {
            let next = self.add(&dir);
            if Self::is_in_bound(&next, rows, cols) {
                neighbours.push(next);
            }
        }
        neighbours
    }
}

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let (col, row) = line.split_once(',').unwrap();
            Point::new(row.parse().unwrap(), col.parse().unwrap())
        })
        .collect_vec()
}

fn search(corrupted: HashSet<&Point>, start: Point, end: Point) -> Option<u32> {
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    q.push_back((start, 0));
    seen.insert(start);
    while let Some((pos, cost)) = q.pop_front() {
        if pos == end {
            return Some(cost);
        }
        for neighbour in pos.neighbour(ROWS, COLS) {
            if corrupted.contains(&neighbour) {
                continue;
            }
            if seen.insert(neighbour) {
                q.push_back((neighbour, cost + 1));
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let falling = parse(input);
    let corrupted: HashSet<&Point> = falling.iter().take(SIZE).collect();
    let start = Point::new(0, 0);
    let end = Point::new(ROWS - 1, COLS - 1);
    search(corrupted, start, end)
}

pub fn part_two(input: &str) -> Option<String> {
    let falling = parse(input);
    let start = Point::new(0, 0);
    let end = Point::new(ROWS - 1, COLS - 1);

    let mut low = 0;
    let mut high = falling.len() - 1;

    while low < high {
        let mid = (low + high) / 2;
        let corrupted = falling.iter().take(mid).collect();
        if search(corrupted, start, end).is_some() {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    let last = falling[low];
    Some(format!("{},{}", last.col, last.row))
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
