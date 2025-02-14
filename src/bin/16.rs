use std::cmp::{Ordering, PartialEq};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::rc::Rc;

advent_of_code::solution!(16);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
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

    fn clockwise(&self) -> Point {
        Self::new(self.col, -self.row)
    }

    fn counter_clockwise(&self) -> Point {
        Self::new(-self.col, self.row)
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Node {
    pos: Point,
    dir: Point,
    cost: u32,
    from: Option<Rc<Node>>,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
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
                'E' => Tile::End,
                'S' => Tile::Start,
                _ => panic!("Unexpected character in input: {}", char),
            };
            if tile == Tile::Start {
                start = point;
            }
            if tile == Tile::End {
                end = point;
            }
            grid.insert(point, tile);
        }
    }
    (start, end, grid)
}

fn moves(grid: &HashMap<Point, Tile>, dir: Point, pos: Point) -> Vec<(Point, Point, u32)> {
    let mut moves = Vec::new();
    let point = pos.add(&dir);
    if let Some(&tile) = grid.get(&point) {
        if Tile::Wall != tile {
            moves.push((point, dir, 1));
        }
    }
    moves.push((pos, dir.clockwise(), 1000));
    moves.push((pos, dir.counter_clockwise(), 1000));
    moves
}

fn shortest_path(grid: &HashMap<Point, Tile>, start: Point, end: Point) -> (u32, usize) {
    let mut best_positions = HashSet::new();
    best_positions.insert(start);
    best_positions.insert(end);
    let mut lowest_cost = u32::MAX;
    let mut shortest: HashMap<(Point, Point), u32> = HashMap::new();
    let mut pq = BinaryHeap::new();

    pq.push(Node {
        pos: start,
        dir: Point::new(0, 1),
        cost: 0,
        from: None,
    });
    while let Some(node) = pq.pop() {
        if node.pos == end {
            lowest_cost = lowest_cost.min(node.cost);
            if node.cost > lowest_cost {
                break;
            }
            // reconstruct
            let mut curr = Rc::new(node);
            while curr.pos != start {
                best_positions.insert(curr.pos);
                if let Some(prev) = &curr.from {
                    curr = prev.clone();
                }
            }
            continue;
        }

        if let Some(&lowest) = shortest.get(&(node.pos, node.dir)) {
            if lowest < node.cost {
                continue;
            }
        }
        shortest.insert((node.pos, node.dir), node.cost);

        // moves
        for (new_pos, new_dir, move_cost) in moves(&grid, node.dir, node.pos) {
            let new_cost = node.cost + move_cost;
            if let Some(&lowest) = shortest.get(&(new_pos, new_dir)) {
                if lowest <= new_cost {
                    continue;
                }
            }

            pq.push(Node {
                pos: new_pos,
                dir: new_dir,
                cost: new_cost,
                from: Some(Rc::new(node.clone())),
            })
        }
    }

    (lowest_cost, best_positions.len())
}

pub fn part_one(input: &str) -> Option<u32> {
    let (start, end, grid) = parse(input);
    Some(shortest_path(&grid, start, end).0)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (start, end, grid) = parse(input);
    Some(shortest_path(&grid, start, end).1 as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
