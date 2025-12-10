use std::fs;

fn area(lhs: &Vec<i64>, rhs: &Vec<i64>) -> i64 {
    let l = (lhs[0] - rhs[0] + 1).abs();
    let w = (lhs[1] - rhs[1] + 1).abs();
    l * w
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split(",")
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let result = input.iter().enumerate().map(|(itr_idx, itr)| {
        if let Some(max) = input.iter().skip(itr_idx + 1).map(|other| { area(itr, other) }).max() {
            return max;
        }
        0
    }).max().unwrap();
    println!("result: {:?}", result);
}
