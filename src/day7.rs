enum Node {
    Dir(Dir),
    File(File),
}

struct Dir {
    name: String,
    contents: Vec<Node>,
}

struct File {
    name: String,
    size: u64,
}

impl Dir {
    fn new(new_name: &str) -> Dir {
        Dir {
            name: new_name.to_string(),
            contents: Vec::new(),
        }
    }

    fn recursively_get_size_and<F>(&self, mut then: F) -> u64
    where
        F: FnMut(u64),
    {
        let mut size = 0u64;

        for item in &self.contents {
            match item {
                Node::Dir(d) => size += d.recursively_get_size_and(&mut then),
                Node::File(f) => size += f.size,
            }
        }

        then(size);

        size
    }

    fn cd(&mut self, p: &str) -> Option<&mut Dir> {
        let mut d = None;
        let mut look = "";
        let mut next: Option<&str> = None;

        if let Some(p) = p.split_once("/") {
            look = p.0;
            next = Some(p.1);
        }

        for my_d in &mut self.contents {
            if let Node::Dir(my_d) = my_d {
                if my_d.name == look {
                    if let Some(next) = next {
                        d = my_d.cd(next);
                    } else {
                        d = Some(my_d);
                    }
                }
            }
        }

        d
    }
}

impl File {
    fn new(new_name: &str, new_size: u64) -> File {
        File {
            name: new_name.to_string(),
            size: new_size,
        }
    }
}

pub fn execute() {
    let inputs = crate::start_day::setup(7);

    let mut fs = Dir::new("root");
    let mut pwd = vec!["root"];

    for input in &inputs {
        let mut parts = input.split(' ');
        match parts.next().unwrap() {
            "$" => match parts.next().unwrap() {
                "cd" => match parts.next().unwrap() {
                    ".." => _ = pwd.pop(),
                    "/" => _ = pwd.drain(1..),
                    d => pwd.push(d),
                }
                _ => (), // ls
            },
            "dir" => {
                let pwd = pwd.join("/");
                let dir = parts.next().unwrap();
                if let Some(pwd) = fs.cd(&pwd) {
                    if let Some(_) = pwd.cd(dir) {
                        // nothing
                    } else {
                        pwd.contents.push(Node::Dir(Dir::new(dir)));
                    }
                }
            }
            sz => {
                if let Ok(sz) = sz.parse::<u64>() {
                    let f = parts.next().unwrap();
                    if let Some(pwd) = fs.cd(&pwd.join("/")) {
                        pwd.contents.push(Node::File(File::new(f, sz)));
                    }
                }
            },
        }
    }

    let mut total_sizes = 0u64;

    fs.recursively_get_size_and(|sz| {
        if sz <= 100000 {
            total_sizes += sz;
        }
    });

    println!("Part 1: {}", total_sizes);
}
