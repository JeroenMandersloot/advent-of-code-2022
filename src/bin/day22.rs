use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Pointer};
use std::ops::{Index, Rem};

use regex::Regex;

type Pos = (i32, i32);
type Adjacency = HashMap<(Pos, Facing), (Pos, Facing)>;

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

fn step_safe(pos: Pos, facing: Facing, width: i32, height: i32) -> Option<Pos> {
    let (x, y) = step(pos, facing);
    if x < 0 || x >= width || y < 0 || y >= height {
        None
    } else {
        Some((x, y))
    }
}


#[derive(Debug)]
struct Cube {
    sides: HashMap<(i32, i32), String>,
}

impl Cube {
    fn dim(&self) -> i32 {
        (self.sides.values().next().unwrap().len() as f32).sqrt() as i32
    }

    fn get(&self, pos: Pos) -> Option<char> {
        let (x, y) = pos;
        let dim = self.dim();
        let sx = x / dim;
        let sy = y / dim;
        let idx = y.rem_euclid(dim) * dim + x.rem_euclid(dim);
        self.sides.get(&(sx, sy)).unwrap().chars().nth(idx as usize)
    }

    fn step(&self, pos: Pos, facing: Facing, adj: &Adjacency) -> (Pos, Facing) {
        let (x, y) = pos;
        let dim = self.dim();
        let sx = x / dim;
        let sy = y / dim;
        let nx = x.rem_euclid(dim);
        let ny = y.rem_euclid(dim);
        let (candidate, f) = match step_safe((nx, ny), facing, self.dim(), self.dim()) {
            Some(_) => (step(pos, facing), facing),
            None => {
                let ((nsx, nsy), f) = *adj.get(&((sx, sy), facing)).unwrap();
                let d = dim - 1;
                let (nnx, nny) = match (facing, f) {
                    (Facing::RIGHT, Facing::RIGHT) => (0, ny),      
                    (Facing::RIGHT, Facing::DOWN)  => (d - ny, 0),  
                    (Facing::RIGHT, Facing::LEFT)  => (d, d - ny),  
                    (Facing::RIGHT, Facing::UP)    => (ny, d),      // Checked
                    (Facing::DOWN, Facing::RIGHT)  => (0, d - nx),  
                    (Facing::DOWN, Facing::DOWN)   => (nx, 0),      
                    (Facing::DOWN, Facing::LEFT)   => (d, nx),      // Checked
                    (Facing::DOWN, Facing::UP)     => (d - nx, d),  
                    (Facing::LEFT, Facing::RIGHT)  => (0, d - ny),  
                    (Facing::LEFT, Facing::DOWN)   => (ny, 0),      // Checked
                    (Facing::LEFT, Facing::LEFT)   => (d, ny),      
                    (Facing::LEFT, Facing::UP)     => (d - ny, d),  
                    (Facing::UP, Facing::RIGHT)    => (0, nx),      
                    (Facing::UP, Facing::DOWN)     => (d - nx, 0),  
                    (Facing::UP, Facing::LEFT)     => (d, nx),      
                    (Facing::UP, Facing::UP)       => (nx, d),      
                };

                ((nsx * dim + nnx, nsy * dim + nny), f)
            }
        };

        if self.get(candidate).unwrap() == '.' {
            (candidate, f)
        } else {
            (pos, facing)
        }
    }

    fn get_adjacent_side(&self, pos: Pos, facing: Facing, path: &mut Vec<Pos>) -> Option<(Pos, Facing)> {
        let candidate = step(pos, facing);
        if self.sides.contains_key(&candidate) {
            return Some((candidate, facing));
        }

        path.push(pos);

        for (i, &f) in [facing.turn_left(), facing.turn_right()].iter().enumerate() {
            let a = step(pos, f);
            if self.sides.contains_key(&a) && !path.contains(&a) {
                if let Some((b, ff)) = self.get_adjacent_side(a, facing, path) {
                    path.pop();
                    return Some((b, if i == 0 { ff.turn_left() } else { ff.turn_right() }));
                }
            }
        }

        let opposite = step(pos, facing.turn_around());
        if self.sides.contains_key(&opposite) && !path.contains(&opposite) {
            for (i, &facing) in [facing.turn_left(), facing.turn_right(), facing.turn_around()].iter().enumerate() {
                let a = step(opposite, facing);
                if self.sides.contains_key(&a) && !path.contains(&a) {
                    if let Some((b, ff)) = self.get_adjacent_side(a, facing, path) {
                        path.pop();
                        let ff = match i {
                            0 => ff.turn_left(),
                            1 => ff.turn_right(),
                            2 => ff.turn_around(),
                            _ => panic!()
                        };
                        return Some((b, ff));
                    }
                }
            }
        }
        return None
    }

    fn get_adjacency_matrix(&self) -> Adjacency {
        let mut res = HashMap::new();
        for pos in self.sides.keys() {
            for facing in Facing::all() {
                let mut path = Vec::new();
                res.insert((*pos, facing), self.get_adjacent_side(*pos, facing, &mut path).unwrap());
            }
        }
        res
    }
}

// impl Display for Cube {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let res = self.layout.chars().collect::<Vec<_>>().chunks(4).map(|c| c.iter().collect::<String>()).collect::<Vec<_>>().join("\n");
//         write!(f, "{}", res)
//     }
// }


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
                sides.insert((b as i32, a as i32), layout);
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
        for facing in Facing::all() {
            println!("{:?} --{:?}-> {:?}", pos, facing, adj.get(&(*pos, facing)).unwrap());
        }
    }

    // println!("{:?} (should be LEFT)", adj.get(&((3, 2), Facing::RIGHT)).unwrap());
    // println!("{:?} (should be LEFT)", adj.get(&((3, 2), Facing::UP)).unwrap());
    // println!("{:?} (should be UP)", adj.get(&((2, 2), Facing::DOWN)).unwrap());
    // println!("{:?} (should be DOWN)", adj.get(&((2, 1), Facing::RIGHT)).unwrap());

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
    let input = aoc::io::get_input(22);
    let (cube, instructions) = parse(&input, 50);


    let adj = cube.get_adjacency_matrix();
    let mut curr_pos = (cube.dim() * *cube.sides.keys().filter(|(x, y)| *y == 0).map(|(x, y)| x).min().unwrap(), 0);
    let mut curr_facing = Facing::RIGHT;

    println!("{:?}", curr_pos);
    for instruction in instructions {
        match instruction.parse::<i32>() {
            Ok(num_steps) => {
                for _ in 0..num_steps {
                    let (a, b) = cube.step(curr_pos, curr_facing, &adj);
                    if curr_pos == a {
                        break;
                    }
                    curr_pos = a;
                    curr_facing = b;
                    println!("{:?} facing {:?}", curr_pos, curr_facing);
                }
            },
            Err(_) => {
                curr_facing = match instruction {
                    "R" => curr_facing.turn_right(),
                    "L" => curr_facing.turn_left(),
                    _ => panic!()
                };
            }
        }
    }

    let solution = 1000 * (curr_pos.1 + 1) + 4 * (curr_pos.0 + 1) + match curr_facing {
        Facing::RIGHT => 0,
        Facing::DOWN => 1,
        Facing::LEFT => 2,
        Facing::UP => 3,
    };

    println!("{solution}");


    // println!("{:?}", part1(&input));
    // println!("{}", part2(&input, is_example));
}