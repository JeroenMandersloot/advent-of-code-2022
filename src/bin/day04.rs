// Q: Why do ``Range::min()`` and ``Range::max()`` act upon ``mut self`` instead of ``&self``?

struct Assignment(u32, u32);

impl Assignment {
    fn dominates(&self, other: &Assignment) -> bool {
        other.0 >= self.0 && other.1 <= self.1
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        (self.0 >= other.0 && self.0 <= other.1) ||
        (self.1 >= other.0 && self.1 <= other.1) ||
        (other.0 >= self.0 && other.0 <= self.1) ||
        (other.1 >= self.0 && other.1 <= self.1)
    }
}


fn parse_assignment(assignment: &str) -> Assignment {
    let (x, y) = assignment.split_once("-").unwrap();
    Assignment(x.parse().unwrap(), y.parse().unwrap())
}


fn part1() -> u32 {
    aoc::io::get_input(4)
        .split("\n")
        .map(|line| line.split_once(",").unwrap())
        .map(|(a, b)| {
            let s = parse_assignment(a);
            let t = parse_assignment(b);
            (s.dominates(&t) | t.dominates(&s)) as u32
        })
        .sum()
}


fn part2() -> u32 {
    aoc::io::get_input(4)
        .split("\n")
        .map(|line| line.split_once(",").unwrap())
        .map(|(a, b)|
            parse_assignment(a).overlaps(&parse_assignment(b)) as u32
        )
        .sum()
}


fn main() {
    println!("{}", part1());
    println!("{}", part2());
}