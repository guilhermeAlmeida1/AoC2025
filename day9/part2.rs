use std::fs;

fn area(lhs: &Vec<i64>, rhs: &Vec<i64>) -> i64 {
    let l = (lhs[0] - rhs[0] + 1).abs();
    let w = (lhs[1] - rhs[1] + 1).abs();
    l * w
}

fn is_inside(x: i64, y: i64, input: &Vec<Vec<i64>>) -> bool {
    let mut n_edges = 0;
    for i in 0..input.len() {
        let next = (i + 1) % input.len();
        if input[i][0] == input[next][0] {
            let min_y = input[i][1].min(input[next][1]);
            let max_y = input[i][1].max(input[next][1]);
            if input[i][0] == x && (min_y..=max_y).contains(&y) {
                // println!("{} {} inside edge: {:?} {:?}", x, y, input[i], input[next]);
                return true;
            }
            if input[i][0] > x && min_y <= y && max_y > y
            {
                n_edges += 1;
            }
        }
        else if input[i][1] == input[next][1] && input[i][1] == y {
            let min_x = input[i][0].min(input[next][0]);
            let max_x = input[i][0].max(input[next][0]);
            if (min_x..=max_x).contains(&x) {
                // println!("{} {} inside edge: {:?} {:?}", x, y, input[i], input[next]);
                return true;
            }
        }
    }
    // println!("{} {} edges count: {} {}", x, y, n_edges % 2 == 1, n_edges);
    n_edges % 2 == 1
}

fn rectangle_is_inside(x1: i64, y1: i64, x2: i64, y2: i64, input: &Vec<Vec<i64>>) -> bool {
    // println!("rectangle: {:?} {:?} {:?} {:?}", itr, other, itr[0]..=other[0], (itr[0]..=other[0])
    let x_min = x1.min(x2);
    let x_max = x1.max(x2);
    let y_min = y1.min(y2);
    let y_max = y1.max(y2);
    (x_min..=x_max).all(|x| { (y_min..=y_max).all(|y| { is_inside(x, y, input) }) })
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split(",")
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // let result = input
    //     .iter()
    //     .enumerate()
    //     .map(|(itr_idx, itr)| {
    //         if let Some(max) = input
    //             .iter()
    //             .skip(itr_idx + 1)
    //             .filter(|other| {
    //                 rectangle_is_inside(itr[0], itr[1], other[0], other[1], &input)
    //             })
    //             .map(|other| area(itr, other))
    //             .max()
    //         {
    //             return max;
    //         }
    //         0
    //     })
    //     .max()
    //     .unwrap();

    let result = input
        .iter()
        .enumerate()
        .map(|(itr_idx, itr)| {
            let mut sorted_by_area = input
                .iter()
                .skip(itr_idx + 1)
                .map(|other| (itr, other, area(itr, other))).collect::<Vec<_>>();
            sorted_by_area.sort_by_key(|(_,_, area)| *area);

            if let Some((_, _, max)) = sorted_by_area.iter().rev().find(|(itr, other, _)| {
                    rectangle_is_inside(itr[0], itr[1], other[0], other[1], &input)
                }) {
                    return *max;
                }
            0
        })
        .max()
        .unwrap();
    println!("result: {:?}", result);

    // let mut print = String::new();
    // let mut test_result = 0;
    // for i in 0i64..9 {
    //     for j in 0i64..14 {
    //         // println!("{:?} {} is inside {}", j, i, is_inside(j, i, &input));
    //         if is_inside(j, i, &input) {
    //             // println!("{} {} is inside", j, i);
    //             print.push('#');
    //             test_result += 1;
    //         }
    //         else {
    //             print.push('.');
    //         }
    //     }
    //     print.push('\n');
    // }
    // println!("test_result: {}", test_result);
    // println!("{}", print);
}
