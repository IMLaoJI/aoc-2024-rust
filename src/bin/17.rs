use itertools::Itertools;

advent_of_code::solution!(17);

struct Computer {
    ip: usize,
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u64>,
}

impl Computer {
    fn compo(&self, operand: u64) -> u64 {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("invalid"),
        }
    }

    fn run(&mut self) -> Option<u64> {
        while self.ip < self.program.len() {
            let opcode = self.program[self.ip];
            let operand = self.program[self.ip + 1];
            self.ip += 2;
            match opcode {
                0 => self.a >>= self.compo(operand),
                1 => self.b ^= operand,
                2 => self.b = self.compo(operand) % 8,
                3 => {
                    if self.a != 0 {
                        self.ip = operand as usize;
                        continue;
                    }
                }
                4 => self.b ^= self.c,
                5 => return Some(self.compo(operand) % 8),
                6 => self.b = self.a >> self.compo(operand),
                7 => self.c = self.a >> self.compo(operand),
                _ => panic!("invalid opcode"),
            }
        }
        None
    }
}

fn parse(input: &str) -> Vec<u64> {
    input
        .split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<String> {
    let nums = parse(input);
    let mut computer = Computer {
        ip: 0,
        a: nums[0],
        b: nums[1],
        c: nums[2],
        program: nums[3..].to_vec(),
    };
    let mut res = Vec::new();
    while let Some(n) = computer.run() {
        res.push(n);
    }
    Some(res.iter().map(|c| c.to_string()).join(","))
}

pub fn part_two(input: &str) -> Option<u32> {
    let nums = parse(input);
    let program = &nums[3..];
    let mut valid = vec![0];
    for &wanted in program.iter().rev() {
        let mut curr_valid = Vec::new();
        for valid_next_a in valid {
            for n in 0..8 {
                let a = (valid_next_a << 3) | n;
                println!("{} {}", a, wanted);
                let mut computer = Computer {
                    ip: 0,
                    a,
                    b: 0,
                    c: 0,
                    program: program.to_vec(),
                };
                if let Some(result) = computer.run() {
                    if result == wanted {
                        curr_valid.push(a);
                    }
                }
            }
        }
        valid = curr_valid;
    }

    Some(*valid.iter().min().unwrap() as u32)
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
