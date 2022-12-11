use std::cell::RefCell;

use regex::{Captures, Regex};

enum Operation {
    MULTIPLY(u64),
    ADD(u64),
    SQUARE,
}

impl Operation {
    fn apply(&self, worry: u64) -> u64 {
        match self {
            Operation::MULTIPLY(o) => worry * o,
            Operation::ADD(o) => worry + o,
            Operation::SQUARE => worry * worry,
        }
    }

    fn from(operation: &str) -> Self {
        let (operator, operand) = operation.split_once(" ").unwrap();
        match operand.parse() {
            Ok(o) if operator == "*" => Operation::MULTIPLY(o),
            Ok(o) if operator == "+" => Operation::ADD(o),
            Err(_) => Operation::SQUARE,
            _ => panic!("Invalid operation: {}", operation),
        }
    }
}

struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    recipient1: usize,
    recipient2: usize,
}

static PATTERN: &str = r"Monkey \d+:
  Starting items: ((?:\d+(?:, )?)+)
  Operation: new = old ([+*] (?:old|\d+))
  Test: divisible by (\d+)
    If true: throw to monkey (\d+)
    If false: throw to monkey (\d+)";

fn create_monkey_from_captures(captures: Captures) -> Monkey {
    let mut vars = captures.iter().map(|c| c.unwrap().as_str()).skip(1);
    Monkey {
        items: vars.next().unwrap().split(", ").map(|item| item.parse().unwrap()).collect(),
        operation: Operation::from(vars.next().unwrap()),
        test: vars.next().unwrap().parse().unwrap(),
        recipient1: vars.next().unwrap().parse().unwrap(),
        recipient2: vars.next().unwrap().parse().unwrap(),
    }
}

fn get_monkeys() -> Vec<Monkey> {
    let input = aoc::io::get_input(11);
    Regex::new(PATTERN)
        .unwrap()
        .captures_iter(&input)
        .map(create_monkey_from_captures)
        .collect()
}

fn simulate_round(monkeys: &mut Vec<Monkey>, counter: &mut Vec<usize>, relief: u64) {
    let lcm: u64 = monkeys.iter().map(|m| m.test).product();

    // Shadow ``monkeys`` by wrapping every monkey in a ``RefCell`` so that we
    // check for multiple mutable references only at runtime rather than compile
    // time. The compiler cannot know that we will never create two mutable
    // references (1 for the current monkey, 1 for the recipient) that reference
    // the same monkey, but we know the current monkey can never also be the
    // recipient. With ``RefCell.borrow_mut()`` we can make this check at
    // runtime, which allows our code to compile.
    let monkeys: Vec<_> = monkeys.iter_mut().map(RefCell::new).collect();
    for (mut monkey, count) in monkeys.iter().map(RefCell::borrow_mut).zip(counter) {
        while !monkey.items.is_empty() {
            *count += 1;
            let item = monkey.items.pop().unwrap();
            let new_item = monkey.operation.apply(item % lcm) / relief;
            let recipient_id = if new_item % monkey.test == 0 {
                monkey.recipient1
            } else {
                monkey.recipient2
            };
            monkeys[recipient_id].borrow_mut().items.push(new_item);
        }
    }
}

fn solve(num_rounds: usize, relief: u64) -> usize {
    let mut monkeys = get_monkeys();
    let mut counter = vec![0; monkeys.len()];
    for _ in 0..num_rounds {
        simulate_round(&mut monkeys, &mut counter, relief);
    }
    counter.sort();
    counter.reverse();
    counter[..2].iter().product::<usize>()
}

fn main() {
    println!("{}", solve(20, 3));  // 67830
    println!("{}", solve(10_000, 1));  // 15305381442
}