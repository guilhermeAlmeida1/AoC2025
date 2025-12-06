use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let split_input = input.lines().position(|l| l.to_owned() == "").unwrap();

    let fresh_list = input
        .lines()
        .take(split_input)
        .map(|s| {
            let mut parts = s.split('-');
            let start = parts.next().unwrap().parse::<usize>().unwrap();
            let end = parts.next().unwrap().parse::<usize>().unwrap();
            start..=end
        })
        .collect::<Vec<_>>();

    let ingredient_list = input
        .lines()
        .skip(split_input + 1)
        .map(|s| s.parse::<usize>().unwrap());
    let result: usize = ingredient_list
        .map(|ingredient| fresh_list.iter().any(|range| range.contains(&ingredient)) as usize)
        .sum();
    println!("result: {:?}", result);
}
