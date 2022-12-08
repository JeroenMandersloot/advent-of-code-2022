// See also:
//   - https://rust-unofficial.github.io/too-many-lists
//   - https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/

// Implement an `Arena` struct to hold our file system nodes such that all
// directories, files, etc. have the same lifetime.
struct FileSystem {
    nodes: Vec<Node>,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            nodes: Vec::new()
        }
    }

    fn create_node(&mut self, directory: Directory) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(Node {
            idx,
            directory,
            parent: None,
            children: Vec::new(),
        });
        idx
    }

    fn get(&self, idx: usize) -> &Node {
        &self.nodes[idx]
    }

    fn get_mut(&mut self, idx: usize) -> &mut Node {
        &mut self.nodes[idx]
    }

    fn size(&self, idx: usize) -> u32 {
        let root = &self.nodes[idx];
        let own_size = root.directory.size();
        let nested_size: u32 = root.children.iter().map(|idx| self.size(*idx)).sum();
        own_size + nested_size
    }
}

struct Node {
    idx: usize,
    directory: Directory,
    parent: Option<usize>,
    children: Vec<usize>,
}

struct Directory {
    name: String,
    files: Vec<File>,
}

impl Directory {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            files: Vec::new(),
        }
    }

    fn size(&self) -> u32 {
        self.files.iter().map(|File(_, size)| size).sum()
    }
}

struct File(String, u32);

fn build_file_system() -> FileSystem {
    let input = aoc::io::get_input(7);
    let mut fs = FileSystem::new();
    let mut cwd = fs.create_node(Directory::new("/"));
    for line in input.lines() {
        if line.starts_with("dir ") {
            let directory = Directory {
                name: String::from(&line[4..]),
                files: Vec::new(),
            };
            let node = fs.create_node(directory);
            fs.get_mut(cwd).children.push(node);
            // We can safely use ``node`` and ``cwd`` again despite the move
            // since both are primitive types that implement the ``Copy`` trait.
            fs.get_mut(node).parent = Some(cwd);
        } else if line == "$ cd .." {
            cwd = fs.get(cwd).parent.unwrap();
        } else if line.starts_with("$ cd ") {
            if let Some(node) = fs
                .get(cwd)
                .children
                .iter()
                .filter(|idx| fs.get(**idx).directory.name == String::from(&line[5..]))
                .next() { cwd = *node; }
        } else if line == "$ ls" {
            // Do nothing.
        } else {
            let (size, filename) = line.split_once(" ").unwrap();
            fs.get_mut(cwd).directory.files.push(File(String::from(filename), size.parse().unwrap()));
        }
    }

    fs
}

fn part1() -> u32 {
    let fs = build_file_system();
    fs.nodes.iter().map(|node| fs.size(node.idx)).filter(|size| *size <= 100000).sum()
}

fn part2() -> u32 {
    let fs = build_file_system();
    let used = fs.size(0);
    let to_free = 30000000 - (70000000 - used);
    fs.nodes.iter().map(|node| fs.size(node.idx)).filter(|size| *size >= to_free).min().unwrap()
}

fn main() {
    println!("{}", part1());  // 1307902
    println!("{}", part2());  // 7068748
}