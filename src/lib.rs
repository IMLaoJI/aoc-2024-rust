use std::collections::HashMap;

pub mod template;
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Point {
    pub row: i32,
    pub col: i32,
}

impl Point {
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    pub fn add(&self, point: &Point) -> Self {
        Self::new(self.row + point.row, self.col + point.col)
    }

    pub fn dirs() -> [Point; 4] {
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

    pub fn is_in_bound(&self, rows: i32, cols: i32) -> bool {
        (self.row >= 0 && self.row < rows) && (self.col >= 0 && self.col < cols)
    }

    pub fn neighbour(&self, rows: i32, cols: i32) -> Vec<Point> {
        let mut neighbours = Vec::new();
        for dir in Self::dirs() {
            let next = self.add(&dir);
            if Self::is_in_bound(&next, rows, cols) {
                neighbours.push(next);
            }
        }
        neighbours
    }
    pub fn manhattan_distance(&self, point: &Point) -> u32 {
        self.row.abs_diff(point.row) + self.col.abs_diff(point.col)
    }
    pub fn execute(&self, inst: char, pad: &HashMap<Point, char>) -> (Point, Option<char>) {
        match inst {
            '^' => (Self::new(self.row - 1, self.col), None),
            '>' => (Self::new(self.row, self.col + 1), None),
            'v' => (Self::new(self.row + 1, self.col), None),
            '<' => (Self::new(self.row, self.col - 1), None),
            'A' => (Self::new(self.row, self.col), Some(pad[self])),
            _ => panic!("at the disco"),
        }
    }
}

pub const DIRS: [(i32, i32); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

pub fn to_grid(input: &str) -> (Vec<&[u8]>, Vec<(usize, usize)>) {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let grid_points = (0..grid.len())
        // turn into iterator over (row_idx, col_idx)
        .flat_map(|r| (0..grid[0].len()).map(move |c| (r, c)))
        .collect();
    (grid, grid_points)
}

pub fn print_grid(grid: &Vec<&[u8]>) {
    for slice in grid {
        let string = String::from_utf8_lossy(slice); // 转换为字符串
        println!("{}", string); // 打印字符串
    }
}

// Use this file to add helper functions and additional modules.
