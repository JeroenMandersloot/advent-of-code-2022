use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Index;

const LEVELS: &str = "abcdefghijklmnopqrstuvwxyz";


struct Graph {
    nodes: Vec<Node>,
    edges: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn find(&self, data: char) -> usize {
        self.nodes.iter().enumerate().filter(|(idx, node)| node.data == data).next().unwrap().0
    }

    fn find_iter(&self, data: char) -> impl Iterator<Item=usize> + '_ {
        self.nodes.iter().enumerate().filter(move |(idx, node)| node.data == data).map(|(idx, _)| idx)
    }

    fn compute_distances(&self, start: usize) -> Vec<usize> {
        let num_nodes = self.nodes.len();
        let mut distances = vec![std::usize::MAX; num_nodes];
        let mut previous: Vec<Option<usize>> = vec![None; num_nodes];
        let mut visited = HashSet::new();
        let mut queue: Vec<_> = (0..num_nodes).collect();
        distances[start] = 0;
        while visited.len() < num_nodes {
            let current = queue
                .iter()
                .filter(|idx| !visited.contains(idx))
                .min_by_key(|idx| distances.get(**idx))
                .unwrap();
            visited.insert(current);
            let neighbours = self.edges.get(current).unwrap().iter().filter(|idx| !visited.contains(idx));
            for neighbour in neighbours {
                let tmp = match distances[*current] {
                    std::usize::MAX => std::usize::MAX,
                    d => d + 1
                };
                if tmp < distances[*neighbour] {
                    distances[*neighbour] = tmp;
                    previous[*neighbour] = Some(*current);
                }
            }
        }

        distances
    }

    fn parse(input: &str) -> Self {
        let mut nodes = Vec::new();
        let mut edges = HashMap::new();
        let chart: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
        let width = chart.iter().map(Vec::len).max().unwrap();
        let height = chart.len();

        for y in 0..height {
            for x in 0..width {
                let node = Node::new(*chart.get(y).unwrap().get(x).unwrap());
                nodes.push(node);
            }
        }

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let mut candidates = Vec::new();
                if y > 0 { candidates.push((y - 1) * width + x) }
                if y < height - 1 { candidates.push((y + 1) * width + x) }
                if x > 0 { candidates.push(y * width + x - 1) }
                if x < width - 1 { candidates.push(y * width + x + 1) }
                let neighbours = candidates.into_iter().filter(|i| get_level(nodes.get(*i).unwrap().data) >= get_level(nodes.get(idx).unwrap().data) - 1).collect();
                edges.insert(idx, neighbours);
            }
        }

        Self { nodes, edges }
    }
}

struct Node {
    data: char,
}

impl Node {
    fn new(data: char) -> Self {
        Self { data }
    }
}

fn get_level(value: char) -> usize {
    1 + match value {
        'S' => 0,
        'E' => LEVELS.len() - 1,
        c => LEVELS.chars().position(|d| c == d).unwrap(),
    }
}

fn main() {
    let input = aoc::io::get_input(12);
    let graph = Graph::parse(&input);
    let start = graph.find('E');
    let end = graph.find('S');
    let distances = graph.compute_distances(start);
    println!("{}", distances[end]);  // 412
    println!("{}", graph.find_iter('a').map(|idx| distances[idx]).min().unwrap());  // 402
}