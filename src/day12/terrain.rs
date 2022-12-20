#[derive(Copy, Clone, PartialEq)]
pub struct Node {
    pub pos: (usize, usize),
    pub parent: Option<(usize, usize)>,
    pub g: u32,
    pub h: u32,
}

impl Node {
    pub fn new(pos: (usize, usize), parent: Option<(usize, usize)>, g: u32, h: u32) -> Self {
        Self { pos, parent, g, h }
    }

    pub fn get_f(&self) -> u32 {
        self.g + self.h
    }
}

pub struct Terrain {
    pub grid: Vec<Vec<u8>>,
    pub start: (usize, usize),
    pub goal: (usize, usize),
}

impl Terrain {
    fn new() -> Self {
        Self {
            grid: Vec::new(),
            start: (0usize, 0usize),
            goal: (0usize, 0usize),
        }
    }

    pub fn from_input(inputs: Vec<String>) -> Self {
        let mut terrain = Terrain::new();

        for (row, line) in inputs.iter().enumerate() {
            terrain.grid.push(Vec::new());
            for (col, ch) in line.chars().enumerate() {
                terrain.grid[row].push(match ch {
                    'S' => {
                        terrain.start = (row, col);
                        0
                    }
                    'E' => {
                        terrain.goal = (row, col);
                        25
                    }
                    c => (c as u8) - b'a',
                })
            }
        }

        terrain
    }

    pub fn make_node(&self, pos: (usize, usize), parent: Option<(usize, usize)>, g: u32) -> Node {
        Node::new(pos, parent, g, distance_squared(pos, self.goal))
    }

    pub fn at(&self, pos: (usize, usize)) -> u8 {
        self.grid[pos.0][pos.1]
    }

    pub fn get_neighbors(&self, node: Node) -> Vec<Node> {
        let mut neighbors: Vec<Node> = Vec::new();

        let height = self.at(node.pos);

        if node.pos.0 > 0 {
            let new_pos = (node.pos.0 - 1, node.pos.1);
            if self.at(new_pos) <= height + 1 {
                neighbors.push(self.make_node(new_pos, Some(node.pos), node.g + 1));
            }
        }
        if node.pos.1 > 0 {
            let new_pos = (node.pos.0, node.pos.1 - 1);
            if self.at(new_pos) <= height + 1 {
                neighbors.push(self.make_node(new_pos, Some(node.pos), node.g + 1));
            }
        }
        if node.pos.0 < self.grid.len() - 1 {
            let new_pos = (node.pos.0 + 1, node.pos.1);
            if self.at(new_pos) <= height + 1 {
                neighbors.push(self.make_node(new_pos, Some(node.pos), node.g + 1));
            }
        }
        if node.pos.1 < self.grid[0].len() - 1 {
            let new_pos = (node.pos.0, node.pos.1 + 1);
            if self.at(new_pos) <= height + 1 {
                neighbors.push(self.make_node(new_pos, Some(node.pos), node.g + 1));
            }
        }

        neighbors
    }
}

fn distance_squared(a: (usize, usize), b: (usize, usize)) -> u32 {
    let a0 = a.0 as i32;
    let a1 = a.1 as i32;
    let b0 = b.0 as i32;
    let b1 = b.1 as i32;
    ((b0 - a0) * (b0 - a0) + (b1 - a1) * (b1 - a1)) as u32
}
