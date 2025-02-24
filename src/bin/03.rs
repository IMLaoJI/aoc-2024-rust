use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let mut sum = 0;
    for x in regex.find_iter(input.trim()) {
        let x = x.as_str();
        let (a, b) = x[4..x.len() - 1].split_once(',').unwrap();
        let prod = a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap();
        sum+=prod;
    }
    Some(sum)
}

fn part_parse(input: &str) -> (u32,u32) {
    let memory = input.as_bytes();
    let mut idx = 0;
    let mut on = true;
    let mut sum_1 = 0;
    let mut sum_2 = 0;

    while idx < memory.len() {
        // go to next instruction
        match &memory[idx..] {
            b if b.starts_with(b"mul(") => {
                idx += 4;
            }
            b if b.starts_with(b"don't()") => {
                on = false;
                idx += 7;
                continue;
            }
            b if b.starts_with(b"do()") => {
                on = true;
                idx += 4;
                continue;
            }
            _ => {
                idx += 1;
                continue;
            }
        }

        // parse num 1
        let mut num_1 = 0;
        let start_idx = idx;
        while memory[idx].is_ascii_digit() {
            num_1 *= 10;
            num_1 += (memory[idx] - b'0') as u32;
            idx += 1;
        }
        // check if number was between 1 and 3 digits
        if idx > start_idx + 3 {
            continue;
        }

        // skip ,
        if memory[idx] != b',' {
            continue;
        }
        idx += 1;

        // parse num 2
        let mut num_2 = 0;
        let start_idx = idx;
        while memory[idx].is_ascii_digit() {
            num_2 *= 10;
            num_2 += (memory[idx] - b'0') as u32;
            idx += 1;
        }
        // check if number was between 1 and 3 digits
        if  idx > start_idx + 3 {
            continue;
        }

        // skip )
        if memory[idx] != b')' {
            continue;
        }
        idx += 1;
        // add product to sum
        let prod = num_1 * num_2;
        sum_1 += prod;
        if on {
            sum_2 += prod
        }
    }
    (sum_1, sum_2)
}

 pub fn part_one_parse(input:&str) -> Option<u32> {
     Some(part_parse(input).0)
 }

pub fn part_two_parse(input:&str) -> Option<u32> {
    Some(part_parse(input).1)
}


pub fn part_two(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let mut on = true;
    let mut sum = 0;
    for regex_match in regex.find_iter(input.trim()){
        match regex_match.as_str(){
            "do()" => on = true,
            "don't()" => on = false,
            x => {
                if on {
                    let (a, b) = x[4..x.len() - 1].split_once(',').unwrap();
                    let prod = a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap();
                    sum+=prod;
                }
            }

        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_parse(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_parse(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
