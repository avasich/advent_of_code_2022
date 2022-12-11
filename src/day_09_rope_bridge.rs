use std::collections::HashSet;

use crate::utils::read_lines;

type Point = (i32, i32);

fn parse_move(line: String) -> (String, u8) {
    let mut tokens = line.split_whitespace();
    let dir = tokens.next().unwrap().to_owned();
    let steps = tokens
        .next()
        .unwrap()
        .parse()
        .expect("unable to parse number of steps");
    (dir, steps)
}

fn move_closer(point: Point, move_to: Point) -> Point {
    let (dx, dy) = (move_to.0 - point.0, move_to.1 - point.1);
    if dx.abs() > 1 || dy.abs() > 1 {
        (point.0 + dx.signum(), point.1 + dy.signum())
    } else {
        point
    }
}

fn move_head((x, y): Point, dir: &str) -> Point {
    match dir {
        "L" => (x - 1, y),
        "R" => (x + 1, y),
        "U" => (x, y + 1),
        "D" => (x, y - 1),
        _ => panic!("unable to parse direction {dir}"),
    }
}

pub fn rope(filename: &str, length: usize) -> usize {
    let moves = read_lines(filename).map(parse_move);

    let mut visited = HashSet::new();
    let mut knots = vec![(0, 0); length];
    visited.insert(*knots.last().unwrap());

    for (dir, n) in moves {
        for _ in 0..n {
            knots[0] = move_head(knots[0], &dir);
            for i in 1..length {
                knots[i] = move_closer(knots[i], knots[i - 1]);
            }
            visited.insert(*knots.last().unwrap());
        }
    }

    visited.len()
}

#[cfg(test)]
mod d09_tests {
    extern crate test;

    use test::Bencher;

    use super::*;

    static BENCH_SIZE: usize = 50;
    static TASK_FILE: &str = "./inputs/day_09/task.txt";
    static EXAMPLE_FILE_1: &str = "./inputs/day_09/example_1.txt";
    static EXAMPLE_FILE_2: &str = "./inputs/day_09/example_2.txt";

    #[test]
    fn run_part_one_example() {
        let res = rope(EXAMPLE_FILE_1, 2);
        assert_eq!(res, 13);
    }

    #[test]
    fn run_part_one_task() {
        let res = rope(TASK_FILE, 2);
        println!("{res}");
    }

    #[test]
    fn run_part_two_example() {
        let res = rope(EXAMPLE_FILE_2, 10);
        assert_eq!(res, 36);
    }

    #[test]
    fn run_part_two_task() {
        let res = rope(TASK_FILE, 10);
        println!("{res}");
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        b.iter(|| {
            (0..BENCH_SIZE).for_each(|_| {
                let _x = rope(TASK_FILE, 10);
            })
        })
    }
}
