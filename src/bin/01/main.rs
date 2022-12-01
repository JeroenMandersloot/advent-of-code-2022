use std::fs;

fn get_calories() -> Vec<u32> {
    fs::read_to_string("inputs/01/part1.txt")
        .unwrap()
        .split("\n\n")
        .into_iter()
        .map(|elf| elf
            .split("\n")
            .into_iter()
            .map(|i| i.parse::<u32>().unwrap()).sum())
        .collect::<Vec<u32>>()
}

#[allow(dead_code)]
fn part1() {
    let result = get_calories().into_iter().max().unwrap();
    println!("{}", result);
}


#[allow(dead_code)]
fn part2() {
    let mut calories = get_calories();
    calories.sort();
    calories.reverse();
    let result: u32 = calories[..3].into_iter().sum();
    println!("{:?}", result);
}

fn main() {
    part1();
    part2();
}