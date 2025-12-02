use std::fs;

fn main() {
    let mut state = 50;
    let mut zeros_part1 = 0;
    let mut zeros_part2 = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let mut next_state = match line.chars().next().unwrap() {
            'R' => state + line[1..].parse::<i32>().unwrap(),
            'L' => state - line[1..].parse::<i32>().unwrap(),
            _ => unreachable!(),
        };
        loop {
            match next_state {
                0 | 100 => {
                    state = 0;
                    zeros_part1 += 1;
                    zeros_part2 += 1;
                    break;
                }
                1..=99 => {
                    state = next_state;
                    break;
                }
                101.. => {
                    next_state -= 100;
                    zeros_part2 += 1;
                }
                -99..=-1 => {
                    next_state += 100;
                    if state != 0 {
                        zeros_part2 += 1;
                    }
                }
                ..=-100 => {
                    next_state += 100;
                    zeros_part2 += 1;
                }
            }
        }
        println!("read: {} | state: {} {}", line, state, zeros_part2);
    }

    assert!(zeros_part1 == 1029);
    println!("out: {} ", zeros_part2);
}
