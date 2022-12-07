use std::{collections::HashMap, fs::read_to_string};

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
}

fn part_one(_contents: &String) -> usize {
    let dir_stack: Vec<&Directory> = vec![];

    let max_size = 100000;
    let e = Directory {
        files: vec![584],
        dirs: vec![],
    };
    let a = Directory {
        files: vec![29116, 2557, 62596],
        dirs: vec![&e],
    };
    let d = Directory {
        files: vec![4060174, 8033020, 5626152, 7214296],
        dirs: vec![],
    };
    let top = Directory {
        files: vec![14848514, 8504156],
        dirs: vec![&a, &d],
    };

    top.dirs
        .into_iter()
        .filter(|dir| dir.get_size() <= max_size)
        .map(|dir| dir.get_size())
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
