mod terrain;
use self::terrain::{Node, Terrain};
use std::{collections::HashMap, cmp::Ordering};

pub fn execute() {
    let terrain = Terrain::from_input(crate::start_day::setup("12"));

    let mut fringe: Vec<Node> = Vec::new();
    let mut closed: HashMap<(usize, usize), Node> = HashMap::new();
    let mut path_length: u32 = u32::MAX;
    let mut paths: Vec<Vec<(usize, usize)>> = Vec::new();

    fringe.push(terrain.make_node(terrain.start, None, 0));

    while let Some(current) = fringe.pop() {
        if current.pos == terrain.goal {
            path_length = std::cmp::min(path_length, current.g);
            let mut path = vec![current.pos];
            let mut head = current;
            while let Some(parent) = head.parent {
                path.push(parent);
                head = *closed.get(&parent).unwrap();
            }
            paths.push(path);
            continue;
        }

        'neighbors: for neighbor in terrain.get_neighbors(current) {
            if closed.contains_key(&neighbor.pos) {
                continue;
            }
            for (i, node) in fringe.iter().enumerate() {
                if node.pos == neighbor.pos {
                    if node.g > neighbor.g {
                        fringe[i] = neighbor;
                    }
                    continue 'neighbors;
                }
            }
            fringe.push(neighbor);
        }

        closed.insert(current.pos, current);

        fringe.sort_by(|a, b| if b.get_f() < a.get_f() {
            Ordering::Less
        } else {
            Ordering::Greater
        });
    }

    println!("Part 1: {}", path_length);
}
