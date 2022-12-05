use regex::Regex;

fn parse_input() -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let input = aoc::io::get_input(5);
    let (raw_config, raw_steps) = input.split_once("\n\n").unwrap();
    let num_stacks = raw_config.lines().last().unwrap().split(" ").count();
    let rows: Vec<_> = raw_config
        .split_once("\n 1")  // Hacky way to ignore line containing stack ids.
        .unwrap().0  // Get all lines that specify the initial crate configuration.
        .lines()
        // Pad each line with spaces until they are all equally long.
        // Each individual crate takes up at most 4 characters:
        //  - the opening bracket: [
        //  - the ID of the crate
        //  - the closing bracket: ]
        //  - an optional space character separating this crate from the next
        .map(|line| format!("{:width$}", line, width = (4 * num_stacks as usize))
            .chars()
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|group| match group[1] {  // The second character holds the crate ID.
                ' ' => None,  // A space character means no crate exists here, so ignore it.
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
            // For each stack, get the corresponding crate from each row from bottom to top.
            .map(|j| rows[num_rows - j - 1][i])
            // Remove all ``None`` values since that means no crate was present.
            .flatten()
            .collect::<Vec<_>>()
        );
    }

    // Regex pattern with capture groups to extract the relevant values for each step.
    let pattern = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut steps = Vec::new();
    for line in raw_steps.lines() {
        let captures: Vec<usize> = pattern
            .captures_iter(line)
            .next()  // Each line should only yield a single match.
            .unwrap()
            .iter()  // Loop over each of the capture groups.
            .flatten()
            .skip(1)  // The first group holds the entire match, skip it.
            .map(|m| m.as_str().parse().unwrap())
            .collect();

        // Destructure the remaining 3 matched capture groups.
        if let [num, from, to] = captures[..] {
            steps.push((num, from - 1, to - 1));
        }
    }

    (stacks, steps)
}

fn part1() -> String {
    let (mut stacks, steps) = parse_input();
    for (num, from, to) in steps {
        // For each crate that needs to be moved...
        for _ in 0..num {
            // ...pick the top crate from the stack where it should be moved from...
            let block = stacks[from].pop().unwrap();
            // ...and place it on top of the stack where it needs to be moved to.
            stacks[to].push(block);
        }
    }
    // Find the crate at the top of each stack and join them into a single String.
    stacks.iter().map(|t| t.last()).flatten().collect()
}

fn part2() -> String {
    let (mut stacks, steps) = parse_input();
    for (num, from, to) in steps {
        let num_remaining = &stacks[from].len() - num;
        // Pick up all crates that need to be moved at once...
        let mut blocks: Vec<_> = stacks[from].drain(num_remaining..).collect();
        // ...and place them on top of the stack they should be moved to.
        stacks[to].append( &mut blocks);
    }
    // Find the crate at the top of each stack and join them into a single String.
    stacks.iter().map(|t| t.last()).flatten().collect()
}


fn main() {
    println!("{}", part1());
    println!("{}", part2());
}