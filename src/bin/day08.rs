use std::cmp::max;

fn get_chart() -> Vec<Vec<u8>> {
    aoc::io::get_input(8)
        .lines()
        .map(|line| line.chars().map(|tree| tree.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn get_dimensions<T>(chart: &Vec<Vec<T>>) -> (usize, usize) {
    let width = chart.iter().map(|x| x.len()).max().unwrap();
    let height = chart.len();
    (width, height)
}

fn get_lines_of_sight(
    chart: &Vec<Vec<u8>>,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> [Vec<u8>; 4] {
    let north = (0..y).map(|row| chart[row][x]).rev().collect();
    let east = (x + 1..width).map(|col| chart[y][col]).collect();
    let south = (y + 1..height).map(|row| chart[row][x]).collect();
    let west = (0..x).map(|col| chart[y][col]).rev().collect();
    [north, east, south, west]
}

fn part1() -> usize {
    let chart = get_chart();
    let (width, height) = get_dimensions(&chart);
    let mut num_visible = 0;
    for x in 0..width {
        for y in 0..height {
            let los = get_lines_of_sight(&chart, x, y, width, height);
            if los.iter().any(|trees| match trees.iter().max() {
                Some(c) => c < &chart[y][x],
                None => true
            }) { num_visible += 1; }
        }
    }

    num_visible
}

fn part2() -> usize {
    let chart = get_chart();
    let (width, height) = get_dimensions(&chart);
    let mut best_score = 0;
    for x in 0..width {
        for y in 0..height {
            let los = get_lines_of_sight(&chart, x, y, width, height);
            let score = los
                .map(|trees| {
                    let mut num_visible = trees.len();
                    for (i, tree) in trees.iter().enumerate() {
                        if *tree >= chart[y][x] {
                            num_visible = i + 1;
                            break;
                        }
                    }
                    num_visible
                })
                .iter()
                .product();
            best_score = max(best_score, score);
        }
    }

    best_score
}

fn main() {
    println!("{}", part1());  // 1825
    println!("{}", part2());  // 235200
}