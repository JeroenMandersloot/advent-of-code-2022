use std::collections::HashSet;

fn parse(input: String) -> HashSet<(i32, i32, i32)> {
    let mut grid = HashSet::new();
    for line in input.lines() {
        if let [x, y, z] = line.split(",").map(|c| c.parse::<i32>().unwrap()).collect::<Vec<_>>()[..] {
            grid.insert((x + 1, y + 1, z + 1));
        }
    }
    grid
}

fn get_neighbours(point: (i32, i32, i32)) -> [(i32, i32, i32); 6] {
    let (x, y, z) = point;
    [(x - 1, y, z), (x + 1, y, z), (x, y - 1, z), (x, y + 1, z), (x, y, z - 1), (x, y, z + 1)]
}

fn part1(grid: &HashSet<(i32, i32, i32)>) -> usize {
    grid.iter().map(|p| {
        let candidates = get_neighbours(*p);
        candidates.into_iter().filter(|c| !grid.contains(c)).count()
    }).sum()
}

fn part2(grid: &HashSet<(i32, i32, i32)>) -> usize {
    let width = *grid.iter().map(|(x, _, _)| x).max().unwrap() + 1;
    let depth = *grid.iter().map(|(_, y, _)| y).max().unwrap() + 1;
    let height = *grid.iter().map(|(_, _, z)| z).max().unwrap() + 1;
    let mut stack = Vec::from([(width, depth, height)]);
    let mut seen = HashSet::new();
    seen.insert(stack[0]);
    let mut surface = 0;
    while !stack.is_empty() {
        let current = stack.pop().unwrap();
        let neighbours = get_neighbours(current);
        for neighbour in neighbours {
            let (x, y, z) = neighbour;
            if x >= 0 && y >= 0 && z >= 0 && x <= width && y <= depth && z <= height {
                if grid.contains(&neighbour) {
                    surface += 1;
                } else if !seen.contains(&neighbour) {
                    stack.push(neighbour);
                    seen.insert(neighbour);
                }
            }
        }
    }
    surface
}

fn main() {
    let input = aoc::io::get_input(18);
    let grid = parse(input);
    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
}