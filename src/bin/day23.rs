use std::collections::{HashMap, HashSet};

type Pos = (i32, i32);

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
    let directions = ['N', 'S', 'W', 'E'];
    let mut i = 0;
    loop {
        let mut num_changed = 0;
        let mut targets = HashMap::new();
        for &elf in elves.iter() {
            let (x, y) = elf;
            let neighbours = [
                (x - 1, y - 1),
                (x - 1, y + 1),
                (x + 1, y - 1),
                (x + 1, y + 1),
                (x, y - 1),
                (x, y + 1),
                (x - 1, y),
                (x + 1, y)
            ];
            let num_occupied = neighbours.iter().filter(|c| elves.contains(c)).count();
            if num_occupied == 0 {
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

                let num_occupied = check.iter().filter(|c| elves.contains(c)).count();
                if num_occupied == 0 {
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
                num_changed += 1;
                elves.remove(&candidates[0]);
                elves.insert(target);
            }
        }

        i += 1;

        // Part 1
        if i == 10 {
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
            println!("{solution}");
        }

        // Part 2
        if num_changed == 0 {
            println!("{i}");
            break;
        }
    };
}