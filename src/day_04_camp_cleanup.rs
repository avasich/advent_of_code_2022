use itertools::Itertools;

fn parse_intervals(line: String) -> (u32, u32, u32, u32) {
    line.split(|c| c == ',' || c == '-')
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect_tuple()
        .unwrap()
}

pub fn count_full_overlaps(filename: &str) -> usize {
    crate::utils::read_lines(filename)
        .map(parse_intervals)
        .filter(|(a1, a2, b1, b2)| a1 <= b1 && b2 <= a2 || b1 <= a1 && a2 <= b2)
        .count()
}

pub fn count_all_overlaps(filename: &str) -> usize {
    crate::utils::read_lines(filename)
        .map(parse_intervals)
        .filter(|(a1, a2, b1, b2)| {
            let a = a1..=a2;
            let b = b1..=b2;
            a.contains(&b1) || a.contains(&b2) || b.contains(&a1) || b.contains(&a2)
        })
        .count()
}

#[cfg(test)]
mod d04_tests {
    use super::*;

    static EXAMPLE_1_FILE: &str = "./inputs/day_04/example_1.txt";
    static TASK_FILE: &str = "./inputs/day_04/task.txt";

    #[test]
    fn example_1_test() {
        let res = count_full_overlaps(EXAMPLE_1_FILE);
        assert_eq!(res, 2);
    }

    #[test]
    fn run_part_one() {
        let res = count_full_overlaps(TASK_FILE);
        println!("{res}");
    }

    #[test]
    fn example_2_test() {
        let res = count_all_overlaps(EXAMPLE_1_FILE);
        assert_eq!(res, 4);
    }

    #[test]
    fn run_part_two() {
        let res = count_all_overlaps(TASK_FILE);
        println!("{res}");
    }
}
