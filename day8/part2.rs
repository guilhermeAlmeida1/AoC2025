use std::collections::HashSet;
use std::fs;

fn distance(lhs: &Vec<i64>, rhs: &Vec<i64>) -> f64 {
    (lhs.iter()
        .zip(rhs.iter())
        .map(|(&v1, &v2)| (v1 - v2) * (v1 - v2))
        .sum::<i64>() as f64)
        .sqrt()
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

    let mut distances = input
        .iter()
        .enumerate()
        .map(|(idx_it, it)| {
            input
                .iter()
                .skip(idx_it + 1)
                .enumerate()
                .map(|(idx_other, other)| (idx_it, idx_it + idx_other + 1, distance(it, other)))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    // println!("distances: {:?}", distances);
    distances.sort_by(|&a, &b| (a.2).partial_cmp(&(b.2)).unwrap());
    // println!("cmp: {:?} {:?}", before, before.len());

    let mut connected_sets = Vec::<HashSet<usize>>::new();
    for i in 0..distances.len() {
        let conn = (distances[i].0, distances[i].1);
        let set0 = connected_sets
            .iter()
            .enumerate()
            .find(|(_, itr)| itr.contains(&conn.0));
        let set1 = connected_sets
            .iter()
            .enumerate()
            .find(|(_, itr)| itr.contains(&conn.1));
        let set0 = match set0 {
            Some(set) => Some(set.0),
            None => None,
        };
        let set1 = match set1 {
            Some(set) => Some(set.0),
            None => None,
        };
        match set0 {
            Some(set0) => match set1 {
                Some(set1) => {
                    if set0 < set1 {
                        connected_sets.remove(set1).iter().for_each(|v| {
                            connected_sets[set0].insert(*v);
                        });
                    } else if set0 > set1 {
                        connected_sets.remove(set0).iter().for_each(|v| {
                            connected_sets[set1].insert(*v);
                        });
                    }
                }
                None => {
                    connected_sets[set0].insert(conn.1);
                }
            },
            None => match set1 {
                Some(set1) => {
                    connected_sets[set1].insert(conn.0);
                }
                None => {
                    let mut new_set = HashSet::new();
                    new_set.insert(conn.0);
                    new_set.insert(conn.1);
                    connected_sets.push(new_set);
                }
            },
        }
        // println!("sets: {:?}", connected_sets);

        if connected_sets.len() == 1 && connected_sets[0].len() == input.len() {
            println!("result: {}", input[conn.0][0] * input[conn.1][0]);
            break;
        }
    }
}
