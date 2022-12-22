use std::collections::{HashMap, HashSet};

use regex::Regex;

type Grid = HashMap<(i32, i32), Option<bool>>;


fn parse(input: &str) -> (Grid, Vec<&str>) {
    let height = input.lines().count() - 2;
    let width = input.lines().take(height).map(|line| line.len()).max().unwrap();
    let mut grid = HashMap::new();

    for (y, line) in input.lines().enumerate().take(height) {
        let line = format!("{:width$}", line, width = width + 1).chars().collect::<String>();
        for (x, c) in line.chars().enumerate() {
            grid.insert((x as i32, y as i32), match c {
                ' ' => None,
                '.' => Some(false),
                '#' => Some(true),
                a => panic!("Invalid character: {a}")
            });
        }
    }

    let instructions = Regex::new(r"(\d+|[RL])").unwrap().captures_iter(input.lines().last().unwrap()).map(|captures| captures.iter().next().unwrap().unwrap().as_str()).collect();
    (grid, instructions)
}

fn draw(grid: &Grid) -> String {
    let width = grid.keys().map(|(x, _)| *x).max().unwrap();
    let height = grid.keys().map(|(_, y)| *y).max().unwrap();
    let mut drawing = String::from("");
    for y in 0..height {
        for x in 0..width {
            drawing.push(match grid.get(&(x, y)).unwrap() {
                None => ' ',
                Some(true) => '#',
                Some(false) => '.',
            });
        }
        drawing.push('\n');
    }
    drawing
}

fn part1(input: &str) -> i32 {
    let (grid, instructions) = parse(&input);
    let width = grid.keys().map(|(x, _)| *x).max().unwrap();
    let height = grid.keys().map(|(_, y)| *y).max().unwrap();
    let mut position = (0i32, 0i32);
    position.0 = *grid.iter().filter(|((_, y), v)| *y == 0 && **v == Some(false)).map(|((x, _), _)| x).min().unwrap();

    let mut history = Vec::new();
    history.push(position);
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut direction_idx = 0;
    for instruction in instructions {
        if let Ok(num_steps) = instruction.parse::<u8>() {
            let direction = directions[direction_idx];
            for _ in 0..num_steps {
                let mut new = position;
                loop {
                    new.0 = (new.0 + direction.0).rem_euclid(width);
                    new.1 = (new.1 + direction.1).rem_euclid(height);
                    if *grid.get(&new).unwrap() != None {
                        break;
                    }
                }
                if let Some(true) = grid.get(&new).unwrap() {
                    break;
                }
                position = new;
                history.push(position);
            }
        } else {
            let m = match instruction {
                "R" => 1,
                "L" => -1,
                v => panic!("Invalid instruction: {v}")
            };
            direction_idx = (direction_idx as i8 + m).rem_euclid(4) as usize;
        }
    }

    let (x, y) = position;
    (y + 1) * 1000 + 4 * (x + 1) + direction_idx as i32
}

fn main() {
    let input = aoc::io::get_input(22);
    println!("{:?}", part1(&input));
    // println!("{}", part2(&input, is_example));
}