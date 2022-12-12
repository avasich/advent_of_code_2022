use std::collections::HashSet;

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!("unsupported symbol {c}"),
    }
}

fn misplaced_item(rucksack: String) -> char {
    let (c1, c2) = rucksack.split_at(rucksack.len() / 2);
    let c1: &HashSet<_> = &c1.chars().collect();
    let c2: &HashSet<_> = &c2.chars().collect();
    *(c1 & c2).iter().next().unwrap()
}

pub fn check_part_one(filename: &str) -> u32 {
    crate::utils::read_lines(filename)
        .map(misplaced_item)
        .map(priority)
        .sum()
}

pub fn check_part_two(filename: &str) -> u32 {
    use itertools::Itertools;

    crate::utils::read_lines(filename)
        .chunks(3)
        .into_iter()
        .flat_map(|chunk| {
            chunk
                .map(|s| HashSet::<_>::from_iter(s.chars()))
                .reduce(|a, b| &a & &b)
                .unwrap()
        })
        .map(priority)
        .sum()
}

#[cfg(test)]
mod d03_tests {
    use super::*;

    static EXAMPLE_1_FILE: &str = "./inputs/day_03/example_1.txt";
    static TASK_FILE: &str = "./inputs/day_03/task.txt";

    #[test]
    fn example_1_test() {
        let res = check_part_one(EXAMPLE_1_FILE);
        assert_eq!(res, 157)
    }

    #[test]
    fn run_part_one() {
        let res = check_part_one(TASK_FILE);
        println!("{res}");
    }

    #[test]
    fn example_2_test() {
        let res = check_part_two(EXAMPLE_1_FILE);
        assert_eq!(res, 70)
    }

    #[test]
    fn run_part_two() {
        let res = check_part_two(TASK_FILE);
        println!("{res}");
    }
}
