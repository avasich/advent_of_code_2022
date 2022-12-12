use std::collections::{HashSet, VecDeque};

fn count_unique<'a>(xs: impl Iterator<Item = &'a char>) -> usize {
    HashSet::<_>::from_iter(xs).len()
}

pub fn sequence_start(filename: &str, len: usize) -> Result<usize, ()> {
    let line = crate::utils::read_lines(filename).next().unwrap();
    let mut chars = line.chars();
    let mut window = VecDeque::from_iter((&mut chars).take(len));

    if count_unique(window.iter()) == len {
        return Ok(1);
    }

    for (i, c) in chars.enumerate() {
        window.pop_front();
        window.push_back(c);

        if count_unique(window.iter()) == len {
            return Ok(i + len + 1);
        }
    }

    Err(())
}

#[cfg(test)]
mod d05_tests {
    use super::*;

    static EXAMPLE_1_FILE: &str = "./inputs/day_06/example_1.txt";
    static EXAMPLE_2_FILE: &str = "./inputs/day_06/example_2.txt";
    static EXAMPLE_3_FILE: &str = "./inputs/day_06/example_3.txt";
    static EXAMPLE_4_FILE: &str = "./inputs/day_06/example_4.txt";
    static EXAMPLE_5_FILE: &str = "./inputs/day_06/example_5.txt";
    static TASK_FILE: &str = "./inputs/day_06/task.txt";

    #[test]
    fn example_1_test() {
        assert_eq!(Ok(7), sequence_start(EXAMPLE_1_FILE, 4));
        assert_eq!(Ok(5), sequence_start(EXAMPLE_2_FILE, 4));
        assert_eq!(Ok(6), sequence_start(EXAMPLE_3_FILE, 4));
        assert_eq!(Ok(10), sequence_start(EXAMPLE_4_FILE, 4));
        assert_eq!(Ok(11), sequence_start(EXAMPLE_5_FILE, 4));
    }

    #[test]
    fn run_part_one() {
        let res = sequence_start(TASK_FILE, 4);
        println!("{res:?}");
    }

    #[test]
    fn example_2_test() {
        assert_eq!(Ok(19), sequence_start(EXAMPLE_1_FILE, 14));
        assert_eq!(Ok(23), sequence_start(EXAMPLE_2_FILE, 14));
        assert_eq!(Ok(23), sequence_start(EXAMPLE_3_FILE, 14));
        assert_eq!(Ok(29), sequence_start(EXAMPLE_4_FILE, 14));
        assert_eq!(Ok(26), sequence_start(EXAMPLE_5_FILE, 14));
    }

    #[test]
    fn run_part_two() {
        let res = sequence_start(TASK_FILE, 14);
        println!("{res:?}");
    }
}
