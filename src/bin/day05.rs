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
    let mut steps = Vec::new();
    for line in raw_steps.lines() {
        let mut words = line.split(" ");
        let num: usize = words.nth(1).unwrap().parse().unwrap();
        let from: usize = words.nth(1).unwrap().parse().unwrap();
        let to: usize = words.nth(1).unwrap().parse().unwrap();
        steps.push((num, from - 1, to - 1));
    }
    (stacks, steps)
}

fn solve(one_by_one: bool) -> String {
    let mut buffer = vec![' '; 100_000_000];
    let (mut stacks, steps) = parse_input();
    for (num, from, to) in steps {
        let from_size = stacks[from].len();
        let buffer = &mut buffer[0..num];  // Shadow
        buffer.copy_from_slice(&stacks[from][from_size - num..from_size]);
        stacks[from].truncate(from_size - num);
        if one_by_one {
            buffer.reverse();
        }
        stacks[to].extend(buffer.iter());
    }
    // Find the crate at the top of each stack and join them into a single String.
    stacks.iter().map(|t| t.last()).flatten().collect()
}

fn main() {
    println!("{}", solve(true));  // CFFHVVHNC
    println!("{}", solve(false));  // FSZWBPTBG
}