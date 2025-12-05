use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let split_input = input.iter().position(|l| l == "").unwrap();

    let fresh_list = &input[0..split_input]
        .iter()
        .map(|s| {
            let mut parts = s.split('-');
            let start = parts.next().unwrap().parse::<usize>().unwrap();
            let end = parts.next().unwrap().parse::<usize>().unwrap();
            start..=end
        })
        .collect::<Vec<_>>();

    let ingredient_list = (&input[(split_input + 1)..])
        .iter()
        .map(|s| s.parse::<usize>().unwrap());
    let result: usize = ingredient_list
        .map(|ingredient| fresh_list.iter().any(|range| range.contains(&ingredient)) as usize)
        .sum();
    println!("result: {:?}", result);
}
