use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(12);

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

    fn same_neighbour(&self, map: &HashMap<Point, char>, target_crop: char) -> Vec<Self> {
        let mut neighbours = Vec::new();
        for dir in Self::dirs() {
            let next = self.add(&dir);
            if map.get(&next).is_some_and(|&crop| crop == target_crop) {
                neighbours.push(next);
            }
        }
        neighbours
    }

    fn dirs() -> [Point; 4] {
        [
            // up
            Point::new(-1, 0),
            // right
            Point::new(0, 1),
            // down
            Point::new(1, 0),
            // left
            Point::new(0, -1),
        ]
    }
    fn perp(&self) -> Self {
        // does not matter if (row, col) turns into (-col, row) or (col, row) for this algorithm
        Point::new(-self.col, self.row)
    }
}

fn parse(input: &str) -> HashMap<Point, char> {
    let rows = input.lines().count() as i32;
    let cols = input.lines().last().unwrap().chars().count() as i32;
    let mut grid = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let point = Point::new(row as i32, col as i32);
            grid.insert(point, ch);
        }
    }
    grid
}

fn shape(
    start: Point,
    crop: char,
    map: &HashMap<Point, char>,
    seen: &mut HashSet<Point>,
) -> HashSet<Point> {
    let mut q = VecDeque::new();
    let mut shape = HashSet::new();
    q.push_back(start);
    shape.insert(start);
    while let Some(point) = q.pop_front() {
        for neighbour in point.same_neighbour(map, crop) {
            if seen.insert(neighbour) {
                shape.insert(neighbour);
                q.push_back(neighbour);
            }
        }
    }
    shape
}
pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);
    let mut seen = HashSet::new();
    let mut sum = 0;
    for (point, corp) in &map {
        if seen.contains(point) {
            continue;
        }
        let shape = shape(*point, *corp, &map, &mut seen);
        let area = shape.len();
        let circumference = circumference(&map, shape);
        sum += circumference * area;
    }
    Some(sum as u32)
}

fn circumference(map: &HashMap<Point, char>, shape: HashSet<Point>) -> usize {
    shape
        .iter()
        .map(|point| 4 - point.same_neighbour(map, *map.get(point).unwrap()).len())
        .sum()
}

fn sides(shape: HashSet<Point>) -> usize {
    let mut sides = HashSet::new();
    for point in &shape {
        for dir in Point::dirs() {
            // look for first out of bounds element in dir
            if shape.contains(&point.add(&dir)) {
                continue;
            }
            println!("{:?} {:?}", point, dir);
            // perpendicular dir
            let perp = dir.perp();
            let mut curr = *point;

            // keep moving in the perpendicular direction while:
            // - a block in the perpendicular direction exists
            // - a block in the original direction doesn't exist
            while shape.contains(&curr.add(&perp)) && !shape.contains(&curr.add(&dir)) {
                curr = curr.add(&perp);
            }
            // when edge was followed, add this (point, dir) to the sides.
            // include dir because 1 point has 4 sides
            sides.insert((curr, dir));
        }
    }
    println!("{:?}", sides);
    sides.len()
}
pub fn part_two(input: &str) -> Option<usize> {
    let map = parse(input);
    let mut seen = HashSet::new();
    let mut sum = 0;
    for (point, crop) in &map {
        if seen.contains(point) {
            continue;
        }
        let shape = shape(*point, *crop, &map, &mut seen);
        let area = shape.len();
        let sides = sides(shape);
        println!("{:?}", sides);
        sum += area * sides;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
