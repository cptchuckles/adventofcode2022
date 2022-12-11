use std::collections::HashMap;

pub fn execute() {
    let inputs = crate::start_day::setup(8);
    let grid = inputs
        .iter()
        .map(|s| s.chars().map(|c| (c as u8) - b'0').collect())
        .collect::<Vec<Vec<u8>>>();

    let (rows, cols) = (grid.len(), grid[0].len());
    let mut visited_trees: HashMap<(usize, usize), bool> = HashMap::new();

    for row in 0..rows {
        visited_trees.insert((row, 0), true);
        visited_trees.insert((row, cols - 1), true);
    }

    for col in 1..(cols - 1) {
        visited_trees.insert((0, col), true);
        visited_trees.insert((rows - 1, col), true);
    }

    {
        let visits = &mut visited_trees;
        // From the north
        search_vertical(&grid, visits, 0..rows, 0..cols);
        // From the south
        search_vertical(&grid, visits, (0..rows).rev(), 0..cols);
        // From the west
        search_horizontal(&grid, visits, 0..rows, 0..cols);
        // From the east
        search_horizontal(&grid, visits, 0..rows, (0..cols).rev());
    }

    let visible_trees = visited_trees.values().filter(|v| **v).count();
    println!("Part 1: {}", visible_trees);
}

fn search_vertical<I1, I2>(
    grid: &Vec<Vec<u8>>,
    visited_trees: &mut HashMap<(usize, usize), bool>,
    rows: I1,
    cols: I2,
) where
    I1: Iterator<Item = usize> + Clone,
    I2: Iterator<Item = usize> + Clone,
{
    for col in cols {
        let mut highest = 0;
        for row in rows.clone() {
            let mut visible = false;
            let height = grid[row][col];
            if grid[row][col] > highest {
                visible = true;
                highest = height;
            }
            if let Some(visibility) = visited_trees.insert((row, col), visible) {
                if visibility != visible {
                    visited_trees.insert((row, col), true);
                }
            }
        }
    }
}

fn search_horizontal<I1, I2>(
    grid: &Vec<Vec<u8>>,
    visited_trees: &mut HashMap<(usize, usize), bool>,
    rows: I1,
    cols: I2,
) where
    I1: Iterator<Item = usize> + Clone,
    I2: Iterator<Item = usize> + Clone,
{
    for row in rows {
        let mut highest = 0;
        for col in cols.clone() {
            let mut visible = false;
            let height = grid[row][col];
            if grid[row][col] > highest {
                visible = true;
                highest = height;
            }
            if let Some(visibility) = visited_trees.insert((row, col), visible) {
                if visibility != visible {
                    visited_trees.insert((row, col), true);
                }
            }
        }
    }
}
