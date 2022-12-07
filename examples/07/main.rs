use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;

#[derive(Clone)]
struct Directory<'a> {
    files: Vec<usize>,
    dirs: Vec<&'a Directory<'a>>,
}

impl<'a> Directory<'a> {
    fn new<'b>() -> Directory<'b> {
        Directory {
            files: vec![],
            dirs: vec![],
        }
    }
    fn get_size(&self) -> usize {
        let total_size: usize = self.files.clone().into_iter().sum();

        let dir_total: usize = self
            .dirs
            .clone()
            .into_iter()
            .map(|dir| dir.get_size())
            .sum();

        total_size + dir_total
    }

    fn add_file(&mut self, size: usize) {
        self.files.push(size);
    }

    fn add_dir(&mut self, dir: &Directory<'a>) {
        self.dirs.push(dir);
    }
}

fn part_one(_contents: &String) -> usize {
    let max_size = 100000;
    let mut dir_stack: Vec<&str> = vec![];
    let mut all_dirs: HashMap<&str, Directory> = HashMap::new();

    for line in _contents.lines() {
        match line.split_once(" ").unwrap() {
            ("$", command) => {
                match command {
                    // no details from this line skip
                    "ls" => {
                        println!("{:?}", dir_stack.last().unwrap());
                    }
                    command if command.starts_with("cd") => {
                        let (_, dir_name) = command.split_once(" ").unwrap();
                        if dir_name == ".." {
                            dir_stack.pop();
                        } else {
                            all_dirs.insert(dir_name, Directory::new());
                            dir_stack.push(dir_name)
                        }
                    }
                    _ => println!("Unknown command: {}", command),
                }
            }
            (size_str, _) if usize::from_str_radix(size_str, 10).is_ok() => {
                let size  = usize::from_str_radix(size_str, 10).unwrap();
                let dir = all_dirs.get(dir_stack.last().unwrap()).unwrap();
                dir.add_file(size);
            }
            _ => println!("Don't know have to handle: {}", line),
        }
    }

    println!("{:?}", dir_stack);

    // all_dirs.insert("e", &Directory {
    //     files: vec![584],
    //     dirs: vec![],
    // });

    // let dir = Directory {
    //     files: vec![29116, 2557, 62596],
    //     dirs: vec![all_dirs.get("e").unwrap()],
    // };
    // all_dirs.insert("a", &dir);

    // all_dirs.insert("d", Directory {
    //     files: vec![4060174, 8033020, 5626152, 7214296],
    //     dirs: vec![],
    // });
    // all_dirs.insert("top", Directory {
    //     files: vec![14848514, 8504156],
    //     dirs: vec![all_dirs.get("a").unwrap(), &all_dirs.get("d").unwrap()],
    // });

    all_dirs
        .into_iter()
        .filter(|(_name, dir)| dir.get_size() <= max_size)
        .map(|(_name, dir)| dir.get_size())
        .sum()
}

fn part_two(_contents: &String) -> u32 {
    2
}

fn main() {
    let contents =
        read_to_string("examples/07/main.in").expect("Something went wrong reading the file");

    println!("?\n- {}", part_one(&contents));

    println!("?\n- {}", part_two(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_contents() -> String {
        read_to_string("examples/07/sample.in").expect("Something went wrong reading the file")
    }

    #[test]
    fn test_part_one() {
        assert_eq!(95437, part_one(&get_contents()));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(2, part_two(&get_contents()));
    }
}
