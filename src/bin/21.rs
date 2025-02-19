use advent_of_code::Point;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

advent_of_code::solution!(21);

fn make_numpad() -> HashMap<Point, char> {
    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+
    HashMap::from([
        (Point::new(0, 0), '7'),
        (Point::new(0, 1), '8'),
        (Point::new(0, 2), '9'),
        (Point::new(1, 0), '4'),
        (Point::new(1, 1), '5'),
        (Point::new(1, 2), '6'),
        (Point::new(2, 0), '1'),
        (Point::new(2, 1), '2'),
        (Point::new(2, 2), '3'),
        (Point::new(3, 1), '0'),
        (Point::new(3, 2), 'A'),
    ])
}

fn make_dirpad() -> HashMap<Point, char> {
    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+
    HashMap::from([
        (Point::new(0, 1), '^'),
        (Point::new(0, 2), 'A'),
        (Point::new(1, 0), '<'),
        (Point::new(1, 1), 'v'),
        (Point::new(1, 2), '>'),
    ])
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Node {
    cost: usize,
    pos: Point,
    instr: char,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let codes = parse(input);
    let mut sum = 0;
    for code in codes {
        let num: usize = code.strip_suffix('A').unwrap().parse().unwrap();
        let len = solve(code.chars().collect(), 2);
        sum += num * len
    }
    Some(sum as u32)
}

fn calc_cost(
    goal: char,
    prev_instr: char,
    depth: u8,
    cache: &mut HashMap<(char, char, u8), usize>,
) -> usize {
    if let Some(&cost) = cache.get(&(goal, prev_instr, depth)) {
        return cost;
    }
    if depth == 0 {
        return 1;
    }

    let point_to_key = make_dirpad();
    let key_to_point: HashMap<_, _> = point_to_key.iter().map(|(pos, c)| (c, pos)).collect();
    let mut pq = BinaryHeap::new();

    pq.push((
        Node {
            cost: 0,
            pos: *key_to_point[&prev_instr],
            instr: 'A',
        },
        None,
    ));

    while let Some((node, output)) = pq.pop() {
        if output.is_some_and(|key| key == goal) {
            cache.insert((goal, prev_instr, depth), node.cost);
            return node.cost;
        }

        for new_instr in "^A<v>".chars() {
            let (new_pos, new_output) = node.pos.execute(new_instr, &point_to_key);
            if !point_to_key.contains_key(&new_pos) {
                continue;
            }
            if new_output.is_some_and(|instr| instr != goal) {
                continue;
            }
            let new_cost = node.cost + calc_cost(new_instr, node.instr, depth - 1, cache);
            pq.push((
                Node {
                    cost: new_cost,
                    pos: new_pos,
                    instr: new_instr,
                },
                new_output,
            ));
        }
    }

    panic!("at the disco");
}

fn solve(code: Vec<char>, depth: u8) -> usize {
    let numpad = make_numpad();
    let mut pq = BinaryHeap::new();
    let mut costmap = HashMap::new();
    let mut cache = HashMap::new();

    pq.push((
        Node {
            cost: 0,
            pos: Point::new(3, 2),
            instr: 'A',
        },
        0,
    ));
    while let Some((node, len)) = pq.pop() {
        if len == code.len() {
            return node.cost;
        }

        if costmap
            .insert((node.pos, node.instr, len), node.cost)
            .is_some()
        {
            continue;
        }

        for new_instr in "^A<v>".chars() {
            let (new_pos, output) = node.pos.execute(new_instr, &numpad);
            if !numpad.contains_key(&new_pos) {
                continue;
            }
            let mut new_len = len;
            // println!("{} {} {:?} {:?} {} {:?} {}", new_instr, node.instr, output, new_pos, code[new_len], node, new_len);

            if let Some(instr) = output {
                if instr != code[new_len] {
                    continue;
                }
                new_len += 1;
            }
            let new_cost =
                node.cost + calc_cost(new_instr, node.instr, depth, &mut cache);
            pq.push((
                Node {
                    cost: new_cost,
                    pos: new_pos,
                    instr: new_instr,
                },
                new_len,
            ));
        }
    }

    panic!("at the disco");
}

pub fn part_two(input: &str) -> Option<u32> {
    let codes = parse(input);
    let mut sum = 0;
    for code in codes {
        let num: usize = code.strip_suffix('A').unwrap().parse().unwrap();
        let len = solve(code.chars().collect(), 20);
        sum += num * len
    }
    Some(sum as u32)
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
