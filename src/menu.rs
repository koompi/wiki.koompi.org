use serde::{Deserialize, Serialize};

// A type to represent a path, split into its component parts
#[derive(Debug)]
pub struct Path {
    pub parts: Vec<String>,
}
impl Path {
    pub fn new(path: &str) -> Path {
        Path {
            parts: path.to_string().split("/").map(|s| s.to_string()).collect(),
        }
    }
}

// A recursive type to represent a directory tree.
// Simplification: If it has children, it is considered
// a directory, else considered a file.
#[derive(Debug, Clone, Deserialize, Serialize)]
// #[template(path = "index.stpl")]
pub struct Dir {
    pub name: String,
    pub children: Vec<Dir>,
}

impl Dir {
    fn new(name: &str) -> Dir {
        Dir {
            name: name.to_string(),
            children: Vec::<Dir>::new(),
        }
    }

    fn find_child(&mut self, name: &str) -> Option<&mut Dir> {
        for c in self.children.iter_mut() {
            if c.name == name {
                return Some(c);
            }
        }
        None
    }

    fn add_child<T>(&mut self, leaf: T) -> &mut Self
    where
        T: Into<Dir>,
    {
        self.children.push(leaf.into());
        self
    }
}

pub fn dir(val: &str) -> Dir {
    Dir::new(val)
}

pub fn build_tree(node: &mut Dir, parts: &Vec<String>, depth: usize) {
    if depth < parts.len() {
        let item = &parts[depth];

        let mut dir = match node.find_child(&item) {
            Some(d) => d,
            None => {
                let d = Dir::new(&item);
                node.add_child(d);
                match node.find_child(&item) {
                    Some(d2) => d2,
                    None => panic!("Got here!"),
                }
            }
        };
        build_tree(&mut dir, parts, depth + 1);
    }
}

// A function to print a Dir in format similar to unix `tree` command.
// fn print_dir(dir: &Dir, depth: u32) {
//     if depth == 0 {
//         println!("{}", dir.name);
//     } else {
//         println!(
//             "{:indent$}{} {}",
//             "",
//             "└──",
//             dir.name,
//             indent = ((depth as usize) - 1) * 4
//         );
//     }

//     for child in dir.children.iter() {
//         print_dir(child, depth + 1)
//     }
// }
