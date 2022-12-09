use std::collections::HashSet;

type Pos = (i32, i32);

fn move_head(head: &Pos, direction: &char) -> Pos {
    match direction {
        'U' => (head.0, head.1 + 1),
        'R' => (head.0 + 1, head.1),
        'D' => (head.0, head.1 - 1),
        'L' => (head.0 - 1, head.1),
        _ => panic!("{}", format!("Invalid direction: {}", direction))
    }
}

fn move_tail(tail: &Pos, head: &Pos) -> Pos {
    let xdiff = head.0 - tail.0;
    let ydiff = head.1 - tail.1;
    let (x, y) = *tail;
    match (xdiff, ydiff) {
        // 2 squares away in a single direction.
        (2, 0) => (x + 1, y),
        (-2, 0) => (x - 1, y),
        (0, 2) => (x, y + 1),
        (0, -2) => (x, y - 1),

        // A knight's move away.
        (1, 2) => (x + 1, y + 1),
        (2, 1) => (x + 1, y + 1),
        (2, -1) => (x + 1, y - 1),
        (1, -2) => (x + 1, y - 1),
        (-1, -2) => (x - 1, y - 1),
        (-2, -1) => (x - 1, y - 1),
        (-2, 1) => (x - 1, y + 1),
        (-1, 2) => (x - 1, y + 1),

        // Double diagonal away (only relevant for part 2).
        (2, 2) => (x + 1, y + 1),
        (2, -2) => (x + 1, y - 1),
        (-2, 2) => (x - 1, y + 1),
        (-2, -2) => (x - 1, y - 1),

        // In all other cases the tail doesn't have to move.
        _ => (x, y)
    }
}

fn solve(num_knots: usize) -> usize {
    let mut knots = vec![(0, 0); num_knots];
    let mut history = HashSet::new();
    history.insert((0, 0));
    for line in aoc::io::get_input(9).lines() {
        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        // Skip space
        chars.next();
        // Remaining characters make up the number of times to repeat the move.
        let num = chars.collect::<String>().parse().unwrap();
        for _ in 0..num {
            let mut head = knots[0];
            let mut new_head = move_head(&head, &direction);
            knots[0] = new_head;
            for tail in knots.iter_mut().skip(1) {
                let new_tail = move_tail(&tail, &new_head);
                if *tail == new_tail {
                    break;
                }
                head = *tail;
                new_head = new_tail;
                *tail = new_tail
            }
            history.insert(knots[num_knots - 1]);
        }
    }

    history.len()
}

fn part1() -> usize {
    solve(2)
}

fn part2() -> usize {
    solve(10)
}

fn main() {
    println!("{}", part1()); // 6498
    println!("{}", part2()); // 2531
}