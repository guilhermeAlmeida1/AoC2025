use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let split_input = input.iter().position(|l| l == "").unwrap();

    let mut fresh_list = (&input[0..split_input])
        .iter()
        .map(|s| {
            let mut parts = s.split('-');
            let start = parts.next().unwrap().parse::<usize>().unwrap();
            let end = parts.next().unwrap().parse::<usize>().unwrap();
            start..=end
        })
        .collect::<Vec<_>>();
    fresh_list.sort_by(|a, b| a.start().cmp(b.start()));

    let result = fresh_list
        .iter()
        .fold((0usize, 0usize), |(acc_last, acc_sum), x| {
            if *x.start() <= acc_last {
                if *x.end() > acc_last {
                    return (*x.end(), acc_sum + *x.end() - acc_last);
                }
                return (acc_last, acc_sum);
            }
            (*x.end(), acc_sum + *x.end() - *x.start() + 1)
        })
        .1;

    println!("result: {:?}", result);
}
