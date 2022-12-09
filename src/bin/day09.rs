use std::collections::HashSet;

type Pos = (i32, i32);

fn move_head(head: &Pos, direction: &str) -> Pos {
    match direction {
        "U" => (head.0, head.1 + 1),
        "R" => (head.0 + 1, head.1),
        "D" => (head.0, head.1 - 1),
        "L" => (head.0 - 1, head.1),
        _ => panic!("{}", format!("Invalid direction: {}", direction))
    }
}

fn get_delta(delta: i32) -> i32 {
    match delta {
        c if c > 0 => 1,
        c if c < 0 => -1,
        _ => 0,
    }
}

fn move_tail(tail: &Pos, head: &Pos) -> Pos {
    let (x, y) = *tail;
    let xdiff = head.0 - x;
    let ydiff = head.1 - y;
    if xdiff.abs() <= 1 && ydiff.abs() <= 1 {
        (x, y)
    } else {
        (x + get_delta(xdiff), y + get_delta(ydiff))
    }
}

fn solve(num_knots: usize) -> usize {
    let mut knots = vec![(0, 0); num_knots];
    let mut history = HashSet::new();
    history.insert((0, 0));
    for line in aoc::io::get_input(9).lines() {
        let (direction, num_repeats) = line.split_once(" ").unwrap();
        let n = num_repeats.parse().unwrap();
        for _ in 0..n {
            let mut head = move_head(&knots[0], &direction);
            knots[0] = head;
            for tail in knots.iter_mut().skip(1) {
                head = move_tail(&tail, &head);
                *tail = head
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