use std::collections::{HashMap, HashSet};
use regex::Regex;

#[derive(Clone, Debug)]
enum Job {
    NUMBER(i64),
    OPERATION(String),
}

fn get_or_compute(monkey: &str, cache: &mut HashMap<String, Job>) -> i64 {
    let job = cache.get(monkey).unwrap().clone();
    match job {
        Job::NUMBER(number) => number,
        Job::OPERATION(op) => {
            if let [a, operator, b] = op.split(" ").collect::<Vec<_>>()[..] {
                let a = get_or_compute(a, cache);
                let b = get_or_compute(b, cache);
                let res = match operator {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,  // TODO: cast to float?
                    _ => panic!("Invalid operator")
                };
                *cache.get_mut(monkey).unwrap() = Job::NUMBER(res);
                res
            } else { panic!("Invalid operation") }
        }
    }
}

fn parse(input: &str) -> HashMap<String, Job> {
    let mut res = HashMap::new();
    for line in input.lines() {
        let (monkey, job) = line.split_once(": ").unwrap();
        let job = match job.parse::<i64>() {
            Ok(number) => Job::NUMBER(number),
            Err(_) => Job::OPERATION(String::from(job))
        };
        res.insert(String::from(monkey), job);
    }
    res
}

fn part1(input: &str) -> i64 {
    let mut cache = parse(&input);
    get_or_compute("root", &mut cache)
}

fn part2(input: &str, is_example: bool) -> i64 {
    // For the example, we need to do the binary search in the opposite direction.
    let correction = if is_example { -1 } else { 1 };
    let operands = Regex::new(r"root: (.+) [*/+-] (.+)").unwrap().captures_iter(input).next().unwrap().iter().skip(1).flatten().map(|m| m.as_str()).collect::<Vec<_>>();
    let a = operands[0];
    let b = operands[1];
    let mut min = Regex::new(r"humn: (\d+)").unwrap().captures_iter(input).next().unwrap().iter().skip(1).next().unwrap().unwrap().as_str().parse::<i64>().unwrap();
    let mut max = 3500000000000i64;
    let mut solutions = HashSet::new();
    loop {
        let current = (min + max) / 2;
        let mut cache = parse(&input);
        let human = cache.get_mut("humn").unwrap();
        *human = Job::NUMBER(current);
        let answer_a = get_or_compute(&a, &mut cache);
        let answer_b = get_or_compute(&b, &mut cache);
        if answer_a == answer_b {
            solutions.insert(current);
            max -= 1;
        } else if correction * answer_a < correction * answer_b {
            max = current;
        } else if correction * answer_a > correction * answer_b {
            min = current;
        }
        if max - min <= 1 {  // Convergence reached.
            break;
        }
    }

    // There are actually multiple correct solutions, and apparently we are
    // supposed to submit the smallest one.
    *solutions.iter().min().unwrap()
}

fn main() {
    let is_example = false;
    let input = if is_example { aoc::io::get_example(21) } else { aoc::io::get_input(21) };
    println!("{}", part1(&input));
    println!("{}", part2(&input, is_example));
}