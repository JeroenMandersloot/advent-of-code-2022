use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Pointer};
use std::ops::{Index, Rem};

use regex::Regex;

type Pos = (i32, i32);

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
enum Facing {
    RIGHT,
    DOWN,
    LEFT,
    UP,
}

impl Facing {
    fn all() -> [Facing; 4] {
        [Facing::RIGHT, Facing::DOWN, Facing::LEFT, Facing::UP]
    }

    fn turn_right(&self) -> Self {
        let all = Facing::all();
        all[(all.iter().position(|f| f == self).unwrap() + 1) % 4]
    }

    fn turn_left(&self) -> Self {
        let all = Facing::all();
        all[(all.iter().position(|f| f == self).unwrap() + 3) % 4]
    }

    fn turn_around(&self) -> Self {
        let all = Facing::all();
        all[(all.iter().position(|f| f == self).unwrap() + 2) % 4]
    }
}

fn step(pos: Pos, facing: Facing) -> Pos {
    let (x, y) = pos;
    match facing {
        Facing::RIGHT => (x + 1, y),
        Facing::DOWN => (x, y + 1),
        Facing::LEFT => (x - 1, y),
        Facing::UP => (x, y - 1),
    }
}

// fn step_wrap(pos: Pos, facing: Facing, width: i32, height: i32) -> Pos {
//     let (x, y) = step(pos, facing);
//     (x.rem_euclid(width), y.rem_euclid(height))
// }
//
// fn step_safe(pos: Pos, facing: Facing, width: i32, height: i32) -> Option<Pos> {
//     let (x, y) = step(pos, facing);
//     if x < 0 || x >= width || y < 0 || y >= height {
//         None
//     } else {
//         Some((x, y))
//     }
// }

#[derive(Debug, Clone, PartialEq)]
struct Side {
    layout: String,
}

impl Side {
    fn dim(&self) -> usize {
        (self.layout.len() as f64).sqrt() as usize
    }

    fn get(&self, pos: Pos) -> Option<char> {
        let (x, y) = pos;
        let idx = y * (self.dim() as i32) + x;
        self.layout.chars().nth(idx as usize)
    }
}

impl Display for Side {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res = self.layout.chars().collect::<Vec<_>>().chunks(4).map(|c| c.iter().collect::<String>()).collect::<Vec<_>>().join("\n");
        write!(f, "{}", res)
    }
}

#[derive(Debug)]
struct Cube {
    sides: HashMap<(i32, i32), Side>,
}

impl Cube {
    fn width(&self) -> i32 {
        *self.sides.keys().map(|(x, _)| x).max().unwrap() + 1
    }

    fn height(&self) -> i32 {
        *self.sides.keys().map(|(_, y)| y).max().unwrap() + 1
    }

    fn find(&self, side: &Side) -> Option<Pos> {
        self.sides.iter().filter(|(_, s)| side == *s).map(|(&pos, _)| pos).next()
    }

    fn get_adjacent_side(&self, pos: Pos, facing: Facing, cache: &mut HashMap<(Pos, Facing), Pos>, path: &mut Vec<Pos>) -> Option<Pos> {
        let cache_key = (pos, facing);
        if cache.contains_key(&cache_key) {
            return Some(*cache.get(&cache_key).unwrap());
        }

        let candidate = step(pos, facing);
        if self.sides.contains_key(&candidate) {
            cache.insert(cache_key, candidate);
            return Some(candidate);
        }

        path.push(pos);

        for f in [facing.turn_left(), facing.turn_right()] {
            let a = step(pos, f);
            if self.sides.contains_key(&a) && !path.contains(&a) {
                if let Some(res) = self.get_adjacent_side(a, facing, cache, path) {
                    path.pop();
                    return Some(res);
                }
            }
        }

        let opposite = step(pos, facing.turn_around());
        if self.sides.contains_key(&opposite) && !path.contains(&opposite) {
            for facing in [facing.turn_left(), facing.turn_right(), facing.turn_around()] {
                let a = step(opposite, facing);
                if self.sides.contains_key(&a) && !path.contains(&a) {
                    if let Some(res) = self.get_adjacent_side(a, facing, cache, path) {
                        path.pop();
                        return Some(res);
                    }
                }
            }
        }
        return None
    }

    fn get_adjacency_matrix(&self) -> HashMap<Pos, Vec<Pos>> {
        let mut cache = HashMap::new();
        let mut res = HashMap::new();
        for pos in self.sides.keys() {
            res.insert(*pos, Facing::all().iter().map(|&facing| {
                let mut path = Vec::new();
                self.get_adjacent_side(*pos, facing, &mut cache, &mut path).unwrap()
            }).collect::<Vec<_>>());
        }
        res
    }

    // fn get_neighbour_safe(&self, pos: Pos, facing: Facing, visited: &mut HashSet<(Pos, Facing)>) -> Option<(Pos, Facing)> {
    //     let cache_key = (pos, facing);
    //     if visited.contains(&cache_key) {
    //         return None;
    //     }
    //     let width = self.width();
    //     let height = self.height();
    //     if let Some(new) = step_safe(pos, facing, width, height) {
    //         println!("{:?} -> {:?} facing {:?} ({:?})", pos, new, facing, self.sides.get(&new));
    //         if let Some(_) = self.sides.get(&new) {
    //             return Some((new, facing));
    //         }
    //     }
    //
    //     visited.insert(cache_key);
    //
    //     let mut v = visited.clone();
    //     if let Some((new, _)) = self.get_neighbour_safe(pos, facing.turn_left(), &mut v) {
    //         println!("Entered branch 1");
    //         if let Some((res, facing)) = self.get_neighbour_safe(new, facing, &mut v) {
    //             println!("Entered branch 1a");
    //             return Some((res, facing.turn_right()));
    //         }
    //     }
    //
    //     let mut v = visited.clone();
    //     if let Some((new, _)) = self.get_neighbour_safe(pos, facing.turn_right(), &mut v) {
    //         println!("Entered branch 2");
    //         if let Some((res, facing)) = self.get_neighbour_safe(new, facing, &mut v) {
    //             println!("Entered branch 2a");
    //             return Some((res, facing.turn_left()));
    //         }
    //     }
    //
    //     let mut v = visited.clone();
    //     if let Some((new, _)) = self.get_neighbour_safe(pos, facing.turn_around(), &mut v) {
    //         println!("Entered branch 3");
    //         for f in [facing.turn_left(), facing.turn_right()] {
    //             if let Some((new2, _)) = self.get_neighbour_safe(new, f, &mut v) {
    //                 if let Some((res, facing)) = self.get_neighbour_safe(new2, f, &mut v) {
    //                     return Some((res, facing.turn_around()));
    //                 }
    //             }
    //         }
    //     }
    //
    //     None
    // }
    //
    // fn get_neighbour(&self, pos: Pos, facing: Facing) -> (&Side, Facing) {
    //     let mut visited = HashSet::new();
    //     let (new, facing) = self.get_neighbour_safe(pos, facing, &mut visited).unwrap();
    //     (self.sides.get(&new).unwrap(), facing)
    // }
}


fn parse(input: &str, dim: usize) -> (Cube, Vec<&str>) {
    let height = input.lines().count() - 2;
    let width = input.lines().take(height).map(|line| line.len()).max().unwrap();

    let mut a = 0;
    let mut y = 0;
    let mut sides = HashMap::new();
    while y < height {
        let mut b = 0;
        let mut x = 0;
        while x < width {
            let layout = input.lines().skip(y).take(dim).map(|line| line.chars().skip(x).take(dim).collect::<String>()).collect::<Vec<_>>().join("");
            if layout.trim().len() > 0 {
                sides.insert((b as i32, a as i32), Side { layout });
            }
            b += 1;
            x = b * dim;
        }
        a += 1;
        y = a * dim;
    }

    let cube = Cube { sides };
    let mut adj = cube.get_adjacency_matrix();
    for pos in cube.sides.keys() {
        println!("{:?}", adj.get(pos).unwrap());
    }

    let instructions = Regex::new(r"(\d+|[RL])").unwrap().captures_iter(input.lines().last().unwrap()).map(|captures| captures.iter().next().unwrap().unwrap().as_str()).collect();
    (cube, instructions)
}

// fn draw(grid: &Grid) -> String {
//     let width = grid.keys().map(|(x, _)| *x).max().unwrap();
//     let height = grid.keys().map(|(_, y)| *y).max().unwrap();
//     let mut drawing = String::from("");
//     for y in 0..height {
//         for x in 0..width {
//             drawing.push(match grid.get(&(x, y)).unwrap() {
//                 None => ' ',
//                 Some(true) => '#',
//                 Some(false) => '.',
//             });
//         }
//         drawing.push('\n');
//     }
//     drawing
// }

fn main() {
    let input = aoc::io::get_example(22);
    parse(&input, 4);
    // println!("{:?}", part1(&input));
    // println!("{}", part2(&input, is_example));
}