use std::collections::{HashMap, HashSet};

struct Graph {
    nodes: Vec<char>,
    edges: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn find(&self, value: char) -> usize {
        self.nodes.iter().enumerate().filter(|(_, node)| **node == value).next().unwrap().0
    }

    fn find_iter(&self, value: char) -> impl Iterator<Item=usize> + '_ {
        self.nodes.iter().enumerate().filter(move |(_, node)| **node == value).map(|(idx, _)| idx)
    }

    fn compute_distances(&self, start: usize) -> Vec<usize> {  // Dijkstra
        let num_nodes = self.nodes.len();
        let mut distances = vec![std::usize::MAX; num_nodes];
        let mut visited = HashSet::new();
        let queue: Vec<_> = (0..num_nodes).collect();
        distances[start] = 0;
        while visited.len() < num_nodes {
            let current = queue.iter().filter(|idx| !visited.contains(idx)).min_by_key(|idx| distances.get(**idx)).unwrap();
            visited.insert(current);
            let neighbours = self.edges.get(current).unwrap().iter().filter(|idx| !visited.contains(idx));
            for neighbour in neighbours {
                let distance = match distances[*current] {
                    std::usize::MAX => std::usize::MAX,
                    d => d + 1
                };
                if distance < distances[*neighbour] {
                    distances[*neighbour] = distance;
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
                nodes.push(*chart.get(y).unwrap().get(x).unwrap());
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

                // Invert all edges because we perform our searches backwards.
                let min_level = get_level(*nodes.get(idx).unwrap()) - 1;
                let neighbours = candidates.into_iter().filter(|i| get_level(*nodes.get(*i).unwrap()) >= min_level).collect();
                edges.insert(idx, neighbours);
            }
        }
        Self { nodes, edges }
    }
}

fn get_level(value: char) -> usize {
    let levels = "abcdefghijklmnopqrstuvwxyz";
    1 + match value {  // +1 makes sure that we don't get underflow errors
        'S' => 0,
        'E' => levels.len() - 1,
        c => levels.chars().position(|d| c == d).unwrap(),
    }
}

fn main() {
    let input = aoc::io::get_input(12);
    let graph = Graph::parse(&input);

    // Invert our ``start`` and ``end`` which is necessary for part 2.
    let start = graph.find('E');
    let end = graph.find('S');
    let distances = graph.compute_distances(start);
    println!("{}", distances[end]);  // 412
    println!("{}", graph.find_iter('a').map(|idx| distances[idx]).min().unwrap());  // 402
}