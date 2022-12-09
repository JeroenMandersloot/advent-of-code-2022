use std::cmp::min;
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
    if xdiff.abs() <= 1 && ydiff.abs() <= 1 {
        return (x, y);
    }
    let xcorr = match xdiff {
        c if c > 0 => 1,
        c if c < 0 => -1,
        _ => 0,
    };
    let ycorr = match ydiff {
        c if c > 0 => 1,
        c if c < 0 => -1,
        _ => 0,
    };
    (x + xcorr, y + ycorr)
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

fn main() {
    println!("{}", solve(2)); // 6498
    println!("{}", solve(10)); // 2531
}