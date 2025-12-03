use std::fs;

fn get_joltage(line: &str) -> u64 {
    let line: Vec<u64> = line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();
    let mut start_idx = 0usize;
    let mut result = 0;

    for i in (0..12).rev() {
        let max = line[start_idx..(line.len() - i)]
            .iter()
            .enumerate()
            .reduce(|(lhs_i, lhs_v), (rhs_i, rhs_v)| {
                if lhs_v >= rhs_v {
                    return (lhs_i, lhs_v);
                }
                (rhs_i, rhs_v)
            })
            .unwrap();
        start_idx += max.0 + 1;
        result += max.1 * 10_u64.pow(i as u32);
    }
    result
}

fn main() {
    let mut result = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        result += get_joltage(line);
    }
    println!("out: {}", result);
}
