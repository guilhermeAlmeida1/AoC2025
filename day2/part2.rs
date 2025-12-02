use std::fs;

fn is_valid(num: usize) -> bool {
    let num = num.to_string();

    for i in (1..=(num.len() / 2)).rev() {
        if num.len() % i == 0 {
            let mut is_invalid_i = true;
            for j in 1..(num.len() / i) {
                if num[0..i] != num[(i * j)..(i * (j + 1))] {
                    is_invalid_i = false;
                    break;
                }
            }
            if is_invalid_i {
                return false;
            }
        }
    }
    true
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap().lines().next().unwrap().to_string();
    let mut result = 0;
    for range in input.split(',') {
        let mut parts = range.split('-');
        let start = parts.next().unwrap().parse::<usize>().unwrap();
        let end = parts.next().unwrap().parse::<usize>().unwrap();
        for i in start..=end {
            if !is_valid(i) {
                result += i;
            }
        }
    }
    println!("result: {}", result);
}
