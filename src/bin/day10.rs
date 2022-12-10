fn parse_instructions() -> Vec<i32> {
    let input = aoc::io::get_input(10);
    let mut x: i32 = 1;
    let mut history = Vec::new();
    for line in input.lines() {
        if line == "noop" {
            history.push(x);
        } else if line.starts_with("addx ") {
            history.push(x);
            x += &line[5..].parse().unwrap();
            history.push(x);
        }
    }
    history
}

fn part1() -> i32 {
    let history = parse_instructions();
    let mut i = 20;
    let mut sum = 0;
    loop {
        match history.get(i - 2) {
            Some(x) => sum += x * (i as i32),
            None => break sum
        }
        i += 40;
    }
}

fn part2() -> String {
    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;
    let num_cycles = WIDTH * HEIGHT;
    let mut screen = [[' '; WIDTH]; HEIGHT];
    let mut x = 1;
    let history = parse_instructions();
    for cycle in 0..num_cycles {
        let row = cycle / WIDTH;
        let col = cycle % WIDTH;
        if (col as i32) >= x - 1 && (col as i32) <= x + 1 {
            screen[row][col] = '#';
        }
        x = history[cycle];
    }
    screen.map(|row| row.iter().collect::<String>()).join("\n")
}

fn main() {
    println!("{}", part1());  // 13220
    println!("{}", part2());  // RUAKHBEK
}