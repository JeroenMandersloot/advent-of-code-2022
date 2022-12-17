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
            let duration = distances.get(origin).unwrap().get(valve).unwrap() + 1;
            if duration > minutes_remaining {
                score
            } else {
                let minutes_remaining = minutes_remaining - duration;
                solve(valve, &valves, distances, minutes_remaining, cache, &opened, score + flow * minutes_remaining)
            }
        }).max().unwrap()
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
    let input = aoc::io::get_input(16);
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
    let opened = BTreeSet::new();
    let mut cache = HashMap::new();
    // let score = solve("AA", &valves, &distances, 30, &mut cache, &opened, 0);
    // let mut a = cache.into_iter().map(|(k, _)| k.into_iter().collect::<Vec<_>>().join(",")).collect::<Vec<_>>();
    // a.sort();
    // println!("{:?}", a.join(";"));

    let now = Instant::now();
    let travelers = ["you", "elephant"];
    let mut score = 0;
    for _ in travelers {
        if cache.is_empty() {
            score = solve("AA", &valves, &distances, 26, &mut cache, &opened, 0);
        } else {
            let prev = cache.clone();
            score = prev.iter().enumerate().map(|(i, (opened, s))| {
                println!("{}/{} ({:?})", i, prev.len() - 1, now.elapsed());
                solve("AA", &valves, &distances, 26, &mut cache, opened, *s)
            }).max().unwrap()
        }
    }

    score
}

fn main() {
    println!("{}", parse());
}