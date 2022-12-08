use std::cmp::max;

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

fn get_lines_of_sight(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> [Vec<(usize, usize)>; 4] {
    let north = (0..y).map(|row| (x, row)).rev().collect();
    let east = (x + 1..width).map(|col| (col, y)).collect();
    let south = (y + 1..height).map(|row| (x, row)).collect();
    let west = (0..x).map(|col| (col, y)).rev().collect();
    [north, east, south, west]
}

fn part1() -> u32 {
    let chart = get_chart();
    let (width, height) = get_dimensions(&chart);
    let mut num_visible = 0;
    for x in 0..width {
        for y in 0..height {
            let los = get_lines_of_sight(x, y, width, height);
            for coordinates in los {
                let mut is_visible = true;
                for (col, row) in coordinates {
                    if chart[row][col] >= chart[y][x] {
                        is_visible = false;
                        break;
                    }
                }

                if is_visible {
                    num_visible += 1;
                    break;
                }
            }
        }
    }

    num_visible
}

fn part2() -> u32 {
    let chart = get_chart();
    let (width, height) = get_dimensions(&chart);
    let mut best_score = 0;
    for x in 0..width {
        for y in 0..height {
            let los = get_lines_of_sight(x, y, width, height);
            let mut scores = [0; 4];
            for (i, coordinates) in los.iter().enumerate() {
                for (col, row) in coordinates {
                    scores[i] += 1;
                    if chart[*row][*col] >= chart[y][x] {
                        break;
                    }
                }
            }
            best_score = max(best_score, scores.iter().product());
        }
    }
    best_score
}

fn main() {
    println!("{}", part1());  // 1825
    println!("{}", part2());  // 235200
}