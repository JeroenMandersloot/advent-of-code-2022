use regex::Regex;

fn parse_input() -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let input = aoc::io::get_input(5);
    let (raw_config, raw_steps) = input.split_once("\n\n").unwrap();
    let num_stacks = raw_config.lines().last().unwrap().split(" ").count();
    let rows: Vec<_> = raw_config
        .split_once("\n 1")  // Hacky way to ignore line containing stack ids
        .unwrap().0
        .lines()
        .map(|line| format!("{:width$}", line, width = (4 * num_stacks as usize))
            .chars()
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|group| match group[1] {
                ' ' => None,
                c => Some(c)
            })
            .collect::<Vec<_>>()
        )
        .collect();

    // Transpose our rows into stacks.
    let num_rows = rows.len();
    let mut stacks = Vec::new();
    for i in 0..num_stacks {
        stacks.push((0..num_rows)
            .map(|j| rows[num_rows - j - 1][i])
            .flatten()
            .collect::<Vec<_>>()
        );
    }

    let mut steps = Vec::new();
    let pattern = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in raw_steps.lines() {
        let captures: Vec<usize> = pattern
            .captures_iter(line)
            .next()
            .unwrap()
            .iter()
            .flatten()
            .skip(1)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        if let [num, from, to] = captures[..] {
            steps.push((num, from - 1, to - 1));
        }
    }

    (stacks, steps)
}

fn part1() -> String {
    let (mut stacks, steps) = parse_input();
    for (num, from, to) in steps {
        for _ in 0..num {
            let block = stacks[from].pop().unwrap();
            stacks[to].push(block);
        }
    }
    stacks.iter().map(|t| t.last()).flatten().collect()
}

fn part2() -> String {
    let (mut stacks, steps) = parse_input();
    for (num, from, to) in steps {
        let num_remaining = &stacks[from].len() - num;
        let mut blocks: Vec<_> = stacks[from].drain(num_remaining..).collect();
        stacks[to].append( &mut blocks);
    }
    stacks.iter().map(|t| t.last()).flatten().collect()
}


fn main() {
    println!("{}", part1());
    println!("{}", part2());
}