use std::fs;

fn is_valid(num: usize) -> bool {
    let num = num.to_string();

    num.len() % 2 != 0 || num[0..(num.len() / 2)] != num[(num.len() / 2)..]
}

fn main() {
    let mut result = 0;
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .to_string();
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
