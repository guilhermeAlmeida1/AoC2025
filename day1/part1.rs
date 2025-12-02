use std::fs;

fn main() {
    let mut state = 50;
    let mut zeros = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        match line.chars().next().unwrap() {
            'R' => state += line[1..].parse::<i32>().unwrap(),
            'L' => state -= line[1..].parse::<i32>().unwrap(),
            _ => break,
        }
        while state < 0 {
            state += 100;
        }
        while state >= 100 {
            state -= 100;
        }
        if state == 0 {
            zeros += 1;
        }
    }
    println!("out: {}", zeros);
}
