use std::cmp::Ordering;

#[derive(Clone, PartialEq, Eq, Ord)]
enum Packet {
    Integer(usize),
    List(Vec<Packet>),
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Integer(a), Packet::Integer(b)) => a.partial_cmp(b),
            (Packet::List(a), Packet::List(b)) => a.partial_cmp(b),
            (Packet::Integer(a), b) => Packet::List(vec![Packet::Integer(*a); 1]).partial_cmp(b),
            (a, Packet::Integer(b)) => a.partial_cmp(&Packet::List(vec![Packet::Integer(*b); 1]))
        }
    }
}

fn parse_packet(line: &str, offset: usize) -> (Packet, usize) {
    let mut i = offset;
    let mut buffer = String::from("");
    let mut res = Vec::new();
    while i < line.len() {
        match line.chars().nth(i).unwrap() {
            '[' => {
                let (packet, j) = parse_packet(line, i + 1);
                res.push(packet);
                i = j;
            }
            ']' => break,
            c if c.is_digit(10) => buffer.push(c),
            _ => {
                if ! buffer.is_empty() {
                    res.push(Packet::Integer(buffer.parse().unwrap()));
                    buffer.truncate(0);
                }
            }
        }

        i += 1;
    }

    if !buffer.is_empty() {
        res.push(Packet::Integer(buffer.parse().unwrap()));
        buffer.truncate(0);
    }

    (Packet::List(res), i)
}

fn get_packets() -> Vec<Packet> {
    let input = aoc::io::get_input(13);
    let mut packets = Vec::new();
    for line in input.lines().filter(|line| !line.trim().is_empty()) {
        let (packet, _) = parse_packet(line, 1);
        packets.push(packet);
    }

    packets
}

fn part1() -> usize {
    let packets = get_packets();
    let mut solution = 0;
    for (i, pair) in packets.iter().as_slice().chunks(2).enumerate() {
        if pair[0] < pair[1] {
            solution += i + 1;
        }
    }

    solution
}

fn part2() -> usize {
    let mut packets = get_packets();
    let (divider1, _) = parse_packet("[[2]]", 1);
    let (divider2, _) = parse_packet("[[6]]", 1);
    packets.push(divider1.clone());
    packets.push(divider2.clone());
    packets.sort();
    [&divider1, &divider2]
        .iter()
        .map(|divider| 1 + packets.iter().position(|packet| packet == *divider).unwrap())
        .product()
}


fn main() {
    println!("{}", part1());  // 6072
    println!("{}", part2());  // 22184
}