use std::cmp::max;
use std::collections::{HashMap, HashSet};

const WIDTH: usize = 7;

fn get_shape_coordinates(shape: &Vec<(usize, usize)>, origin: (usize, usize)) -> Vec<(usize, usize)> {
    shape.iter().map(|(x, y)| (origin.0 + x, origin.1 + y)).collect()
}

fn collides(grid: &HashSet<(usize, usize)>, shape: &Vec<(usize, usize)>, origin: (usize, usize)) -> bool {
    let cs = get_shape_coordinates(shape, origin);
    cs.into_iter().filter(|c| grid.contains(c)).count() > 0
}

fn simulate(instructions: &str, until: usize) -> usize {
    let shapes: [Vec<(usize, usize)>; 5] = [
        Vec::from([(0, 0), (1, 0), (2, 0), (3, 0)]),
        Vec::from([(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
        Vec::from([(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]),
        Vec::from([(0, 0), (0, 1), (0, 2), (0, 3)]),
        Vec::from([(0, 0), (0, 1), (1, 0), (1, 1)]),
    ];

    let mut grid = HashSet::new();
    let mut height: usize = 0;
    let mut shape_idx: usize = 0;
    let mut instruction_idx: usize = 0;

    // We keep track of when we encounter each ``(shape_idx, instruction_idx)``
    // combination so that we can detect repeating patterns. This allows us to
    // skip ahead many, many iterations so that we can predict the height even
    // after trillions of shapes have stopped. Each time we see a particular
    // combination, we store the iteration number ``it`` and the ``height`` of
    // the grid at that point in time.
    let mut cycle_detector: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    let mut it: usize = 0;
    while it < until {
        it += 1;
        let key = (shape_idx, instruction_idx);
        let value = (it, height);
        if !cycle_detector.contains_key(&key) {
            let mut tracker = Vec::new();
            tracker.push(value);
            cycle_detector.insert(key, tracker);
        } else {
            let tracker = cycle_detector.get_mut(&key).unwrap();
            tracker.push(value);

            // Once we've encountered this combination 3 times we should get
            // a reliable pattern.
            if tracker.len() == 3 {
                let (base_it, base_height) = tracker[1];
                let (next_it, next_height) = tracker[2];
                let it_step = next_it - base_it;
                let skip_steps = (until - base_it) / it_step;

                // Fast forward to the nearest iteration that is still below
                // the number of stopped blocks we're interested in.
                it = base_it + skip_steps * it_step;
                let new_height = base_height + skip_steps * (next_height - base_height);
                let height_diff = new_height - height;
                height = new_height;
                let new_grid = HashSet::from_iter(grid.iter().map(|(x, y)| (*x, *y + height_diff)));
                grid = new_grid;
            }
        }

        let mut x = 2;
        let mut y = height + 3;
        let shape = &shapes[shape_idx];
        let shape_width = shape.iter().map(|(x, _)| x).max().unwrap() + 1;

        loop {
            // Move sideways.
            let instruction = instructions.chars().nth(instruction_idx).unwrap();
            let c1 = match instruction {
                '>' if x + shape_width < WIDTH => (x + 1, y),
                '<' if x > 0 => (x - 1, y),
                _ => (x, y)
            };
            if !collides(&grid, shape, c1) {
                (x, y) = c1;
            }

            // Prepare the next instruction.
            instruction_idx = (instruction_idx + 1) % instructions.len();

            // Move down.
            if y > 0 && !collides(&grid, shape, (x, y - 1)) {
                y -= 1;
            } else {
                let shape_coordinates = get_shape_coordinates(shape, (x, y));
                for coordinate in &shape_coordinates {
                    grid.insert(*coordinate);
                }

                // Update the height of the grid if the stopped block has
                // increased the total height. (It could be the block stopped
                // at a lower position than the previous highest block, so that
                // is why we must take the ``max()`` here).
                height = max(shape_coordinates.into_iter().map(|(_, y)| y).max().unwrap() + 1, height);

                // Move on to the next shape.
                shape_idx = (shape_idx + 1) % shapes.len();
                break;
            }
        }
    }

    height
}


fn main() {
    let instructions = aoc::io::get_input(17);
    println!("{}", simulate(&instructions, 2022));  // 3224
    println!("{}", simulate(&instructions, 1_000_000_000_000));  // 1595988538691
}