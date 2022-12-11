use itertools::Itertools;

pub fn max_calories(filename: &str, n: usize) -> u32 {
    let mut totals = vec![];
    let mut sum = 0;
    let lines = crate::utils::read_lines(filename);

    for line in lines {
        if line.is_empty() {
            totals.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<u32>().unwrap();
        }
    }

    totals.push(sum);
    totals.iter().sorted_by(|a, b| b.cmp(a)).take(n).sum()
}

#[cfg(test)]
mod d01_tests {
    use super::*;

    static EXAMPLE_1: &str = "./inputs/day_01/example_1.txt";
    static TASK: &str = "./inputs/day_01/task.txt";

    #[test]
    fn part_one_example_test() {
        let res = max_calories(EXAMPLE_1, 1);
        assert_eq!(res, 24000)
    }

    #[test]
    fn run_part_one_task() {
        let res = max_calories(TASK, 1);
        println!("{res}");
    }

    #[test]
    fn part_two_example_test() {
        let res = max_calories(EXAMPLE_1, 3);
        assert_eq!(res, 45000)
    }

    #[test]
    fn run_part_two_task() {
        let res = max_calories(TASK, 3);
        println!("{res}");
    }
}
