pub fn execute() {
    let inputs = crate::start_day::setup(5);

    let mut stacks: Vec<Vec<char>> = Vec::new();

    let mut lines = inputs.iter().enumerate();

    while let Some((i, s)) = lines.next() {
        if s.len() == 0 {
            let mut backlines = inputs.iter().take(i).rev();

            if let Some(s) = backlines.next() {
                for _ in 0..((s.len() - 1) / 4 + 1) {
                    stacks.push(Vec::new());
                }
            }

            while let Some(s) = backlines.next() {
                let s = "..".to_string() + s;
                let mut chars = s.chars();
                let mut stack = 0;
                while let Some(c) = chars.nth(3) {
                    match c {
                        'A'..='Z' => stacks[stack].push(c),
                        _ => (),
                    }
                    stack += 1;
                }
            }

            break;
        }
    }

    while let Some((_, s)) = lines.next() {
        let m: Vec<u32> = s
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        for _ in 0..m[0] {
            if let Some(c) = stacks[m[1] as usize - 1].pop() {
                stacks[m[2] as usize - 1].push(c);
            }
        }
    }

    let mut tops: Vec<char> = Vec::new();

    for stack in stacks {
        if let Some(c) = stack.last() {
            tops.push(*c);
        }
    }

    println!("Part 1: {}", tops.iter().collect::<String>());
}