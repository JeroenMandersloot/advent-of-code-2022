use std::collections::{HashMap, HashSet};

type Pos = (i32, i32);

fn draw(elves: &HashSet<Pos>) {
    // let xmin = elves.iter().map(|(x, _)| *x).min().unwrap() - 2;
    // let xmax = elves.iter().map(|(x, _)| *x).max().unwrap() + 3;
    // let ymin = elves.iter().map(|(_, y)| *y).min().unwrap() - 2;
    // let ymax = elves.iter().map(|(_, y)| *y).max().unwrap() + 3;
    let (xmin, xmax, ymin, ymax) = (0, 5, 0, 6);

    let mut drawing = String::from("");
    for y in ymin..ymax {
        for x in xmin..xmax {
            if !elves.contains(&(x, y)) {
                drawing.push('.');
            } else {
                drawing.push('#');
            }
        }
        drawing.push('\n');
    }

    println!("{drawing}");
}

fn part1(elves: &mut HashSet<Pos>) -> u32 {
    let directions = ['N', 'S', 'W', 'E'];

    draw(elves);
    for i in 0..10 {
        println!("{}", directions[i % 4]);
        let mut targets = HashMap::new();
        for &elf in elves.iter() {
            let (x, y) = elf;
            let a = [
                (x - 1, y - 1),
                (x - 1, y + 1),
                (x + 1, y - 1),
                (x + 1, y + 1),
                (x, y - 1),
                (x, y + 1),
                (x - 1, y),
                (x + 1, y)
            ];
            let num_empty = a.iter().filter(|c| !elves.contains(c)).count();
            if num_empty == 8 {
                continue;
            }

            for d in 0..4 {
                let direction = directions[(d + i) % 4];
                let check = match direction {
                    'N' => [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1)],
                    'S' => [(x - 1, y + 1), (x, y + 1), (x + 1, y + 1)],
                    'W' => [(x - 1, y - 1), (x - 1, y), (x - 1, y + 1)],
                    'E' => [(x + 1, y - 1), (x + 1, y), (x + 1, y + 1)],
                    _ => panic!(),
                };

                let num_empty = check.iter().filter(|c| !elves.contains(c)).count();
                if num_empty == 3 {
                    let target = check[1];
                    if !targets.contains_key(&target) {
                        targets.insert(target, Vec::new());
                    }
                    targets.get_mut(&target).unwrap().push(elf);
                    break;
                }
            }
        }

        for (&target, candidates) in targets.iter() {
            if candidates.len() == 1 {
                elves.remove(&candidates[0]);
                elves.insert(target);
            }
        }

        draw(elves);
    }

    let xmin = elves.iter().map(|(x, _)| *x).min().unwrap();
    let xmax = elves.iter().map(|(x, _)| *x).max().unwrap() + 1;
    let ymin = elves.iter().map(|(_, y)| *y).min().unwrap();
    let ymax = elves.iter().map(|(_, y)| *y).max().unwrap() + 1;

    let mut solution = 0;
    for x in xmin..xmax {
        for y in ymin..ymax {
            if !elves.contains(&(x, y)) {
                solution += 1;
            }
        }
    }

    solution
}

fn part2() -> u32 {
    0
}

fn parse(input: &str) -> HashSet<Pos> {
    let mut elves = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert((x as i32, y as i32));
            }
        }
    }
    elves
}

fn main() {
    let input = aoc::io::get_input(23);
    let mut elves = parse(&input);
    println!("{}", part1(&mut elves));
    // println!("{}", part2());
}