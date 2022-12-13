use std::collections::HashSet;

pub fn execute() {
    let inputs = crate::start_day::setup(8);
    let grid = inputs
        .iter()
        .map(|s| s.chars().map(|c| (c as u8) - b'0').collect())
        .collect::<Vec<Vec<u8>>>();

    println!("Part 1: {}", count_visible_trees_from_without(&grid));
    println!("Part 2: {}", find_best_scenic_score(&grid));
}

fn count_visible_trees_from_without(grid: &Vec<Vec<u8>>) -> usize {
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

fn find_best_scenic_score(grid: &Vec<Vec<u8>>) -> u64 {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut best_score = 0u64;

    for row in 0..rows {
        for col in 0..cols {
            let current_score = count_visible_trees_from_pos(grid, (row, col));
            best_score = std::cmp::max(current_score, best_score);
        }
    }

    best_score
}

fn count_visible_trees_from_pos(grid: &Vec<Vec<u8>>, pos: (usize, usize)) -> u64 {
    let (rows, cols) = (grid.len(), grid[0].len());

    let starting_height = grid[pos.0][pos.1];

    let mut upscore = 0u64;
    for rowup in (0..pos.0).rev() {
        let current_height = grid[rowup][pos.1];
        upscore += 1;
        if current_height >= starting_height {
            break;
        }
    }

    let mut downscore = 0u64;
    for rowdown in (pos.0 + 1)..rows {
        let current_height = grid[rowdown][pos.1];
        downscore += 1;
        if current_height >= starting_height {
            break;
        }
    }

    let mut leftscore = 0u64;
    for colleft in (0..pos.1).rev() {
        let current_height = grid[pos.0][colleft];
        leftscore += 1;
        if current_height >= starting_height {
            break;
        }
    }

    let mut rightscore = 0u64;
    for colright in (pos.1 + 1)..cols {
        let current_height = grid[pos.0][colright];
        rightscore += 1;
        if current_height >= starting_height {
            break;
        }
    }

    [rightscore, upscore, leftscore, downscore]
        .iter()
        .map(|score| match score {
            0 => 1,
            &n => n,
        })
        .fold(1, |acc, n| acc * n)
}
