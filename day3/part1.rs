use std::fs;

fn get_joltage(line: &str) -> u32 {
    let line: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let v0 = line[..(line.len() - 1)]
        .iter()
        .enumerate()
        .reduce(|(lhs_i, lhs_v), (rhs_i, rhs_v)| {
            if lhs_v >= rhs_v {
                return (lhs_i, lhs_v);
            }
            (rhs_i, rhs_v)
        })
        .unwrap();
    let v1 = line[(v0.0 + 1)..].iter().max().unwrap();

    v0.1 * 10 + v1
}

fn main() {
    let mut result = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        result += get_joltage(line);
    }
    println!("out: {}", result);
}
