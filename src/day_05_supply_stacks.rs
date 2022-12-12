use itertools::Itertools;
use std::collections::VecDeque;

pub trait CrateMover {
    fn apply_moves(stacks: &mut Vec<VecDeque<char>>, moves: impl Iterator<Item = Move>);
}

struct CrateMover9000;
impl CrateMover for CrateMover9000 {
    fn apply_moves(stacks: &mut Vec<VecDeque<char>>, moves: impl Iterator<Item = Move>) {
        for (count, from, to) in moves {
            for _ in 0..count {
                let item = stacks[from]
                    .pop_back()
                    .expect("trying to move item from empty stack");
                stacks[to].push_back(item);
            }
        }
    }
}

struct CrateMover9001;
impl CrateMover for CrateMover9001 {
    fn apply_moves(stacks: &mut Vec<VecDeque<char>>, moves: impl Iterator<Item = Move>) {
        for (count, from, to) in moves {
            let to_move: Vec<_> = (0..count)
                .map(|_| {
                    stacks[from]
                        .pop_back()
                        .expect("trying to move item from empty stack")
                })
                .collect();
            stacks[to].extend(to_move.iter().rev());
        }
    }
}

fn parse_stacks(lines: impl Iterator<Item = String>, total: usize) -> Vec<VecDeque<char>> {
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); total];

    for line in lines {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, c)| *c != ' ')
            .for_each(|(i, c)| stacks[i].push_front(c));
    }

    stacks
}

type Move = (u32, usize, usize);

fn parse_move(line: String) -> Move {
    let mut ms = line.split_whitespace().skip(1).step_by(2);
    let count = ms.next().unwrap().parse().unwrap();
    let from = ms.next().unwrap().parse::<usize>().unwrap() - 1;
    let to = ms.next().unwrap().parse::<usize>().unwrap() - 1;

    (count, from, to)
}

pub fn move_crates<CM: CrateMover>(filename: &str) -> String {
    let groups = crate::utils::read_lines(filename)
        .filter(|s| !s.is_empty())
        .group_by(|s| s.trim().chars().next());
    let mut groups_iter = groups.into_iter();

    let (k, stacks_lines) = groups_iter.next().unwrap();
    assert_eq!(k, Some('['));

    let (k, mut stack_numbers) = groups_iter.next().unwrap();
    assert_eq!(k, Some('1'));

    let stacks_count = stack_numbers.next().unwrap().split_whitespace().count();
    let mut stacks = parse_stacks(stacks_lines, stacks_count);

    let (k, moves) = groups_iter.next().unwrap();
    assert_eq!(k, Some('m'));

    CM::apply_moves(&mut stacks, moves.map(parse_move));

    stacks.iter().map(|stack| stack.back().unwrap()).join("")
}

#[cfg(test)]
mod d05_tests {
    use super::*;

    static EXAMPLE_1_FILE: &str = "./inputs/day_05/example_1.txt";
    static TASK_FILE: &str = "./inputs/day_05/task.txt";

    #[test]
    fn example_1_test() {
        let res = move_crates::<CrateMover9000>(EXAMPLE_1_FILE);
        assert_eq!(&res, "CMZ");
    }

    #[test]
    fn run_part_one() {
        let res = move_crates::<CrateMover9000>(TASK_FILE);
        println!("{res}");
    }

    #[test]
    fn example_2_test() {
        let res = move_crates::<CrateMover9001>(EXAMPLE_1_FILE);
        assert_eq!(&res, "MCD");
    }

    #[test]
    fn run_part_two() {
        let res = move_crates::<CrateMover9001>(TASK_FILE);
        println!("{res}");
    }
}
