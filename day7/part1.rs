use std::collections::HashSet;
use std::fs;

fn process_line(mut state: (HashSet<usize>, usize), line: &str) -> (HashSet<usize>, usize) {
    let mut iter = line.chars();
    let mut last_pos = 0;
    while let Some(pos) = iter.position(|c| c == '^') {
        let pos = pos + last_pos;
        last_pos = pos + 1;
        if state.0.contains(&pos) {
            state.0.remove(&pos);
            state.0.insert(pos + 1);
            state.0.insert(pos - 1);
            state.1 += 1;
        }
    }
    state
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut state = (HashSet::new(), 0);
    state.0.insert(input.lines().next().unwrap().chars().position(|c| c == 'S').unwrap());
    for line in input.lines().skip(1) {
        state = process_line(state, line);
    }
    println!("result: {:?}", state.1);
}
