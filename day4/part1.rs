use std::fs;

fn main() {
    // grid has padding (extra lines and columns at the edges)
    let mut grid: Vec<Vec<usize>> = vec![vec![]];
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.chars().fold(vec![0usize], |mut acc, v| {
                if v == '@' {
                    acc.push(1);
                } else {
                    acc.push(0)
                }
                acc
            })
        })
        .for_each(|mut v| {
            v.push(0);
            grid.push(v);
        });
    let mut borders: Vec<usize> = Vec::new();
    borders.resize(grid[1].len(), 0);
    grid.push(borders.clone());
    grid[0] = borders;

    let mut result = 0;
    for i in 1..(grid.len() - 1) {
        for j in 1..(grid[i].len() - 1) {
            if grid[i][j] == 1 {
                let neighbours: usize = [
                    grid[i - 1][j - 1],
                    grid[i - 1][j],
                    grid[i - 1][j + 1],
                    grid[i][j - 1],
                    grid[i][j + 1],
                    grid[i + 1][j - 1],
                    grid[i + 1][j],
                    grid[i + 1][j + 1],
                ]
                .iter()
                .sum();
                if neighbours < 4 {
                    result += 1;
                }
            }
        }
    }

    println!("out: {}", result);
}
