use std::fs::read_to_string;

use itertools::Itertools;

fn part_one(_contents: &String) -> String {
    let (map_str, commands) = _contents.split_once("\n\n").unwrap();
    let mut map = Vec::new();

    for line in map_str.lines() {
        for (idx, cargo_name) in line.chars().enumerate() {
            if cargo_name.is_alphabetic() {
                let row = (idx - 1) / 4;

                if row >= map.len() {
                    map.resize(row + 1, vec![]);
                }

                map[row].push(cargo_name.to_string());
            }
            if cargo_name.is_numeric() {
                // numbers are on the last line so we can fix the order of the stack now
                let row = (idx - 1) / 4;
                map[row].reverse();
            }
        }
    }

    for line in commands.lines() {
        let a = line
            .split_ascii_whitespace()
            .into_iter()
            .skip(1)
            .step_by(2)
            .map(|s| usize::from_str_radix(s, 10).unwrap())
            .collect::<Vec<usize>>();
        let count = a[0];
        let from = a[1] - 1;
        let to = a[2] - 1;

        for _count in 0..count {
            let value = map[from].pop().unwrap();
            map[to].push(value);
        }
    }

    map.iter()
        .map(|row| row.last().unwrap().to_owned())
        .into_iter()
        .join("")
}

fn part_two(_contents: &String) -> String {
    let (map_str, commands) = _contents.split_once("\n\n").unwrap();
    let mut map = Vec::new();

    for line in map_str.lines() {
        for (idx, cargo_name) in line.chars().enumerate() {
            if cargo_name.is_alphabetic() {
                let row = (idx - 1) / 4;

                if row >= map.len() {
                    map.resize(row + 1, vec![]);
                }

                map[row].push(cargo_name.to_string());
            }
            if cargo_name.is_numeric() {
                // numbers are on the last line so we can fix the order of the stack now
                let row = (idx - 1) / 4;
                map[row].reverse();
            }
        }
    }

    for line in commands.lines() {
        let a = line
            .split_ascii_whitespace()
            .into_iter()
            .skip(1)
            .step_by(2)
            .map(|s| usize::from_str_radix(s, 10).unwrap())
            .collect::<Vec<usize>>();
        let count = a[0];
        let from = a[1] - 1;
        let to = a[2] - 1;
        let mut moving = vec![];

        for _count in 0..count {
            let value = map[from].pop().unwrap();
            moving.push(value);
        }
        moving.reverse();
        map[to].append(&mut moving);
    }

    map.iter()
        .map(|row| row.last().unwrap().to_owned())
        .into_iter()
        .join("")
}

fn main() {
    let contents =
        read_to_string("examples/05/main.in").expect("Something went wrong reading the file");

    println!("After the rearrangement procedure completes, what crate ends up on top of each stack?\n- {}", part_one(&contents));

    println!("After the rearrangement procedure completes, what crate ends up on top of each stack?\n- {}", part_two(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_contents() -> String {
        read_to_string("examples/05/sample.in").expect("Something went wrong reading the file")
    }

    #[test]
    fn test_part_one() {
        assert_eq!("CMZ", part_one(&get_contents()));
    }

    #[test]
    fn test_part_two() {
        assert_eq!("MCD", part_two(&get_contents()));
    }
}
