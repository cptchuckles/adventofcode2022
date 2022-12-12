use std::collections::HashSet;

pub fn execute() {
    let inputs = crate::start_day::setup(8);
    let grid = inputs
        .iter()
        .map(|s| s.chars().map(|c| (c as u8) - b'0').collect())
        .collect::<Vec<Vec<u8>>>();

    println!("Part 1: {}", count_visible_trees(&grid));
}

fn count_visible_trees(grid: &Vec<Vec<u8>>) -> usize {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();

    for row in 0..rows {
        visible_trees.insert((row, 0));
        visible_trees.insert((row, cols - 1));
    }

    for col in 1..(cols - 1) {
        visible_trees.insert((0, col));
        visible_trees.insert((rows - 1, col));
    }

    let visits = &mut visible_trees;
    // From the north
    search_vertical(grid, visits, 0..rows, 0..cols);
    // From the south
    search_vertical(grid, visits, (0..rows).rev(), 0..cols);
    // From the west
    search_horizontal(grid, visits, 0..rows, 0..cols);
    // From the east
    search_horizontal(grid, visits, 0..rows, (0..cols).rev());

    visible_trees.iter().count()
}

fn search_vertical<I1, I2>(
    grid: &Vec<Vec<u8>>,
    visible_trees: &mut HashSet<(usize, usize)>,
    rows: I1,
    cols: I2,
) where
    I1: Iterator<Item = usize> + Clone,
    I2: Iterator<Item = usize> + Clone,
{
    for col in cols {
        let mut highest = 0;
        for row in rows.clone() {
            let height = grid[row][col];
            if grid[row][col] > highest {
                highest = height;
                visible_trees.insert((row, col));
            }
        }
    }
}

fn search_horizontal<I1, I2>(
    grid: &Vec<Vec<u8>>,
    visible_trees: &mut HashSet<(usize, usize)>,
    rows: I1,
    cols: I2,
) where
    I1: Iterator<Item = usize> + Clone,
    I2: Iterator<Item = usize> + Clone,
{
    for row in rows {
        let mut highest = 0;
        for col in cols.clone() {
            let height = grid[row][col];
            if grid[row][col] > highest {
                highest = height;
                visible_trees.insert((row, col));
            }
        }
    }
}
