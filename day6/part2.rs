use std::fs;
use std::ops::{Add, Mul};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut nums = input
        .lines()
        .rev()
        .skip(1)
        .map(|l| { l.chars().collect::<Vec<_>>() }).collect::<Vec<_>>();

    let max_size = nums.iter().map(|row| row.len()).max().unwrap();
    nums.iter_mut().for_each(|row| row.resize(max_size, ' '));

    let ops = input.lines().rev().next().unwrap().split_whitespace().collect::<Vec<_>>();

    let mut result = 0;
    let mut char_idx = 0;
    for op_idx in 0..ops.len() {
        let (op, mut op_result): (fn(usize, usize) -> usize, _) = match ops[op_idx] {
                "+" => (usize::add, 0),
                "*" => (usize::mul, 1),
                _ => unreachable!(),
        };

        for i in char_idx..max_size {
            let mut used = false;
            let mut current_num = 0;
            for row_idx in (0..nums.len()).rev() {
                if let Some(n) = nums[row_idx][i].to_digit(10) {
                    current_num = current_num * 10 + n;
                    used = true;
                }
            }
            if !used {
                char_idx = i+1;
                break;
            }
            op_result = op(op_result, current_num as usize);
        }
        result += op_result;
    }
    println!("result: {:?}", result);
}
