use itertools::Itertools;

advent_of_code::solution!(13);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

fn parse(input: &str) -> Vec<[Point; 3]> {
    let mut machines = Vec::new();
    for block in input.split("\r\n\r\n") {
        let (adx, ady, bdx, bdy, x, y) = block
            .split(|c: char| !c.is_ascii_digit())
            .filter(|c| !c.is_empty())
            .map(|c| c.parse().unwrap())
            .collect_tuple()
            .unwrap();
        let a = Point::new(adx, ady);
        let b = Point::new(bdx, bdy);
        let prize = Point::new(x, y);
        machines.push([a, b, prize]);
    }
    machines
}

fn solve(a: Point, b: Point, prize: Point) -> Option<i64> {
    assert_ne!(a.y as f64 / a.x as f64, b.y as f64 / b.x as f64);
    let na = (prize.x * b.y - prize.y * b.x) / ((a.x * b.y) - (a.y * b.x));
    let nb = (prize.x - na * a.x) / b.x;
    let solution = Point::new(na * a.x + nb * b.x, na * a.y + nb * b.y);
    (solution == prize).then_some(3 * na + nb)
}

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        parse(input)
            .into_iter()
            .filter_map(|[a, b, prize]| solve(a, b, prize))
            .sum(),
    )
}
const TEN_TRILLY: i64 = 10_000_000_000_000;
pub fn part_two(input: &str) -> Option<i64> {
    Some(
        parse(input)
            .into_iter()
            .filter_map(|[a, b, mut prize]| {
                prize.x += TEN_TRILLY;
                prize.y += TEN_TRILLY;
                solve(a, b, prize)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
