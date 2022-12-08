// See also:
//   - https://rust-unofficial.github.io/too-many-lists
//   - https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/

use std::collections::HashMap;

struct Arena<T> {
    nodes: Vec<Node<T>>,
}

impl<T> Arena<T> {
    fn new() -> Self {
        Self {
            nodes: Vec::new()
        }
    }

    fn get(&self, idx: usize) -> &Node<T> {
        return &self.nodes[idx]
    }

    fn get_mut(&mut self, idx: usize) -> &mut Node<T> {
        return &mut self.nodes[idx]
    }

    fn node(&mut self, data: T) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(Node {
            idx,
            data,
            parent: None,
            children: Vec::new()
        });
        idx
    }
}

impl Arena<Directory> {
    fn size(&self, idx: usize) -> u32 {
        let root = &self.nodes[idx];
        let a = root.data.size();
        let b: u32 = root.children.iter().map(|idx| self.size(*idx)).sum();
        a + b
    }
}

struct Node<T> {
    idx: usize,
    data: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T> {
    fn new(idx: usize, data: T) -> Self {
        Self {
            idx,
            data,
            parent: None,
            children: Vec::new(),
        }
    }
}

struct Directory {
    name: String,
    files: Vec<(String, u32)>
}

impl Directory {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            files: Vec::new(),
        }
    }

    fn size(&self) -> u32 {
        self.files.iter().map(|(_, size)| size).sum()
    }
}

fn aap() -> Arena<Directory> {
    let input = aoc::io::get_input(7);
    let mut filesystem = Arena::new();
    let root = filesystem.node(Directory::new(""));
    let mut cwd = root;
    for line in input.lines() {
        if line.starts_with("dir ") {
            let mut directory = Directory {
                name: String::from(&line[4..]),
                files: Vec::new(),
            };
            let node = filesystem.node(directory);
            filesystem.get_mut(cwd).children.push(node);
            filesystem.get_mut(node).parent = Some(cwd);
        }
        else if line.starts_with("$ cd ") {
            let dirname = &line[5..];
            if dirname == ".." {
                cwd = filesystem.get(cwd).parent.unwrap();
            } else {
                let a = filesystem
                    .get(cwd)
                    .children
                    .iter()
                    .filter(|idx| filesystem.get(**idx).data.name == String::from(dirname))
                    .next();

                if let Some(node) = a {
                    cwd = *node;
                }
            }
        }
        else if line == "$ ls" {
            // Do nothing.
        }
        else {
            let (size, filename) = line.split_once(" ").unwrap();
            filesystem.get_mut(cwd).data.files.push((String::from(filename), size.parse().unwrap()));
            println!("{}: {} ({})", filesystem.get(cwd).data.name, filename, size);
        }
    }

    filesystem
}

fn part1() -> u32 {
    let arena = aap();
    arena.nodes.iter().map(|node| arena.size(node.idx)).filter(|size| *size <= 100000).sum()
}

fn part2() -> u32 {
    let arena = aap();
    let used = arena.size(0);
    let to_free = 30000000 - (70000000 - used);
    arena.nodes.iter().map(|node| arena.size(node.idx)).filter(|size| *size >= to_free).min().unwrap()
}

fn main() {
    println!("{}", part1());  // 1307902
    println!("{}", part2());  // 7068748
}