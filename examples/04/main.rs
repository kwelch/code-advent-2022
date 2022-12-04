use std::fs::read_to_string;

use itertools::Itertools;

fn convert_range_to_array(range: &str) -> (u32, u32) {
    range
        .split_once("-")
        .map(|(s, e)| (s.parse().unwrap(), e.parse().unwrap()))
        .unwrap()
}

fn part_one(_contents: &String) -> usize {
    _contents
        .lines()
        .flat_map(|line| line.split(',').tuples())
        .filter(|(a1, a2)| {
            let (rs1, re1) = convert_range_to_array(a1);
            let (rs2, re2) = convert_range_to_array(a2);

            ((rs1 <= rs2 && rs2 <= re1) && (rs1 <= re2 && re2 <= re1))
                || ((rs2 <= rs1 && rs1 <= re2) && (rs2 <= re1 && re1 <= re2))
        })
        .count()
}

fn part_two(_contents: &String) -> usize {
    _contents
        .lines()
        .flat_map(|line| line.split(',').tuples())
        .filter(|(a1, a2)| {
            let (rs1, re1) = convert_range_to_array(a1);
            let (rs2, re2) = convert_range_to_array(a2);

            ((rs1 <= rs2 && rs2 <= re1) || (rs1 <= re2 && re2 <= re1))
                || ((rs2 <= rs1 && rs1 <= re2) || (rs2 <= re1 && re1 <= re2))
        })
        .count()
}

fn main() {
    let contents =
        read_to_string("examples/04/main.in").expect("Something went wrong reading the file");

    println!(
        "In how many assignment pairs does one range fully contain the other?\n- {}",
        part_one(&contents)
    );

    println!(
        "In how many assignment pairs do the ranges overlap?\n- {}",
        part_two(&contents)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_contents() -> String {
        read_to_string("examples/04/sample.in").expect("Something went wrong reading the file")
    }

    #[test]
    fn test_part_one() {
        assert_eq!(2, part_one(&get_contents()));
    }

    #[test]
    fn test_convert_range_to_array() {
        assert_eq!((2, 6), convert_range_to_array("2-6"));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(4, part_two(&get_contents()));
    }
}
