use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use regex::Regex;

fn parse(target: i32) {
    let mut a = HashSet::new();
    let pattern = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    let input = aoc::io::get_input(15);
    for line in input.lines() {
        if let [xs, ys, xb, yb] = pattern.captures_iter(line).next().unwrap().iter().skip(1).map(|m| m.unwrap().as_str().parse::<i32>().unwrap()).collect::<Vec<_>>()[..] {
            let distance = (xs - xb).abs() + (ys - yb).abs();
            let remainder = distance - (ys - target).abs();
            if remainder >= 0 {
                for d in 0..(remainder + 1) {
                    for m in [-d, d] {
                        let c = (xs + m, target);
                        if c != (xb, yb) {
                            a.insert(c);
                        }
                    }
                }
            }
        }
    }

    println!("{}", a.len());
}

fn merge_ranges(ranges: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    if !ranges.is_empty() {
        let mut a = ranges[0];
        for i in 1..ranges.len() {
            let (s, t) = a;
            let (v, w) = ranges[i];
            if (v == 0 || t >= v - 1) && s <= w + 1 {
                a = (min(s, v), max(t, w));
            } else {
                res.push(a);
                a = (v, w);
            }
        }
        res.push(a);
    }
    res
}

fn insert_range(range: (usize, usize), cache: &mut Vec<(usize, usize)>) {
    let (s, t) = range;
    let mut i = 0;
    while i < cache.len() {
        let (v, w) = cache[i];
        if s < v || (s == v && t < w) {
            break;
        }
        i += 1;
    }
    cache.insert(i,range);
}

fn parse2(limit: usize) {
    let mut yc = HashMap::new();
    for l in 0..(limit + 1) {
        yc.insert(l, Vec::new());
    }

    let pattern = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    let input = aoc::io::get_input(15);
    let mut i = 0;
    for line in input.lines() {
        i += 1;
        println!("Line {}", i);
        if let [xs, ys, xb, yb] = pattern.captures_iter(line).next().unwrap().iter().skip(1).map(|m| m.unwrap().as_str().parse::<i32>().unwrap()).collect::<Vec<_>>()[..] {
            let distance = (xs - xb).abs() + (ys - yb).abs();
            for yd in -distance..(distance + 1) {
                let dd = distance - yd.abs();
                let range = (max(0, xs - dd) as usize, min(limit as i32, xs + dd) as usize);
                if range.0 >= 0 && range.0 <= limit && range.1 >= 0 && range.1 <= limit && range.0 != range.1 {
                    let idx = min(limit as i32, max(0, ys + yd)) as usize;
                    let mut vec = yc.get_mut(&idx).unwrap();
                    insert_range(range, vec);
                }
            }
        }
    }

    for i in 0..(limit + 1) {
        let ranges = merge_ranges(yc.get(&i).unwrap());
        if ranges.len() == 2 {
            println!("({}, {})", ranges[0].1 + 1, i);
        }
    }


    // yc.iter().map(|(idx, vec)| (idx, merge_ranges(&vec))).collect::<HashMap<_, _>>();
    // let ycc = yc.iter().map(|(idx, vec)| (idx, merge_ranges(&vec))).collect::<HashMap<_, _>>();
    // for i in 0..(limit + 1) {
    //     println!("{}", i);
    //     println!("  {:?}", yc.get(&i).unwrap());
    //     println!("  {:?}", ycc.get(&i).unwrap());
    // }
}

fn main() {
    // parse(2000000);
    parse2(4000000);

    // let mut a = Vec::new();
    // insert_range((1, 10), &mut a);
    // insert_range((20, 50), &mut a);
    // insert_range((5, 19), &mut a);
    // insert_range((30, 90), &mut a);
    // println!("{:?}", a);
    // println!("{:?}", merge_ranges(&a));
}

// 3157535 * 4000000 + 3363767 = 12630143000000
//                               12630143363767