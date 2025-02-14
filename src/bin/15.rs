use itertools::Itertools;
use std::collections::vec_deque::VecDeque;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(15);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn add(&self, point: &Point) -> Self {
        Self::new(self.x + point.x, self.y + point.y)
    }
}

#[derive(Eq, Ord, Clone, Copy, Hash, PartialEq, PartialOrd, Debug)]
enum Tile {
    Empty,
    Wall,
    Robot,
    Box,
    LeftBox,
    RightBox,
}

fn draw(grid: &HashMap<Point, Tile>, grid_str: &str) {
    let rows = grid_str.lines().count() as i32;
    let cols = grid_str.lines().next().unwrap().len() as i32;
    for i in 0..rows {
        for j in 0..cols {
            print!("{:?}", map_tile(grid[&Point::new(i, j)]));
        }
        println!();
    }
}

fn map_tile(ch: Tile) -> char {
    match ch {
        Tile::Wall => '#',
        Tile::Empty => '.',
        Tile::Box => 'O',
        Tile::Robot => '@',
        _ => panic!("invalid"),
    }
}

fn parse_grid(map: &str) -> HashMap<Point, Tile> {
    let mut grid = HashMap::new();
    for (row, line) in map.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            let point = Point::new(row as i32, col as i32);
            let tile = match char {
                '#' => Tile::Wall,
                '.' => Tile::Empty,
                'O' => Tile::Box,
                '@' => Tile::Robot,
                '[' => Tile::LeftBox,
                ']' => Tile::RightBox,
                _ => panic!("invalid"),
            };
            grid.insert(point, tile);
        }
    }
    grid
}

fn parse_instructions(instructions_str: &str) -> Vec<Point> {
    let mut instructions = Vec::new();
    for char in instructions_str.chars() {
        let point = match char {
            '^' => Point::new(-1, 0),
            '>' => Point::new(0, 1),
            'v' => Point::new(1, 0),
            '<' => Point::new(0, -1),
            _ => continue,
        };
        instructions.push(point);
    }
    instructions
}

fn do_instructions(
    mut grid: HashMap<Point, Tile>,
    instructions: Vec<Point>,
    mut draw_fn: impl FnMut(&HashMap<Point, Tile>),
) -> HashMap<Point, Tile> {
    let mut robot = grid
        .iter()
        .find_map(|(point, tile)| (*tile == Tile::Robot).then_some(*point))
        .unwrap();
    'outer: for inst in instructions {
        let mut q = VecDeque::new();
        let mut seen = HashSet::new();
        q.push_back(robot);

        while let Some(point) = q.pop_front() {
            if !seen.insert(point) {
                continue;
            }
            let new = point.add(&inst);
            // println!("{:?} {:?} {:?} {:?}", new, point, grid[&new], inst);
            let new_tile = grid[&new];
            match new_tile {
                Tile::Empty => continue,
                Tile::Wall => continue 'outer,
                Tile::Robot => continue,
                Tile::Box => q.push_back(new),
                Tile::LeftBox => {
                    q.push_back(new);
                    let right = Point::new(new.x, new.y + 1);
                    q.push_back(right)
                }
                Tile::RightBox => {
                    q.push_back(new);
                    let left = Point::new(new.x, new.y - 1);
                    q.push_back(left)
                }
            }
        }

        while !seen.is_empty() {
            for point in seen.iter().copied().collect_vec() {
                let new = point.add(&inst);
                // println!("{:?} {:?} {:?} {:?} {:?}", seen.contains(&new), point, new, inst, seen);
                if !seen.contains(&new) {
                    *grid.entry(new).or_insert(Tile::Empty) = grid[&point];
                    *grid.entry(point).or_insert(Tile::Empty) = Tile::Empty;
                    seen.remove(&point);
                } else {
                }
            }
        }
        draw_fn(&grid);
        robot = robot.add(&inst);
    }
    grid
}
fn embigger(input: &str) -> String {
    let mut map = String::new();
    for line in input.lines() {
        for char in line.chars() {
            let replacement = match char {
                '#' => "##",
                '.' => "..",
                'O' => "[]",
                '@' => "@.",
                _ => panic!("invalid"),
            };
            map.push_str(replacement);
        }
        map.push('\n');
    }
    map
}
fn gps(grid: &HashMap<Point, Tile>, item: Tile) -> i32 {
    grid.iter()
        .filter_map(|(point, tile)| (*tile == item).then_some(100 * point.x + point.y))
        .sum()
}

pub fn part_one(input: &str) -> Option<i32> {
    let (grid_str, instructions_str) = input.split_once("\r\n\r\n").unwrap();

    let mut grid = parse_grid(grid_str);
    let instructions = parse_instructions(instructions_str);
    grid = do_instructions(grid, instructions, |grid_state| {
        // draw(&grid_state, grid_str);
    });
    Some(gps(&grid, Tile::Box))
}

pub fn part_two(input: &str) -> Option<i32> {
    let (grid_str, instructions_str) = input.split_once("\r\n\r\n").unwrap();
    let grid_str = embigger(grid_str);
    let mut grid = parse_grid(&grid_str);
    let instructions = parse_instructions(instructions_str);
    grid = do_instructions(grid, instructions, |grid_state| {
        // draw(&grid_state, grid_str);
    });
    Some(gps(&grid, Tile::LeftBox))
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
