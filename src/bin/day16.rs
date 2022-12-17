use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::time::Instant;

use regex::Regex;

fn combinations<'a, T>(vec: &'a Vec<&T>) -> Vec<(&'a T, &'a T)> {
    let mut res = Vec::new();
    if !vec.is_empty() {
        for i in 0..(vec.len()-1) {
            for j in i..vec.len() {
                if i != j {
                    res.push((vec[i], vec[j]));
                }
            }
        }
    }
    res
}

fn calc(origin: &str, valves: &HashMap<&str, usize>, distances: &HashMap<&str, HashMap<&str, usize>>, minutes_remaining: usize) -> usize {
    if minutes_remaining <= 0 || valves.is_empty() {
        return 0;
    }
    valves.iter().map(|(valve, flow)| {
        let duration = distances.get(origin).unwrap().get(valve).unwrap() + 1;
        if duration > minutes_remaining {
            0
        } else {
            let minutes_remaining = minutes_remaining - duration;
            let mut valves = valves.clone();
            valves.remove(valve);
            if origin == "AA" {
                println!("AA -> {}: {} * {}", valve, minutes_remaining, flow)
            }
            flow * minutes_remaining + calc(valve, &valves, distances, minutes_remaining)
        }
    }).max().unwrap()
}

fn calc_duo<'a>(
    origin: (&str, &str),
    valves: &HashMap<&str, usize>,
    distances: &HashMap<&str, HashMap<&str, usize>>,
    minutes_remaining: (usize, usize),
    cache: &mut HashMap<((String, String), Vec<String>, (usize, usize)), usize>
) -> usize {
    let o_key = (String::from(origin.0), String::from(origin.1));
    let key = (o_key, valves.keys().map(|k| String::from(k.deref())).collect::<Vec<_>>(), minutes_remaining);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }
    if valves.is_empty() {
        return 0;
    }
    if valves.len() == 1 {
        return max(
            calc(origin.0, valves, distances, minutes_remaining.0),
            calc(origin.1, valves, distances, minutes_remaining.1)
        );
    }

    let now = Instant::now();

    // let mut valves = valves.clone();
    let nodes = valves.keys().map(|k| k.deref()).collect::<Vec<_>>();
    let mut a = 0;
    for i in 0..nodes.len() {
        for j in 0..nodes.len() {
            if i == j {
                continue;
            }
            if origin == ("AA", "AA") {
                println!("({}, {}) out of {} ({:?})", i, j, nodes.len(), now.elapsed());
            }
            let you = nodes[i];
            let elephant = nodes[j];
            let flow0 = valves.get(you).unwrap();
            let flow1 = valves.get(elephant).unwrap();
            let mut valves = valves.clone();
            valves.remove(you);
            valves.remove(elephant);
            let duration0 = distances.get(&origin.0).unwrap().get(you).unwrap() + 1;
            let duration1 = distances.get(&origin.1).unwrap().get(elephant).unwrap() + 1;
            let minutes_remaining_0 = if duration0 > minutes_remaining.0 { 0 } else { minutes_remaining.0 - duration0 };
            let minutes_remaining_1 = if duration1 > minutes_remaining.1 { 0 } else { minutes_remaining.1 - duration1 };
            let minutes_remaining = (minutes_remaining_0, minutes_remaining_1);
            let res = flow0 * minutes_remaining_0 +
                flow1 * minutes_remaining_1 +
                calc_duo((you, elephant), &valves, distances, minutes_remaining, cache);
            a = max(a, res);
        }
    }

    cache.insert(key, a);
    a
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
    let mut edges = HashMap::new();
    let mut valves = HashMap::new();
    let input = aoc::io::get_input(16);
    let pattern = Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? ((?:[A-Z]+(?:, )?)+)").unwrap();
    for line in input.lines() {
        let captures = pattern.captures_iter(line).next().unwrap();
        let mut matches = captures.iter().skip(1).map(|m| m.unwrap().as_str());
        let origin = matches.next().unwrap();
        let flow = matches.next().unwrap().parse::<usize>().unwrap();
        if flow > 0 {
            valves.insert(origin, flow);
        }
        edges.insert(origin, matches.next().unwrap().split(", ").collect::<Vec<_>>());
    }

    let distances = distance_matrix(&edges);
    // calc("AA", &valves, &distances, 30)

    let mut cache = HashMap::new();
    calc_duo(("AA", "AA"), &valves, &distances, (26, 26), &mut cache)
}

fn main() {
    println!("{}", parse());
}