use std::fs::read_to_string;

use itertools::Itertools;

fn get_first_uniq_set_index(input: &str, max_size: usize) -> usize {
    let mut marker = vec![];
    let mut output = 0;

    for (idx, letter) in input.chars().enumerate() {
        marker.push(letter);

        if marker.len() > max_size {
            marker.drain(0..1);
        }

        let uniq: Vec<char> = marker.clone().into_iter().unique().collect();

        if uniq.len() == max_size {
            output = idx + 1;
            break;
        }
    }

    output
}

fn part_one(_contents: &str) -> usize {
    get_first_uniq_set_index(_contents, 4)
}

fn part_two(_contents: &str) -> usize {
    get_first_uniq_set_index(_contents, 14)
}

fn main() {
    let contents =
        read_to_string("examples/06/main.in").expect("Something went wrong reading the file");

    println!("Part 1?\n- {}", part_one(&contents));

    println!("Part 2?\n- {}", part_two(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> String {
        read_to_string("examples/06/sample.in").expect("Something went wrong reading the file")
    }

    #[test]
    fn test_part_one() {
        let mut tests = vec![];
        let binding = get_test_data();
        for line in binding.lines() {
            tests.push(line);
        }

        assert_eq!(7, part_one(tests[0]));
        assert_eq!(5, part_one(tests[1]));
        assert_eq!(6, part_one(tests[2]));
        assert_eq!(10, part_one(tests[3]));
        assert_eq!(11, part_one(tests[4]));
    }

    #[test]
    fn test_part_two() {
        let mut tests = vec![];
        let binding = get_test_data();
        for line in binding.lines() {
            tests.push(line);
        }
        assert_eq!(19, part_two(tests[0]));
        assert_eq!(23, part_two(tests[1]));
        assert_eq!(23, part_two(tests[2]));
        assert_eq!(29, part_two(tests[3]));
        assert_eq!(26, part_two(tests[4]));
    }
}
