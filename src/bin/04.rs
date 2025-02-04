use advent_of_code::{to_grid, DIRS};
use std::str::from_utf8;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, grid_points) = to_grid(input);
    Some(
        grid_points
            .into_iter()
            .filter(|(r, c)| grid[*r][*c] == b'X')
            .flat_map(|(r, c)| {
                DIRS.iter().map(move |(dr, dc)| {
                    [
                        (r as i32 + dr, c as i32 + dc),
                        (r as i32 + 2 * dr, c as i32 + 2 * dc),
                        (r as i32 + 3 * dr, c as i32 + 3 * dc),
                    ]
                })
            })
            .filter(|&[_, _, (r, c)]| {
                (r >= 0 && r < grid.len() as i32) && (c >= 0 && c < grid[0].len() as i32)
            })
            .map(|[(r1, c1), (r2, c2), (c3, r3)]| {
                [
                    (r1 as usize, c1 as usize),
                    (r2 as usize, c2 as usize),
                    (r3 as usize, c3 as usize),
                ]
            })
            .map(|[(r1, c1), (r2, c2), (c3, r3)]| [grid[r1][c1], grid[r2][c2], grid[r3][c3]])
            // .inspect(|word| println!("{:?}", std::str::from_utf8(word).unwrap()))
            .filter(|word| word == b"MAS")
            .count() as u32,
    )
}

pub fn part_one_common(input: &str) -> Option<u32> {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let mut count = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] != b'X' {
                continue;
            }
            let r = r as i32;
            let c = c as i32;
            for (dr, dc) in DIRS {
                let final_r = r + dr * 3;
                let final_c = c + dc * 3;
                if final_r < 0
                    || final_r >= grid.len() as i32
                    || final_c < 0
                    || final_c >= grid[0].len() as i32
                {
                    continue;
                }

                if &[
                    grid[(r + dr) as usize][(c + dc) as usize],
                    grid[(r + 2 * dr) as usize][(c + 2 * dc) as usize],
                    grid[(r + 3 * dr) as usize][(c + 3 * dc) as usize],
                ] == b"MAS"
                {
                    count += 1
                }
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    Some(
        (1..grid.len() - 1)
            .flat_map(|r| (1..grid[0].len() - 1).map(move |c| (r, c)))
            .filter(|(r, c)| grid[*r][*c] == b'A')
            .map(|(r, c)| {
                [
                    grid[r - 1][c - 1],
                    grid[r - 1][c + 1],
                    grid[r + 1][c + 1],
                    grid[r + 1][c - 1],
                ]
            })
            .filter(|word| word == b"MSSM" || word == b"MMSS" || word == b"SMMS" || word == b"SSMM")
            .count() as u32,
    )
}

pub fn part_two_common(input: &str) -> Option<u32> {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let mut count = 0;
    for r in 1..grid.len() - 1 {
        for c in 1..grid[0].len() - 1 {
            if grid[r][c] != b'A' {
                continue;
            }
            let circle = [
                grid[r - 1][c - 1],
                grid[r - 1][c + 1],
                grid[r + 1][c + 1],
                grid[r + 1][c - 1],
            ];
            if &circle == b"MSSM" || &circle == b"MMSS" || &circle == b"SMMS" || &circle == b"SSMM"
            {
                count += 1;
            }
        }
    }
    Some(count)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_common(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_common(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
