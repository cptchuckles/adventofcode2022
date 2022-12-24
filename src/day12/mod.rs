mod terrain;
use self::terrain::{Node, Terrain};
use std::collections::HashSet;

pub fn execute() {
    let terrain = Terrain::from_input(crate::start_day::setup("12"));

    println!("Part 1: {}", a_star(&terrain));
    println!("Part 2: {}", dijkstra(&terrain));
}

fn a_star(terrain: &Terrain) -> u32 {
    let mut fringe: Vec<Node> = Vec::new();
    let mut closed: HashSet<(usize, usize)> = HashSet::new();
    let mut shortest_path: u32 = u32::MAX;

    fringe.push(terrain.make_node(terrain.start, None, 0));

    while let Some(current) = fringe.pop() {
        if current.pos == terrain.goal {
            shortest_path = std::cmp::min(shortest_path, current.g);
            continue;
        }

        'neighbors: for &neighbor in terrain
            .get_neighbors(current)
            .iter()
            .filter(|n| terrain.at(n.pos) <= terrain.at(current.pos) + 1)
        {
            if closed.contains(&neighbor.pos) {
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

        closed.insert(current.pos);

        fringe.sort_by(|a, b| {
            b.get_f()
                .partial_cmp(&a.get_f())
                .expect("Floating point comparison failed inexplicably")
        });
    }

    shortest_path
}

fn dijkstra(terrain: &Terrain) -> u32 {
    let mut fringe: Vec<Node> = Vec::new();
    let mut closed: HashSet<(usize, usize)> = HashSet::new();
    let mut shortest_path = u32::MAX;

    fringe.push(terrain.make_node(terrain.goal, None, 0));

    while let Some(current) = fringe.pop() {
        if terrain.at(current.pos) == 0 {
            shortest_path = std::cmp::min(shortest_path, current.g);
            continue;
        }

        for &neighbor in terrain
            .get_neighbors(current)
            .iter()
            .filter(|n| terrain.at(n.pos) >= terrain.at(current.pos) - 1)
        {
            if closed.insert(neighbor.pos) {
                fringe.push(neighbor);
            }
        }

        closed.insert(current.pos);

        fringe.sort_by(|a, b| b.g.cmp(&a.g));
    }

    shortest_path
}
