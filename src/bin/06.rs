use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(6);

fn parse(input: &str) -> ((isize, isize), Vec<Vec<char>>) {
    let mut grid = Vec::new();
    let mut start = (0, 0);
    input.lines().enumerate().for_each(|(i, line)| {
        let mut row = Vec::new();
        line.chars().enumerate().for_each(|(j, mut ch)| {
            if ch == '^' {
                start = (i as isize, j as isize);
                ch = 'c'
            }
            row.push(ch);
        });
        grid.push(row);
    });

    (start, grid)
}

fn get_path(
    grid: &Vec<Vec<char>>,
    start_pos: (isize, isize),
    start_dir: char,
) -> HashSet<(isize, isize)> {
    let dir_map = HashMap::from([('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1))]);
    let mut dir = start_dir;
    let mut pos = start_pos;
    let mut seen = HashSet::new();
    while (pos.0 >= 0 && pos.0 < grid.len() as isize)
        && (pos.1 >= 0 && pos.1 < grid[0].len() as isize)
    {
        seen.insert(pos);
        let offset = dir_map[&dir];
        let new_pos = (pos.0 + offset.0, pos.1 + offset.1);
        if let Some('#') = grid
            .get(new_pos.0 as usize)
            .and_then(|row| row.get(new_pos.1 as usize))
        {
            dir = match dir {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => panic!("Unexpected direction"),
            }
        } else {
            pos = new_pos;
        }
    }
    seen
}

fn loops(grid: &Vec<Vec<char>>, start_pos: (isize, isize), start_dir: char) -> bool {
    let dir_map = HashMap::from([('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1))]);
    let mut dir = start_dir;
    let mut pos = start_pos;
    let mut seen = HashSet::new();
    while (pos.0 >= 0 && pos.0 < grid.len() as isize)
        && (pos.1 >= 0 && pos.1 < grid[0].len() as isize)
    {
        if (seen.contains(&(pos, dir))) {
            return true;
        }
        seen.insert((pos, dir));
        let offset = dir_map[&dir];
        let new_pos = (pos.0 + offset.0, pos.1 + offset.1);
        if let Some('#') = grid
            .get(new_pos.0 as usize)
            .and_then(|row| row.get(new_pos.1 as usize))
        {
            dir = match dir {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => panic!("Unexpected direction"),
            }
        } else {
            pos = new_pos;
        }
    }
    false
}
pub fn part_one(input: &str) -> Option<u32> {
    let (start, grid) = parse(input);
    Some(get_path(&grid, start, '^').len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (start, mut grid) = parse(input);
    let path = get_path(&grid, start, '^');
    let mut sum = 0;
    for path_pos in path {
        if path_pos == start {
            continue;
        }
        grid[path_pos.0 as usize][path_pos.1 as usize] = '#';
        // detect  loop
        if loops(&grid, start, '^') {
            sum += 1;
        }
        grid[path_pos.0 as usize][path_pos.1 as usize] = '.';
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
