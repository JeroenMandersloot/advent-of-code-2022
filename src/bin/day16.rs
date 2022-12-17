use std::cmp::max;
use std::collections::{BTreeSet, HashMap};

use regex::Regex;

fn simulate<'a>(
    origin: &str,
    valves: &HashMap<&'a str, usize>,
    distances: &HashMap<(&str, &str), usize>,
    minutes_remaining: usize,
    opened: &BTreeSet<&'a str>,
    score: usize,
    cache: &mut HashMap<BTreeSet<&'a str>, usize>,
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
                simulate(valve, &valves, distances, minutes_remaining, &opened, score + flow * minutes_remaining, cache)
            }
        }).max().unwrap()
    }
}

fn parse(input: &str) -> (HashMap<&str, usize>, HashMap<(&str, &str), usize>) {
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
            stack.push((*start, *neighbour, 1));
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

fn solve(input: &str, time: usize, num_travelers: usize) -> usize {
    let (valves, distances) = parse(&input);
    let mut cache = HashMap::new();
    cache.insert(BTreeSet::new(), 0);
    let mut score = 0;
    for _ in 0..num_travelers {
        let prev = cache.clone();
        score = prev.iter().enumerate().map(|(i, (opened, s))| {
            if (i + 1) % 1000 == 0 { println!("{}/{}", i + 1, prev.len()) }
            simulate("AA", &valves, &distances, time, opened, *s, &mut cache)
        }).max().unwrap();
    }
    score
}

fn main() {
    let input = aoc::io::get_input(16);
    println!("{}", solve(&input, 30, 1));
    println!("{}", solve(&input, 26, 2));  // Takes ~1 minute
}