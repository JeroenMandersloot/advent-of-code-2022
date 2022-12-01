fn get_calories() -> Vec<u32> {
    aoc::io::get_input(1)
        .split("\n\n")
        .map(|elf| elf
            .split("\n")
            .map(|i| i
                .parse::<u32>()
                .unwrap())
            .sum())
        .collect::<Vec<u32>>()
}

#[allow(dead_code)]
fn part1() -> u32 {
    *get_calories().iter().max().unwrap()
}


#[allow(dead_code)]
fn part2() -> u32 {
    let mut calories = get_calories();
    calories.sort();
    calories.reverse();
    calories[..3].iter().sum()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}