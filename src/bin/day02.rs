// Q: How can you create a constant ``HashMap``?
// Q: Does Rust have a ``Map`` interface?

#[derive(PartialEq, Eq, Clone)]
enum Choice {
    ROCK,
    PAPER,
    SCISSOR,
}


fn win(opponent: &Choice) -> Choice {
    match opponent {
        Choice::ROCK => Choice::PAPER,
        Choice::PAPER => Choice::SCISSOR,
        Choice::SCISSOR => Choice::ROCK,
    }
}


fn lose(opponent: &Choice) -> Choice {
    match opponent {
        Choice::ROCK => Choice::SCISSOR,
        Choice::PAPER => Choice::ROCK,
        Choice::SCISSOR => Choice::PAPER,
    }
}


fn determine_points(opponent: &Choice, you: &Choice) -> u32 {
    let base = match you {
        Choice::ROCK => 1,
        Choice::PAPER => 2,
        Choice::SCISSOR => 3,
    };

    base + {
        if win(opponent) == *you {
            6
        } else if opponent == you {
            3
        } else {
            0
        }
    }
}


fn part1() -> u32 {
    aoc::io::get_input(2)
        .split("\n")
        .map(|line| {
            let (a, b) = line.split_once(" ").unwrap();
            let opponent = match a {
                "A" => Ok(Choice::ROCK),
                "B" => Ok(Choice::PAPER),
                "C" => Ok(Choice::SCISSOR),
                _ => Err(format!("Invalid choice: {}", a))
            }.unwrap();
            let you = match b {
                "X" => Ok(Choice::ROCK),
                "Y" => Ok(Choice::PAPER),
                "Z" => Ok(Choice::SCISSOR),
                _ => Err(format!("Invalid choice: {}", b))
            }.unwrap();
            determine_points(&opponent, &you)
        }).sum()
}


fn part2() -> u32 {
    aoc::io::get_input(2)
        .split("\n")
        .map(|line| {
            let (a, b) = line.split_once(" ").unwrap();
            let opponent = match a {
                "A" => Ok(Choice::ROCK),
                "B" => Ok(Choice::PAPER),
                "C" => Ok(Choice::SCISSOR),
                _ => Err(format!("Invalid choice: {}", a))
            }.unwrap();
            let you = match b {
                "X" => Ok(lose(&opponent)),
                "Y" => Ok(opponent.clone()),
                "Z" => Ok(win(&opponent)),
                _ => Err(format!("Invalid choice: {}", b))
            }.unwrap();
            determine_points(&opponent, &you)
        }).sum()
}


fn main() {
    println!("{}", part1());
    println!("{}", part2());
}