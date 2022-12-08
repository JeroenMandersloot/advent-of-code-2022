use std::cmp::max;
use std::collections::HashMap;

fn get_chart() -> Vec<Vec<u8>> {
    aoc::io::get_input(8)
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|tree| tree.to_digit(10).unwrap() as u8)
                .collect()
        }).collect()
}

fn get_dimensions<T>(chart: &Vec<Vec<T>>) -> (usize, usize) {
    let width = chart.iter().map(|x| x.len()).max().unwrap();
    let height = chart.len();

    (width, height)
}

fn get_relevant_coordinates(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> [Vec<(usize, usize)>; 4] {
    let north = (0..y).map(|row| (x, row)).rev().collect();
    let south = (y + 1..height).map(|row| (x, row)).collect();
    let east = (x + 1..width).map(|col| (col, y)).collect();
    let west = (0..x).map(|col| (col, y)).rev().collect();

    [north, east, south, west]
}

fn get_scenic_score(chart: &Vec<Vec<u8>>, x: usize, y: usize) -> u32 {
    let (width, height) = get_dimensions(chart);  // TODO: pass as arguments.
    let coordinates = get_relevant_coordinates(x, y, width, height);
    let mut total = 1;
    let me = &chart[y][x];
    for direction in coordinates {
        let mut score = 0;
        for (col, row) in direction {
            score += 1;
            if chart[row][col] >= *me {
                break;
            }
        }
        total *= score;
    }
    total
}

fn part1() -> u32 {
    let chart = get_chart();
    let (width, height) = get_dimensions(&chart);

    let mut mask: Vec<Vec<bool>> = vec![vec![false; width]; height];

    for x in 0..width {
        for y in 0..height {
            if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                mask[y][x] = true;
            }
        }
    }

    for x in 0..width {
        let mut tallest: u8 = 0;
        for y in 0..height {
            let c = &chart[y][x];
            if *c > tallest {
                mask[y][x] = true;
            }
            tallest = max(tallest, *c);
        }
    }

    for x in 0..width {
        let mut tallest: u8 = 0;
        for y in (0..height).rev() {
            let c = &chart[y][x];
            if *c > tallest {
                mask[y][x] = true;
            }
            tallest = max(tallest, *c);
        }
    }

    for y in 0..height {
        let mut tallest: u8 = 0;
        for x in 0..width {
            let c = &chart[y][x];
            if *c > tallest {
                mask[y][x] = true;
            }
            tallest = max(tallest, *c);
        }
    }

    for y in 0..height {
        let mut tallest: u8 = 0;
        for x in (0..width).rev() {
            let c = &chart[y][x];
            if *c > tallest {
                mask[y][x] = true;
            }
            tallest = max(tallest, *c);
        }
    }

    let mut count = 0;
    for x in 0..width {
        for y in 0..height {
            if mask[y][x] {
                count += 1;
            }
        }
    }

    count
}

fn part2() -> u32 {
    let chart = get_chart();
    let (width, height) = get_dimensions(&chart);
    let mut best_score = 0;
    for x in 0..width {
        for y in 0..height {
            best_score = max(best_score, get_scenic_score(&chart, x, y));
        }
    }
    best_score
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}