use std::collections::HashMap;

use regex::Regex;

type Pos = (i32, i32);
type Adjacency = HashMap<(Pos, Facing), (Pos, Facing)>;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
enum Facing { RIGHT, DOWN, LEFT, UP }

impl Facing {
    fn all() -> [Facing; 4] { [Facing::RIGHT, Facing::DOWN, Facing::LEFT, Facing::UP] }
    fn turn_right(&self) -> Self { self.modulo(1) }
    fn turn_around(&self) -> Self { self.modulo(2) }
    fn turn_left(&self) -> Self { self.modulo(3) }
    fn modulo(&self, modulo: usize) -> Self {
        let all = Facing::all();
        all[(all.iter().position(|f| f == self).unwrap() + modulo) % 4]
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
        let side_x = x / dim;
        let side_y = y / dim;
        let nx = x.rem_euclid(dim);
        let ny = y.rem_euclid(dim);
        let (new_pos, new_facing) = match step_safe((nx, ny), facing, self.dim(), self.dim()) {
            Some(_) => (step(pos, facing), facing),
            None => {
                let d = dim - 1;  // Convenience variable.
                let ((new_side_x, new_side_y), f) = *adj.get(&((side_x, side_y), facing)).unwrap();
                let (new_nx, new_ny) = match (facing, f) {
                    (Facing::RIGHT, Facing::RIGHT) => (0, ny),
                    (Facing::RIGHT, Facing::DOWN) => (d - ny, 0),
                    (Facing::RIGHT, Facing::LEFT) => (d, d - ny),
                    (Facing::RIGHT, Facing::UP) => (ny, d),
                    (Facing::DOWN, Facing::RIGHT) => (0, d - nx),
                    (Facing::DOWN, Facing::DOWN) => (nx, 0),
                    (Facing::DOWN, Facing::LEFT) => (d, nx),
                    (Facing::DOWN, Facing::UP) => (d - nx, d),
                    (Facing::LEFT, Facing::RIGHT) => (0, d - ny),
                    (Facing::LEFT, Facing::DOWN) => (ny, 0),
                    (Facing::LEFT, Facing::LEFT) => (d, ny),
                    (Facing::LEFT, Facing::UP) => (d - ny, d),
                    (Facing::UP, Facing::RIGHT) => (0, nx),
                    (Facing::UP, Facing::DOWN) => (d - nx, 0),
                    (Facing::UP, Facing::LEFT) => (d, nx),
                    (Facing::UP, Facing::UP) => (nx, d),
                };

                ((new_side_x * dim + new_nx, new_side_y * dim + new_ny), f)
            }
        };

        if self.get(new_pos).unwrap() == '.' {
            (new_pos, new_facing)
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

        for rotation in [Facing::turn_left, Facing::turn_right] {
            let other = step(pos, rotation(&facing));
            if self.sides.contains_key(&other) && !path.contains(&other) {
                if let Some((target, facing)) = self.get_adjacent_side(other, facing, path) {
                    path.pop();
                    return Some((target, rotation(&facing)));
                }
            }
        }

        let other = step(pos, facing.turn_around());
        if self.sides.contains_key(&other) && !path.contains(&other) {
            for rotation in [Facing::turn_left, Facing::turn_right, Facing::turn_around] {
                let opposite = step(other, rotation(&facing));
                if self.sides.contains_key(&opposite) && !path.contains(&opposite) {
                    if let Some((target, facing)) = self.get_adjacent_side(opposite, rotation(&facing), path) {
                        path.pop();
                        return Some((target, rotation(&facing)));
                    }
                }
            }
        }

        return None;
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

fn parse(input: &str, dim: usize) -> (Cube, Vec<&str>) {
    let height = input.lines().count() - 2;
    let width = input.lines().take(height).map(|line| line.len()).max().unwrap();
    let mut y = 0;
    let mut sides = HashMap::new();
    while y < height {
        let mut x = 0;
        while x < width {
            let layout = input.lines().skip(y).take(dim).map(|line| line.chars().skip(x).take(dim).collect::<String>()).collect::<Vec<_>>().join("");
            if layout.trim().len() > 0 {
                sides.insert(((x / dim) as i32, (y / dim) as i32), layout);
            }
            x += dim;
        }
        y += dim;
    }

    let cube = Cube { sides };
    let instructions = Regex::new(r"(\d+|[RL])").unwrap().captures_iter(input.lines().last().unwrap()).map(|captures| captures.iter().next().unwrap().unwrap().as_str()).collect();
    (cube, instructions)
}

fn part2(cube: &Cube, instructions: &Vec<&str>) -> i32 {
    let adj = cube.get_adjacency_matrix();
    let mut pos = (cube.dim() * cube.sides.keys().filter(|(_, y)| *y == 0).map(|(x, _)| *x).min().unwrap(), 0);
    let mut facing = Facing::RIGHT;
    for instruction in instructions {
        match instruction.parse::<i32>() {
            Ok(num_steps) => {
                for _ in 0..num_steps {
                    let (new_pos, new_facing) = cube.step(pos, facing, &adj);
                    if pos == new_pos { break; }
                    pos = new_pos;
                    facing = new_facing;
                }
            }
            Err(_) => {
                facing = if *instruction == "R" { facing.turn_right() } else { facing.turn_left() };
            }
        }
    }

    let facing_idx = Facing::all().iter().position(|&f| f == facing).unwrap() as i32;
    let (x, y) = pos;
    1000 * (y + 1) + 4 * (x + 1) + facing_idx
}

fn main() {
    let input = aoc::io::get_input(22);
    let (cube, instructions) = parse(&input, 50);
    println!("{}", part2(&cube, &instructions));
}