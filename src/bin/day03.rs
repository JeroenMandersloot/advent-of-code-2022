// Q: Why does ``&str.split("")`` yield an empty string at the start?
// Q: How would you implement a ``Counter`` in Rust?

use std::collections::HashSet;

const PRIORITIES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";


fn part1() -> u32 {
    aoc::io::get_input(3)
        .split("\n")
        .map(|line| {
            let rucksacks = line.split_at(line.len() / 2);
            let a: HashSet<_> = rucksacks.0.chars().collect();
            let b: HashSet<_> = rucksacks.1.chars().collect();
            1 + PRIORITIES.find(*a.intersection(&b).next().unwrap()).unwrap() as u32
        })
        .sum()
}

fn part2() -> u32 {
    aoc::io::get_input(3)
        .split("\n")
        .map(str::chars)
        .map(HashSet::<char>::from_iter)
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| {
            let mut x: HashSet<_> = PRIORITIES.chars().collect();
            x.retain(|a| group.iter().all(|b| b.contains(a)));
            1 + PRIORITIES.find(*x.iter().next().unwrap()).unwrap() as u32
        })
        .sum()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}