use std::cmp::{max, min};
use std::collections::HashSet;

fn solve(part: usize) -> usize {
    let mut cave = HashSet::new();
    let input = aoc::io::get_input(14);
    for line in input.lines() {
        line.split(" -> ")
            .map(|coordinates| {
                let (x, y) = coordinates.split_once(",").unwrap();
                (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
            })
            .reduce(|(x1, y1), (x2, y2)| {
                for x in min(x1, x2)..(max(x1, x2) + 1) {
                    for y in min(y1, y2)..(max(y1, y2) + 1) {
                        cave.insert((x, y));
                    }
                }
                (x2, y2)
            });
    }

    let height = *cave.iter().map(|(_, y)| y).max().unwrap();
    let source: (usize, usize) = (500, 0);
    let num_rocks = cave.len();
    while !cave.contains(&source)  {
        let mut sand = source;
        let mut done = false;
        while !done && sand.1 < height + 1 {
            done = true;
            let (x, y) = sand;
            for c in [x, x - 1, x + 1] {
                let candidate = (c, y + 1);
                if !cave.contains(&candidate) {
                    sand = candidate;
                    done = false;
                    break;
                }
            }
        }
        if !done && part == 1 { break }
        cave.insert(sand);
    }
    cave.len() - num_rocks
}

fn main() {
    println!("{}", solve(1));
    println!("{}", solve(2));
}