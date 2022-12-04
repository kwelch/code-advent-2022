use std::fs::read_to_string;

use itertools::Itertools;

static ASCII_LOWER: [char; 53] = [
    // empty first to avoid 0
    '-', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
    's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',
    'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn split_rucksack(line: &str) -> (&str, &str) {
    line.split_at(line.len() / 2)
}

fn get_matching_char(s1: &str, s2: &str) -> char {
    s1.chars().find(|c| s2.contains(*c)).unwrap()
}

fn convert_char_to_priority(c: char) -> usize {
    ASCII_LOWER
        .into_iter()
        .find_position(|l| *l == c)
        .unwrap()
        .0
}

fn part_one(_contents: &String) -> usize {
    _contents
        .lines()
        .map(|line| split_rucksack(line))
        .map(|(s1, s2)| get_matching_char(s1, s2))
        .map(|same_char: char| convert_char_to_priority(same_char))
        .sum()
}

fn part_two(_contents: &String) -> usize {
    _contents
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut sack_group| {
            let s1 = sack_group.next().unwrap();
            let s2 = sack_group.next().unwrap();
            let s3 = sack_group.next().unwrap();
            let matching_char = s1
                .chars()
                .find(|c| s2.contains(*c) && s3.contains(*c))
                .unwrap();
            convert_char_to_priority(matching_char)
        })
        .sum()
}

fn main() {
    let contents =
        read_to_string("examples/03/main.in").expect("Something went wrong reading the file");

    println!(
        "What is the sum of the priorities of those item types?\n- {}",
        part_one(&contents)
    );

    println!(
        "What is the sum of the priorities of those item types?\n- {}",
        part_two(&contents)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_contents() -> String {
        read_to_string("examples/03/sample.in").expect("Something went wrong reading the file")
    }

    #[test]
    fn test_part_one() {
        assert_eq!(157, part_one(&get_contents()));
    }

    #[test]
    fn test_split_rucksack() {
        assert_eq!(
            ("vJrwpWtwJgWr", "hcsFMMfFFhFp"),
            split_rucksack("vJrwpWtwJgWrhcsFMMfFFhFp")
        );
    }

    #[test]
    fn test_get_matching_char() {
        assert_eq!('p', get_matching_char("vJrwpWtwJgWr", "hcsFMMfFFhFp"));
    }

    #[test]
    fn test_convert_char_to_priority() {
        assert_eq!(16, convert_char_to_priority('p'));
        assert_eq!(38, convert_char_to_priority('L'));
        assert_eq!(42, convert_char_to_priority('P'));
        assert_eq!(22, convert_char_to_priority('v'));
        assert_eq!(20, convert_char_to_priority('t'));
        assert_eq!(19, convert_char_to_priority('s'));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(70, part_two(&get_contents()));
    }
}
