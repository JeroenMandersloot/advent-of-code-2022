use std::collections::HashSet;

type Pos = (i32, i32);

fn m(head: &Pos, tail: &Pos, direction: &char) -> (Pos, Pos) {
    let new_head = match direction {
        'U' => (head.0, head.1 + 1),
        'R' => (head.0 + 1, head.1),
        'D' => (head.0, head.1 - 1),
        'L' => (head.0 - 1, head.1),
        _ => panic!("{}", format!("Invalid direction: {}", direction))
    };

    let xdiff = (new_head.0 - tail.0).abs();
    let ydiff = (new_head.1 - tail.1).abs();
    if (xdiff == 2 && ydiff <= 1) || (xdiff <= 1 && ydiff == 2) {
        (new_head, *head)
    } else {
        (new_head, *tail)
    }
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

fn part1() -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut history = HashSet::new();
    history.insert(tail);
    for line in aoc::io::get_input(9).lines() {
        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        // Skip space
        chars.next();
        // Remaining characters make up the number of times to repeat the move.
        let num = chars.collect::<String>().parse().unwrap();
        for _ in 0..num {
            let (new_head, new_tail) = m(&head, &tail, &direction);
            history.insert(new_tail);
            head = new_head;
            tail = new_tail;
        }
    }

    history.len()
}

fn part2() -> u8 {
    0
}

fn main() {
    println!("{}", part1()); // 6498
    println!("{}", part2());
}