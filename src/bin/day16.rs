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
) -> HashMap<BTreeSet<&'a str>, usize> {
    let mut options: HashMap<BTreeSet<&str>, usize> = HashMap::new();
    options.insert(opened.clone(),score);
    for (valve, flow) in valves {
        if !opened.contains(valve) {
            let mut opened = opened.clone();
            opened.insert(valve);
            let key = (origin, valve as &str);
            let duration = distances.get(&key).unwrap() + 1;
            if duration <= minutes_remaining {
                let minutes_remaining = minutes_remaining - duration;
                let results = simulate(valve, &valves, distances, minutes_remaining, &opened, score + flow * minutes_remaining);
                for (opened, score) in results {
                    let previous = match options.get(&opened) {
                        Some(score) => *score,
                        None => 0
                    };
                    options.insert(opened, max(previous, score));
                }
            }
        }
    }
    return options;
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

fn solve(input: &str, time: usize, elephant: bool) -> usize {
    let (valves, distances) = parse(&input);
    let opened = BTreeSet::new();
    let you = simulate("AA", &valves, &distances, time, &opened, 0);
    if elephant {  // Part 2
        you.iter().map(|(ignore, score)| {
            let valves = valves.iter().filter(|(valve, _)| !ignore.contains(*valve)).map(|(v, f)| (*v, *f)).collect();
            let result = simulate("AA", &valves, &distances, time, &opened, 0);
            score + *result.iter().map(|(_, s)| s).max().unwrap()
        }).max().unwrap()
    } else {  // Part 1
        *you.iter().map(|(_, score)| score).max().unwrap()
    }
}

fn main() {
    let input = aoc::io::get_input(16);
    println!("{}", solve(&input, 30, false));
    println!("{}", solve(&input, 26, true));  // Takes ~2 minutes
}