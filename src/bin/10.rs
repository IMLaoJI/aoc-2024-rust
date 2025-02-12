use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(10);

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    fn add(&self, other: &Self) -> Self {
        Point::new(self.row + other.row, self.col + other.col)
    }

    fn is_in_bound(&self, rows: i32, cols: i32) -> bool {
        (self.row >= 0 && self.row < rows) && (self.col >= 0 && self.col < cols)
    }

    fn neighbours(&self, rows: i32, cols: i32) -> Vec<Self> {
        let mut neighbours = Vec::new();
        let dirs = [
            // up
            Point::new(-1, 0),
            // right
            Point::new(0, 1),
            // down
            Point::new(1, 0),
            // left
            Point::new(0, -1),
        ];
        for dir in dirs {
            let next = self.add(&dir);
            if next.is_in_bound(rows, cols) {
                neighbours.push(next);
            };
        }
        neighbours
    }
}

fn parse(input: &str) -> (i32, i32, HashMap<Point, u32>) {
    let rows = input.lines().count() as i32;
    let cols = input.lines().last().unwrap().chars().count() as i32;
    let mut grid = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let point = Point::new(row as i32, col as i32);
            grid.insert(point, ch.to_digit(10).unwrap());
        }
    }

    (rows, cols, grid)
}

fn both(input: &str) -> (usize, usize) {
    let (rows, cols, grid) = parse(input);
    let starts = grid
        .iter()
        .filter_map(|(point, &height)| (height == 0).then_some(*point));
    let mut nines = 0;
    let mut routes = 0;
    for start in starts {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        let mut ending = Vec::new();
        queue.push_back((start, 0));
        seen.insert(start);

        while let Some((point, height)) = queue.pop_front() {
            if height == 9 {
                ending.push(point);
                continue;
            }

            for neighbour in point.neighbours(rows, cols) {
                let new_height = grid[&neighbour];
                if new_height != height + 1 {
                    continue;
                }
                queue.push_back((neighbour, new_height));
            }
        }
        routes += ending.len();
        nines += ending.iter().collect::<HashSet<_>>().len();
    }
    (nines, routes)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(both(input).0 as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(both(input).1 as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
