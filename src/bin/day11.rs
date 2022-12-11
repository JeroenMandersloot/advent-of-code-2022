use std::cell::RefCell;
use std::collections::VecDeque;
use regex::{Regex, RegexBuilder};

enum Operation {
    MULTIPLY(u32),
    ADD(u32),
    SQUARE,
}

impl Operation {
    fn apply(&self, worry: u32) -> u32 {
        match self {
            Operation::MULTIPLY(o) => worry * o,
            Operation::ADD(o) => worry + o,
            Operation::SQUARE => worry * worry,
        }
    }
}

impl From<&str> for Operation {
    fn from(a: &str) -> Self {
        let (operator, operand) = a[4..].split_once(" ").unwrap();
        match operand.parse() {
            Ok(o) if operator == "*" => Operation::MULTIPLY(o),
            Ok(o) if operator == "+" => Operation::ADD(o),
            Err(_) => Operation::SQUARE,
            _ => panic!("Invalid operation: {}", a),
        }
    }
}

struct Monkey {
    id: usize,
    items: VecDeque<u32>,
    operation: Operation,
    test: u32,
    y: usize,
    n: usize,
}

static PATTERN: &str = r"Monkey (\d+):
  Starting items: (.+)
  Operation: new = (.+)
  Test: divisible by (\d+)
    If true: throw to monkey (\d+)
    If false: throw to monkey (\d+)";

fn get_monkeys() -> Vec<Monkey> {
    let input = aoc::io::get_input(11);
    let pattern = RegexBuilder::new(PATTERN.trim())
        .multi_line(true)
        .build()
        .unwrap();
    pattern
        .captures_iter(&input)
        .map(|monkey| {
            let mut vars = monkey.iter().map(|c| c.unwrap().as_str()).skip(1);
            Monkey {
                id: vars.next().unwrap().parse().unwrap(),
                items: vars.next().unwrap().split(", ").map(|item| item.parse().unwrap()).collect(),
                operation: Operation::from(vars.next().unwrap()),
                test: vars.next().unwrap().parse().unwrap(),
                y: vars.next().unwrap().parse().unwrap(),
                n: vars.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

fn simulate_round(monkeys: &mut Vec<Monkey>, counter: &mut Vec<usize>) {
    let mut aapjes: Vec<_> = monkeys.iter_mut().map(RefCell::new).collect();
    for i in 0..aapjes.len() {
        let mut monkey = aapjes[i].borrow_mut();
        // println!("Monkey {}", monkey.id);
        while !monkey.items.is_empty() {
            counter[i] += 1;
            let item = monkey.items.pop_front().unwrap();
            // println!("  Monkey inspects an item with a worry level of {}", &item);
            let new_item = monkey.operation.apply(item) / 3;
            // println!("    Worry level updated to {}", &new_item);
            let recipient_id = if new_item % monkey.test == 0 { monkey.y } else { monkey.n };
            // println!("    Item thrown to monkey {}", &recipient_id);
            let mut recipient = aapjes[recipient_id].borrow_mut();
            recipient.items.push_back(new_item);
        }
    }
}

fn part1() -> usize {
    let mut monkeys = get_monkeys();
    let mut counter = vec![0; monkeys.len()];
    for i in 0..20 {
        simulate_round(&mut monkeys, &mut counter);
        println!("{}", i);
        dbg!(&counter);
    }
    counter.sort();
    counter.reverse();
    println!("{}", counter[..2].iter().product::<usize>());
    monkeys.len()
}

fn main() {
    println!("{}", part1());  // 67830
}