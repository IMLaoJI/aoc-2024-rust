pub mod template;

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
        .flat_map(|r| (0..grid[0].len()).map(move |c| (r , c )))
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
