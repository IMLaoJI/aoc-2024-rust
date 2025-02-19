use itertools::Itertools;
use std::cmp::PartialEq;
use std::collections::HashMap;

advent_of_code::solution!(24);

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Operator {
    And,
    Or,
    Xor,
}

impl Operator {
    fn execute(&self, a: bool, b: bool) -> bool {
        match self {
            Self::And => a & b,
            Self::Or => a | b,
            Self::Xor => a ^ b,
        }
    }
}

#[derive(Copy, Clone)]
struct Operation<'a> {
    lhs: &'a str,
    op: Operator,
    rhs: &'a str,
}

fn parse(input: &str) -> (HashMap<&str, bool>, HashMap<&str, Operation>) {
    let (top, bottom) = input.split_once("\r\n\r\n").unwrap();
    let mut wires = HashMap::new();
    for line in top.lines() {
        let (left, right) = line.split_once(": ").unwrap();
        wires.insert(left, right == "1");
    }
    let mut operations = HashMap::new();
    for line in bottom.lines() {
        let (left, right) = line.split_once(" -> ").unwrap();
        let (lhs, op, rhs) = left.split_whitespace().collect_tuple().unwrap();
        let op = match op {
            "AND" => Operator::And,
            "OR" => Operator::Or,
            "XOR" => Operator::Xor,
            _ => panic!("invalid operator"),
        };
        operations.insert(right, Operation { lhs, op, rhs });
    }
    (wires, operations)
}

fn calc<'a>(
    wires: &mut HashMap<&'a str, bool>,
    operations: &HashMap<&str, Operation<'a>>,
    wire: &'a str,
) -> bool {
    if let Some(&on) = wires.get(wire) {
        return on;
    }
    let Operation { lhs, op, rhs } = &operations[wire];
    let lhs = calc(wires, operations, lhs);
    let rhs = calc(wires, operations, rhs);
    let res = op.execute(lhs, rhs);
    wires.insert(wire, res);
    res
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut wires, operations) = parse(input);
    Some(
        operations
            .keys()
            .filter(|k| k.starts_with("z"))
            .sorted()
            .rev()
            .map(|k| calc(&mut wires, &operations, k))
            .fold(0, |acc, bit| acc << 1 | bit as u32),
    )
}

/**
关键分析步骤
二进制加法逻辑验证：

最低位（z00）：应为x0 XOR y0。
中间位（zXX）：应为(x_i XOR y_i) XOR carry，其中carry是前一位的进位。
进位计算：carry = (x_i AND y_i) OR ((x_i XOR y_i) AND previous_carry)。
    直接进位 (x_i AND y_i)：当两个加数位均为1时，必然产生进位。
    传递进位 ((x_i XOR y_i) AND C_prev)：若当前位有且仅有一个1（即 x_i XOR y_i = 1），且低位有进位时，当前位需将进位传递到高位。
手动验证电路结构：
 x00 ──┐
      XOR → z00
y00 ──┘

x01 ──┐
      XOR ─┬─┐
y01 ──┘    │ XOR → z01
           │

进位计算：carry
x01 ────────┐
           AND → (direct_carry) ─┐
y01 ────────┘                    │ OR → z01 (当前位进位输出)
x01 ───┐                         │
       XOR → (sum_part) ── AND ← previous_carry (来自z00的进位)
y01 ───┘


递归检查每个Z位的计算是否符合加法逻辑：
验证是否为XOR操作。
验证操作数是否来自正确的中间XOR和进位线路。
自动化错误检测：

遍历所有可能的线路对，交换后检查验证进度是否提升。
重复四次直到找到所有错误对。
**/
pub fn part_two(input: &str) -> Option<String> {
    let (_, mut ops) = parse(input);
    let mut swaps = Vec::new();
    let wires: Vec<&str> = ops.keys().copied().collect();
    for _ in 0..4 {
        let baseline = progress(&ops);
        for (a, b) in wires.iter().tuple_combinations() {
            swap_wires(&mut ops, a, b);
            if progress(&ops) > baseline {
                swaps.push([*a, *b]);
                break;
            }
            swap_wires(&mut ops, a, b);
        }
    }

    Some(swaps.into_iter().flatten().sorted().intersperse(",").collect())
}

fn swap_wires<'a>(map: &mut HashMap<&'a str, Operation<'a>>, a: &'a str, b: &'a str) {
    let temp = map[a];
    map.insert(a, map[b]);
    map.insert(b, temp);
}


fn progress(ops: &HashMap<&str, Operation>) -> i32 {
    (0..)
        .find(|&idx| !is_ok_z(ops, &make_wire('z', idx), idx))
        .unwrap()
}
fn is_ok_z(ops: &HashMap<&str, Operation>, wire: &str, num: i32) -> bool {
    if let Some(Operation { lhs, op, rhs }) = ops.get(wire) {
        if *op != Operator::Xor {
            return false;
        }
        if num == 0 {
            let mut operands = [*lhs, *rhs];
            operands.sort();
            return operands == ["x00", "y00"];
        }
        return (is_ok_xor(ops, lhs, num) && is_ok_carry_bit(ops, rhs, num))
            || (is_ok_xor(ops, rhs, num) && is_ok_carry_bit(ops, lhs, num));
    }
    false
}

fn is_ok_carry_bit(ops: &HashMap<&str, Operation>, wire: &&str, num: i32) -> bool {
    if let Some(Operation { lhs, op, rhs }) = ops.get(wire) {
        if num == 1 {
            if *op != Operator::And {
                return false;
            }
            let mut operands = [*lhs, *rhs];
            operands.sort();
            return operands == ["x00", "y00"];
        }
        if *op != Operator::Or {
            return false;
        }
        return (is_ok_direct_carry(ops, lhs, num - 1) && is_ok_recarry(ops, rhs, num - 1))
            || (is_ok_direct_carry(ops, rhs, num - 1) && is_ok_recarry(ops, lhs, num - 1));
    }
    false
}
fn is_ok_direct_carry(ops: &HashMap<&str, Operation>, wire: &str, num: i32) -> bool {
    if let Some(Operation { lhs, op, rhs }) = ops.get(wire) {
        if *op != Operator::And {
            return false;
        }
        let mut operands = [*lhs, *rhs];
        operands.sort();
        return operands == [make_wire('x', num), make_wire('y', num)];
    }
    false
}

fn is_ok_recarry(ops: &HashMap<&str, Operation>, wire: &str, num: i32) -> bool {
    if let Some(Operation { lhs, op, rhs }) = ops.get(wire) {
        if *op != Operator::And {
            return false;
        }
        return (is_ok_xor(ops, lhs, num) && is_ok_carry_bit(ops, rhs, num))
            || (is_ok_xor(ops, rhs, num) && is_ok_carry_bit(ops, lhs, num));
    }
    false
}

fn is_ok_xor(ops: &HashMap<&str, Operation>, wire: &&str, num: i32) -> bool {
    if let Some(Operation { lhs, op, rhs }) = ops.get(wire) {
        if *op != Operator::Xor {
            return false;
        }
        let mut operands = [*lhs, *rhs];
        operands.sort();
        return operands == [make_wire('x', num), make_wire('y', num)];
    }
    false
}

fn make_wire(c: char, n: i32) -> String {
    format!("{}{:02}", c, n)
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
