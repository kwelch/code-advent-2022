

fn find_elf_snacks(input_str: String) -> i32 {
    let split_str = input_str.split("\n");

    let mut current_elf_total = 0;
    let mut highest_elf_total = 0;

    for str in split_str {
        let curr_val = str.parse::<i32>();
        if !curr_val.is_ok() {
            if current_elf_total > highest_elf_total {
                highest_elf_total = current_elf_total;
            }
            current_elf_total = 0;
        } else {
            current_elf_total = current_elf_total + curr_val.unwrap();
        }
    }

    return highest_elf_total;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::fs;

    #[test]
    fn test_sample() {
        //added '\n\' so that the format matched and we didn't have weird spaces
        let input = "1000\n\
        2000\n\
        3000\n\
        \n\
        4000\n\
        \n\
        5000\n\
        6000\n\
        \n\
        7000\n\
        8000\n\
        9000\n\
        \n\
        10000".to_string();
        assert_eq!(find_elf_snacks(input), 24000);
    }

    #[test]
    fn test_real() {
        let contents = fs::read_to_string("./src/tests/2022-1-input.txt").expect("Should be able to read the file");
        // test will fail but that is fine, the output will give me the answer
        assert_eq!(find_elf_snacks(contents), 0)
    }
}
