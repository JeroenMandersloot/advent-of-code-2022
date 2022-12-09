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
    let xdiff = (head.0 - tail.0).abs();
    let ydiff = (head.1 - tail.1).abs();
    return (xdiff == 2 && ydiff <= 2) || (xdiff <= 2 && ydiff == 2);
}

// fn draw_history(history: &HashSet<Pos>) -> Vec<Vec<char>> {
//     let xs = history.iter().map(|(x, y)| x);
//     let xmin = xs.min().unwrap();
//     let xmax = xs.max().unwrap();
//     let ymin = history.iter().map(|(x, y)| y).min().unwrap();
//     let ymax = history.iter().map(|(x, y)| y).max().unwrap();
//     let width = (xmax - xmin + 1) as usize;
//     let height = (ymax - ymin + 1) as usize;
//
//     let row = vec!['.'; width];
//     let mut chart = Vec::new();
//     for _ in 0..height {
//         chart.push(row.clone());
//     }
//
//     for (x, y) in history {
//         let point = ((x - xmin) as usize, (y - ymin) as usize);
//         chart[height - 1 - point.1][point.0] = '#';
//     }
//
//     let origin = (-xmin as usize, -ymin as usize);
//     chart[height - 1 - origin.1][origin.0] = 's';
//
//     println!("{}", chart.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<_>>().join("\n"));
// }

// fn part1() -> usize {
    // let mut head = (0, 0);
    // let mut tail = (0, 0);
    // let mut history = HashSet::new();
    // history.insert(tail);
    // for line in aoc::io::get_input(9).lines() {
    //     let mut chars = line.chars();
    //     let direction = chars.next().unwrap();
    //     // Skip space
    //     chars.next();
    //     // Remaining characters make up the number of times to repeat the move.
    //     let num = chars.collect::<String>().parse().unwrap();
    //     for _ in 0..num {
    //         let (new_head, new_tail) = m(&head, &tail, &direction);
    //         history.insert(new_tail);
    //         head = new_head;
    //         tail = new_tail;
    //     }
    // }
    //
    // history.len()
    // 8
// }

fn solve(num_knots: usize) -> usize {
    let mut knots = vec![(0, 0); num_knots];
    let mut history = HashSet::new();
    history.insert((0, 0));
    for line in aoc::io::get_example(9).lines() {
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
                if !move_tail(&tail, &new_head) {
                    break;
                }
                let new_tail = head;
                head = *tail;
                new_head = new_tail;
                *tail = new_tail
            }
            println!("{:?}", knots);
            history.insert(knots[num_knots - 1]);
        }
        println!("---");
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
    // println!("{}", part1()); // 6498
    println!("{}", part2());
}