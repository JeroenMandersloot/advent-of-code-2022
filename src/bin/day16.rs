use std::cmp::{max, min};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::time::Instant;

use regex::Regex;

fn solve<'a>(
    origin: &'a str,
    valves: &'a HashMap<&str, usize>,
    distances: &HashMap<(&str, &str), usize>,
    minutes_remaining: usize,
    cache: &mut HashMap<BTreeSet<&'a str>, usize>,
    opened: &BTreeSet<&'a str>,
    score: usize,
) -> usize {
    let key = opened.clone();
    let prev = match cache.get(&key) {
        Some(v) => *v,
        None => 0
    };
    cache.insert(key, max(score, prev));
    if opened.len() == valves.len() {
        score
    } else {
        valves.into_iter().filter(|(valve, _)| !opened.contains(**valve)).map(|(valve, flow)| {
            let mut opened = opened.clone();
            opened.insert(valve);
            let key = (origin, valve as &str);
            let duration = distances.get(&key).unwrap() + 1;
            if duration > minutes_remaining {
                score
            } else {
                let minutes_remaining = minutes_remaining - duration;
                solve(valve, &valves, distances, minutes_remaining, cache, &opened, score + flow * minutes_remaining)
            }
        }).max().unwrap()
    }
}

fn parse(input: &String) -> (HashMap<&str, usize>, HashMap<(&str, &str), usize>) {
    let pattern = Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? ((?:[A-Z]+(?:, )?)+)").unwrap();
    let cms = pattern.captures_iter(&input);
    let mut edges = HashMap::new();
    let mut valves = HashMap::new();
    for captures in cms {
        let mut matches = captures.iter().skip(1).map(|m| m.unwrap().as_str());
        let origin = matches.next().unwrap();
        let flow = matches.next().unwrap().parse::<usize>().unwrap();
        if flow > 0 {
            valves.insert(origin, flow);
        }
        edges.insert(origin, matches.next().unwrap().split(", ").collect::<Vec<_>>());
    }

    let mut stack = Vec::new();
    for (start, neighbours) in &edges {
        for neighbour in neighbours {
            stack.push((*start, *neighbour, 1usize));
        }
    }

    let mut distances = HashMap::new();
    while !stack.is_empty() {
        let (start, end, distance) = stack.pop().unwrap();
        let key = (start, end);
        if !distances.contains_key(&key) || distance < *distances.get(&key).unwrap() {
            distances.insert(key, distance);
            let neighbours = edges.get(&end).unwrap();
            for neighbour in neighbours {
                stack.push((start, neighbour, distance + 1));
            }
        }
    }
    (valves, distances)
}

fn part1(input: &String) -> usize {
    let opened = BTreeSet::new();
    let mut cache = HashMap::new();
    let (valves, distances) = parse(input);
    solve("AA", &valves, &distances, 30, &mut cache, &opened, 0)
}

fn part2(input: &String) -> usize {
    let num_travelers = 2;
    let opened = BTreeSet::new();
    let mut cache = HashMap::new();
    let (valves, distances) = parse(&input);
    solve("AA", &valves, &distances, 26, &mut cache, &opened, 0);
    (1..num_travelers).map(|_| {
        let prev = cache.clone();
        prev.iter().enumerate().map(|(i, (opened, s))| {
            println!("{}/{})", i, prev.len() - 1);
            solve("AA", &valves, &distances, 26, &mut cache, opened, *s)
        }).max().unwrap()
    }).last().unwrap()
}

fn main() {
    let input = aoc::io::get_example(16);
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}