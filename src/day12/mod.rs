mod terrain;
use self::terrain::{Node, Terrain};
use std::collections::HashMap;

pub fn execute() {
    let terrain = Terrain::from_input(crate::start_day::setup("12"));

    let mut fringe: HashMap<(usize, usize), Node> = HashMap::new();
    let mut closed: HashMap<(usize, usize), Node> = HashMap::new();
    let mut path_length: u32 = u32::MAX;
    let mut paths: Vec<Vec<(usize, usize)>> = Vec::new();

    fringe.insert(terrain.start, terrain.make_node(terrain.start, None, 0));

    while fringe.len() > 0 {
        let current = *fringe.values().min_by_key(|n| n.get_f()).unwrap();
        fringe.remove(&current.pos);

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

        for neighbor in terrain.get_neighbors(current) {
            if closed.contains_key(&neighbor.pos) {
                continue;
            }
            if let Some(node) = fringe.get(&neighbor.pos) {
                if node.g < neighbor.g {
                    continue;
                }
            }
            fringe.insert(neighbor.pos, neighbor);
        }

        closed.insert(current.pos, current);
    }

    println!("Part 1: {}", path_length);
}
