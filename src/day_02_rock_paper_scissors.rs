#[derive(Copy, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    fn from_str(s: &str) -> Self {
        use Move::*;

        match s {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => panic!("unknown move notation: {s}"),
        }
    }

    fn points_against(&self, other: Self) -> u32 {
        use Move::*;
        use Outcome::*;

        (match (self, other) {
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Win,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Draw,
            _ => Loss,
        }) as u32
            + *self as u32
    }

    fn points_against_if_outcome_is(other: Move, outcome: Outcome) -> u32 {
        use Move::*;
        use Outcome::*;

        (match (outcome, other) {
            (Win, Rock) | (Loss, Scissors) | (Draw, Paper) => Paper,
            (Win, Paper) | (Loss, Rock) | (Draw, Scissors) => Scissors,
            _ => Rock,
        }) as u32
            + outcome as u32
    }
}

#[derive(Copy, Clone)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl Outcome {
    fn from_str(s: &str) -> Self {
        use Outcome::*;

        match s {
            "X" => Loss,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("unknown move notation: {s}"),
        }
    }
}

pub fn score_part_one(filename: &str) -> u32 {
    crate::utils::read_lines(filename)
        .map(|line| {
            let mut moves = line.split_whitespace().map(Move::from_str);
            let op_move = moves.next().unwrap();
            let my_move = moves.next().unwrap();
            my_move.points_against(op_move)
        })
        .sum()
}

pub fn score_part_two(filename: &str) -> u32 {
    crate::utils::read_lines(filename)
        .map(|line| {
            let mut tokens = line.split_whitespace();
            let op_move = Move::from_str(tokens.next().unwrap());
            let outcome = Outcome::from_str(tokens.next().unwrap());
            Move::points_against_if_outcome_is(op_move, outcome)
        })
        .sum()
}

#[cfg(test)]
mod d02_tests {
    use super::*;

    static TASK: &str = "./inputs/day_02/task.txt";
    static EXAMPLE_1: &str = "./inputs/day_02/example_1.txt";

    #[test]
    fn part_one_example_test() {
        let res = score_part_one(EXAMPLE_1);
        assert_eq!(res, 15);
    }

    #[test]
    fn run_part_one_task() {
        let res = score_part_one(TASK);
        println!("{res}");
    }

    #[test]
    fn part_two_example_test() {
        let res = score_part_two(EXAMPLE_1);
        assert_eq!(res, 12);
    }

    #[test]
    fn run_part_two_task() {
        let res = score_part_two(TASK);
        println!("{res}");
    }
}
