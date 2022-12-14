use std::cmp::{max, min};

use regex::Regex;

type Pos = (usize, usize);

#[derive(Debug, Clone, Eq, PartialEq)]
enum Tile {
    ROCK,
    SAND,
    AIR,
}

struct Cave {
    chart: Vec<Vec<Tile>>,
    active: Option<Pos>,
    source: Pos,
}

impl Cave {
    fn draw(&self) {
        let mut drawing: String = String::from("");
        for y in 0..self.height() {
            let row = &self.chart[y];
            for x in 0..self.width() {
                let tile = &row[x];
                drawing.push(match tile {
                    Tile::ROCK => '#',
                    Tile::SAND => 'o',
                    Tile::AIR if (x, y) == self.source => '+',
                    _ => ' ',
                });
            }
            drawing.push('\n');
        }

        println!("{}", drawing);
    }

    fn width(&self) -> usize { self.chart.iter().map(|row| row.len()).max().unwrap() }
    fn height(&self) -> usize { self.chart.iter().len() }

    fn step(&mut self) {
        if let Some((x, y)) = self.active {
            self.active = None;

            if x == self.width() - 1 {
                let height = self.height();
                for i in 0..height {
                    let mut row = &mut self.chart[i];
                    if i == height - 1 {
                        row.push(Tile::ROCK);
                    } else {
                        row.push(Tile::AIR);
                    }
                }
            }

            if y == self.height() - 1 {
                self.chart[y][x] = Tile::AIR;
            } else {
                let candidates = [x, x - 1, x + 1];
                for candidate in candidates {
                    if self.chart[y + 1][candidate] == Tile::AIR {
                        self.chart[y][x] = Tile::AIR;
                        self.chart[y + 1][candidate] = Tile::SAND;
                        self.active = Some((candidate, y + 1));
                        break;
                    }
                }
            }
        } else if self.chart[self.source.1][self.source.0] == Tile::AIR {
            self.chart[self.source.1][self.source.0] = Tile::SAND;
            self.active = Some(self.source);
        }
    }

    fn find_iter<'a>(&'a self, tile: &'a Tile) -> impl Iterator<Item=Pos> + '_ {
        self.chart.iter().flatten().enumerate().filter(move |(_, t)| *t == tile).map(|(idx, _)| (idx % self.width(), idx / self.width()))
    }
}

fn build_cave() -> Cave {
    let input = aoc::io::get_input(14);
    let mut rocks = Vec::new();
    for line in input.lines() {
        line.split(" -> ")
            .map(|coordinates| {
                let (x, y) = coordinates.split_once(",").unwrap();
                (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
            })
            .reduce(|(x1, y1), (x2, y2)| {
                if x1 == x2 {
                    for y in min(y1, y2)..(max(y1, y2) + 1) {
                        rocks.push((x1, y));
                    }
                } else if y1 == y2 {
                    for x in min(x1, x2)..(max(x1, x2) + 1) {
                        rocks.push((x, y1));
                    }
                }
                (x2, y2)
            });
    }

    let height = *rocks.iter().map(|(_, y)| y).max().unwrap() + 1;
    let width = *rocks.iter().map(|(x, _)| x).max().unwrap() + 1;

    let mut chart = vec![vec![Tile::AIR; width]; height];
    for (x, y) in rocks {
        chart[y][x] = Tile::ROCK;
    }

    chart.push(vec![Tile::AIR; width]);
    chart.push(vec![Tile::ROCK; width]);

    Cave { chart, active: None, source: (500, 0) }
}

fn part1() -> usize {
    let mut cave = build_cave();
    while cave.active == None || cave.active.unwrap().1 < cave.height() - 2 {
        cave.step();
    }
    cave.find_iter(&Tile::SAND).count() - 1
}

fn part2() -> usize {
    let mut cave = build_cave();
    let mut prev = cave.active;
    cave.step();
    while prev != cave.active {
        prev = cave.active;
        cave.step();
    }

    cave.draw();
    cave.find_iter(&Tile::SAND).count()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}