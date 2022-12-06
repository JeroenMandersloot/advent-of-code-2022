use std::collections::{HashSet, VecDeque};

fn find_marker(num_distinct: usize) -> Option<usize> {
    let input = aoc::io::get_input(6);
    let mut stream = input.chars();
    let mut buffer: VecDeque<_> = stream.by_ref().take(num_distinct).collect();
    for (i, c) in stream.enumerate() {
        let h: HashSet<&char> = HashSet::from_iter(&buffer);
        if h.len() == num_distinct {
            return Some(i + num_distinct);
        }
        buffer.push_back(c);
        buffer.pop_front();
    }
    return None;
}

fn main() {
    println!("{}", find_marker(4).unwrap());
    println!("{}", find_marker(14).unwrap());
}