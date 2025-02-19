use advent_of_code::Point;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(20);

#[derive(Eq, Ord, Clone, Copy, Hash, PartialEq, PartialOrd, Debug)]
enum Tile {
    Empty,
    Wall,
}
fn parse(input: &str) -> (Point, Point, HashMap<Point, Tile>) {
    let mut start = Point::new(0, 0);
    let mut end = Point::new(0, 0);
    let mut grid = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            let point = Point::new(row as i32, col as i32);
            let tile = match char {
                '#' => Tile::Wall,
                '.' => Tile::Empty,
                'E' => {
                    start.row = row as i32;
                    start.col = col as i32;
                    Tile::Empty
                }
                'S' => {
                    end.row = row as i32;
                    end.col = col as i32;
                    Tile::Empty
                }
                _ => panic!("Unexpected character in input: {}", char),
            };

            grid.insert(point, tile);
        }
    }
    (start, end, grid)
}

fn neighbours(point: &Point, grid: &HashMap<Point, Tile>) -> Vec<Point> {
    let mut neighbours = Vec::new();

    for dir in Point::dirs() {
        let next = point.add(&dir);
        if grid.contains_key(&next) {
            neighbours.push(next);
        }
    }
    neighbours
}

fn build_distmap(grid: HashMap<Point, Tile>, start: Point, end: Point) -> HashMap<Point, u32> {
    let mut q = VecDeque::new();
    let mut distmap = HashMap::new();

    q.push_back((start, 0));

    while let Some((point, cost)) = q.pop_front() {
        if distmap.contains_key(&point) {
            continue;
        }
        distmap.insert(point, cost);

        if point == end {
            return distmap;
        }
        for neighbour in neighbours(&point, &grid) {
            if grid[&neighbour] != Tile::Wall {
                q.push_back((neighbour, cost + 1));
            }
        }
    }
    panic!("No distmap found for {:?}", start);
}

fn count(distmap: &HashMap<Point, u32>, max_skip: u32) -> usize {
    distmap
        .iter()
        .tuple_combinations()
        .filter(|((p1, c1), (p2, &c2))| {
            let skip_size = p1.manhattan_distance(p2);
            if skip_size <= max_skip {
                let saved = c1.abs_diff(c2) - skip_size;
                if saved >= 100  {
                    return true;
                }
            }
            false
        })
        .count()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (start, end, grid) = parse(input);
    let dist_map = build_distmap(grid, start, end);
    Some(count(&dist_map, 2) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (start, end, grid) = parse(input);
    let dist_map = build_distmap(grid, start, end);
    Some(count(&dist_map, 20) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1411));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1010263));
    }
}
