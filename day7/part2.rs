use std::collections::HashMap;
use std::fs;

fn process_line(mut state: HashMap<usize, usize>, line: &str) -> HashMap<usize, usize> {
    let mut iter = line.chars();
    let mut last_pos = 0;
    while let Some(pos) = iter.position(|c| c == '^') {
        let pos = pos + last_pos;
        last_pos = pos + 1;
        if let Some(v) = state.remove(&pos) {
            for i in [pos - 1, pos + 1] {
                if let Some(branch_v) = state.get_mut(&i) {
                    *branch_v += v;
                } else {
                    state.insert(i, v);
                }
            }
        }
    }
    state
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut state = HashMap::new();
    state.insert(input.lines().next().unwrap().chars().position(|c| c == 'S').unwrap(), 1);
    for line in input.lines().skip(1) {
        state = process_line(state, line);
    }
    println!("result: {:?}", state.into_values().sum::<usize>());
}
