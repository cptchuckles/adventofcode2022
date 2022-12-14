use std::collections::HashMap;

pub fn execute() {
    let inputs = crate::start_day::setup("07");

    let mut dirs: HashMap<String, u64> = HashMap::new();
    let mut pwd = vec!["".to_string()];

    for input in inputs {
        let mut parts = input.split(' ');
        match parts.next().unwrap() {
            "$" => match parts.next().unwrap() {
                "cd" => match parts.next().unwrap() {
                    "/" => _ = pwd.drain(1..),
                    ".." => _ = pwd.pop(),
                    dirname => pwd.push(dirname.to_string()),
                },
                _ => (), // ls
            },
            sz => if let Ok(sz) = sz.parse::<u64>() {
                let mut path = pwd.clone();
                while let Some(basename) = path.pop() {
                    let pathstr = path.join("/") + "/" + &basename;
                    if let Some(current_sz) = dirs.insert(pathstr.clone(), sz) {
                        dirs.insert(pathstr, current_sz + sz);
                    }
                }
            }
        }
    }

    let mut total_sizes = 0u64;

    let du = *dirs.get("/").unwrap();
    let space_available = 70_000_000u64 - du;
    let space_needed = 30_000_000u64 - space_available;
    let mut smallest_nuked_dir_size = du;

    for (_, v) in dirs {
        if v <= 100000 {
            total_sizes += v;
        }
        if v >= space_needed {
            smallest_nuked_dir_size = std::cmp::min(smallest_nuked_dir_size, v);
        }
    }

    println!("Part 1: {}", total_sizes);
    println!("Part 2: {}", smallest_nuked_dir_size);
}
