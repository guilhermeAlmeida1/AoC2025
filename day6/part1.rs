use std::fs;
use std::ops::{Add, Mul};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let nums = input
        .lines()
        .rev()
        .skip(1)
        .map(|l| {
            l.split_whitespace()
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let ops = input.lines().rev().next().unwrap().split_whitespace().map(
        |op| -> fn(usize, usize) -> usize {
            match op {
                "+" => usize::add,
                "*" => usize::mul,
                _ => unreachable!(),
            }
        },
    );

    let result: usize = ops
        .enumerate()
        .map(|(idx, op)| nums.iter().map(|line| line[idx]).reduce(op).unwrap())
        .sum();

    println!("result: {:?}", result);
}
