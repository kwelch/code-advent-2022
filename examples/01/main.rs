use std::fs::read_to_string;

fn part_one(contents: &String) -> u32 {
    let mut split_str = contents.split("\n").peekable();
    let mut current_elf_total = 0;
    let mut highest_elf_total = 0;

    while let Some(str) = split_str.next()  {
        let curr_val = str.parse::<u32>();
        if !curr_val.is_ok() {

            if current_elf_total > highest_elf_total {
                highest_elf_total = current_elf_total;
            }
            current_elf_total = 0;
        } else {
            current_elf_total = current_elf_total + curr_val.unwrap();
        }
        
        // if it is the last one 
        if split_str.peek().is_none() {
            if current_elf_total > highest_elf_total {
                highest_elf_total = current_elf_total;
            }
        }
    }

    return highest_elf_total;
}

fn part_two(contents: &String) -> u32 {
    let top_count = 3;
    let mut split_str = contents.split("\n").peekable();
    let mut current_elf_total = 0;
    let mut elfs = Vec::new();

    while let Some(str) = split_str.next()  {
        let curr_val = str.parse::<u32>();
        if !curr_val.is_ok() {
            elfs.push(current_elf_total);
            current_elf_total = 0;
        } else {
            current_elf_total = current_elf_total + curr_val.unwrap();
        }
        
        // if it is the last one 
        if split_str.peek().is_none() {
            elfs.push(current_elf_total);
        }
    }
    let mut total_top_elf_snacks = 0;
    let mut top_elfs = elfs.clone();
    top_elfs.sort();
    top_elfs.reverse();
    top_elfs.truncate(top_count);

    for elf_snacks in top_elfs {
        total_top_elf_snacks = total_top_elf_snacks + elf_snacks;
    }

    return total_top_elf_snacks;
}

fn main() {
    let contents =
        read_to_string("examples/01/main.in").expect("Something went wrong reading the file");


    println!("?\n- {}", part_one(&contents));

    println!("?\n- {}", part_two(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_contents() -> String {
        read_to_string("examples/01/sample.in").expect("Something went wrong reading the file")
    }

    #[test]
    fn test_part_one() {
        assert_eq!(24000, part_one(&get_contents()));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(45000, part_two(&get_contents()));
    }
}