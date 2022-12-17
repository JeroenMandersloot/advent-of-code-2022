use std::cmp::{max, min};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::time::Instant;

use regex::Regex;

fn solve<'a>(
    origin: &'a str,
    valves: &HashMap<&'a str, usize>,
    distances: &'a HashMap<&str, HashMap<&str, usize>>,
    minutes_remaining: usize,
    cache: &mut HashMap<BTreeSet<&'a str>, usize>,
) -> usize {
    if minutes_remaining <= 0 || valves.is_empty() {
        0
    } else {
        let solution = valves.into_iter().map(|(valve, flow)| {
            let duration = distances.get(origin).unwrap().get(valve).unwrap() + 1;
            let minutes_remaining = if duration > minutes_remaining { 0 } else { minutes_remaining - duration };
            let mut valves = valves.clone();
            valves.remove(valve);
            flow * minutes_remaining + solve(valve, &valves, distances, minutes_remaining, cache)
        }).max().unwrap();
        if solution > 0 {
            cache.insert(BTreeSet::from_iter(valves.clone().into_keys()), solution);
        }
        solution
    }
}

fn distance_matrix<'a>(edges: &'a HashMap<&str, Vec<&str>>) -> HashMap<&'a str, HashMap<&'a str, usize>> {
    let num_nodes = edges.len();
    let mut matrix = HashMap::new();
    for start in edges.keys() {
        let mut distances = HashMap::new();
        let mut visited = HashSet::new();
        let queue: Vec<_> = edges.keys().collect();
        distances.insert(*start, 0);
        while visited.len() < num_nodes {
            let current = **queue
                .iter()
                .filter(|key| !visited.contains(***key))
                .min_by_key(|key| match distances.get(***key) {
                    Some(d) => *d,
                    None => std::usize::MAX
                })
                .unwrap();
            visited.insert(current);
            let neighbours = edges.get(current).unwrap().iter().filter(|key| !visited.contains(**key));
            for neighbour in neighbours {
                if let Some(distance) = distances.get(current) {
                    if let Some(d) = distances.get(neighbour) {
                        if distance < d {
                            distances.insert(*neighbour, *distance + 1);
                        }
                    } else {
                        distances.insert(*neighbour, *distance + 1);
                    }
                }
            }
        }
        matrix.insert(*start, distances);
    }
    matrix
}

fn parse() -> usize {
    let input = aoc::io::get_example(16);
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

    let distances = distance_matrix(&edges);
    let mut cache = HashMap::new();
    let score = solve("AA", &valves, &distances, 30, &mut cache);

    // let travelers = ["you", "elephant"];
    // let mut score = 0;
    // for _ in travelers {
    //     if cache.is_empty() {
    //         score = solve("AA", &valves, &distances, 26, &mut cache)
    //     } else {
    //         let prev = cache.clone();
    //         score = prev.iter().map(|(remaining, s)| {
    //             let a = remaining.iter().map(|valve| (*valve, *valves.get(valve).unwrap())).collect();
    //             let r = solve("AA", &a, &distances, 26, &mut cache);
    //
    //             dbg!((s, r, &a));
    //             r
    //         }).max().unwrap()
    //     }
    // }

    score
}

fn main() {
    println!("{}", parse());
}