use std::fs;

fn get_calories() -> Vec<u32> {
    fs::read_to_string("inputs/01/part1.txt")
        .unwrap()
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
    calories[..3].iter().sum::<u32>()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}