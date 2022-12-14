use std::cell::RefCell;

use itertools::Itertools;

type Worry = i64;

pub struct Monkey {
    items: RefCell<Vec<Worry>>,
    operation: Box<dyn Fn(Worry) -> Worry>,
    test_divider: Worry,
    monkey_if_true: usize,
    monkey_if_false: usize,
}

#[derive(Copy, Clone)]
enum WorryUpdate {
    Divide(Worry),
    Mod(Worry),
}

impl Monkey {
    fn inspect_items(&self, update: WorryUpdate) -> usize {
        match update {
            WorryUpdate::Divide(x) => self
                .items
                .borrow_mut()
                .iter_mut()
                .for_each(|it| *it = (self.operation)(*it) / x),
            WorryUpdate::Mod(m) => self
                .items
                .borrow_mut()
                .iter_mut()
                .for_each(|it| *it = (self.operation)(*it) % m),
        }
        self.items.borrow().len()
    }

    fn dispatch_item(&self, it: Worry) -> usize {
        if it % self.test_divider == 0 {
            self.monkey_if_true
        } else {
            self.monkey_if_false
        }
    }
}

pub fn keep_away(monkeys: Vec<Monkey>, divide_worry: bool, rounds: usize) -> usize {
    let worry_update = if divide_worry {
        WorryUpdate::Divide(3)
    } else {
        let base = monkeys.iter().fold(1, |acc, m| acc * m.test_divider);
        WorryUpdate::Mod(base)
    };

    let mut activity = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for (i, m) in monkeys.iter().enumerate() {
            activity[i] += m.inspect_items(worry_update);
            for &it in m.items.borrow_mut().iter() {
                let to = m.dispatch_item(it);
                monkeys[to].items.borrow_mut().push(it);
            }
            m.items.borrow_mut().clear();
        }
    }

    activity.iter().sorted_by(|a, b| b.cmp(a)).take(2).product()
}

pub fn parse_monkeys(filename: &str) -> Vec<Monkey> {
    fn take_second(line: String, delimiter: &str) -> String {
        line.split_once(delimiter)
            .map_or("", |s| s.1.trim())
            .to_string()
    }

    fn parse_items(line: String) -> Vec<Worry> {
        take_second(line, ":")
            .split_terminator(',')
            .map(|item| item.trim().parse().unwrap())
            .collect()
    }

    fn parse_operation(line: String) -> Box<dyn Fn(Worry) -> Worry> {
        let operation_tokens: Vec<_> = line.split_whitespace().rev().take(2).collect();
        match *operation_tokens.as_slice() {
            ["old", "*"] => Box::new(|x| x * x),
            [a, "+"] => {
                let a: Worry = a.parse().unwrap();
                Box::new(move |x| x + a)
            }
            [a, "*"] => {
                let a: Worry = a.parse().unwrap();
                Box::new(move |x| x * a)
            }
            _ => panic!("unknown operation '{operation_tokens:?}'"),
        }
    }

    fn parse_divider(line: String) -> Worry {
        take_second(line, "by").parse().unwrap()
    }

    fn parse_condition(line: String) -> usize {
        take_second(line, "monkey").parse().unwrap()
    }

    crate::utils::read_lines(filename)
        .filter(|l| !l.is_empty() && !l.starts_with("//"))
        .chunks(6)
        .into_iter()
        .map(|mut chunk| {
            let _header = chunk.next();
            Monkey {
                items: RefCell::new(parse_items(chunk.next().unwrap())),
                operation: parse_operation(chunk.next().unwrap()),
                test_divider: parse_divider(chunk.next().unwrap()),
                monkey_if_true: parse_condition(chunk.next().unwrap()),
                monkey_if_false: parse_condition(chunk.next().unwrap()),
            }
        })
        .collect()
}

#[cfg(test)]
mod d11_tests {
    extern crate test;

    use test::Bencher;

    use super::*;

    static TASK_FILE: &str = "./inputs/day_11/task.txt";
    static EXAMPLE_FILE_1: &str = "./inputs/day_11/example_1.txt";

    #[test]
    fn test_example_1() {
        let monkeys = parse_monkeys(EXAMPLE_FILE_1);
        let res = keep_away(monkeys, true, 20);
        assert_eq!(res, 10605);
    }

    #[test]
    fn test_task_1() {
        let monkeys = parse_monkeys(TASK_FILE);
        let res = keep_away(monkeys, true, 20);
        println!("{res}");
    }

    #[test]
    fn test_example_2() {
        let monkeys = parse_monkeys(EXAMPLE_FILE_1);
        let res = keep_away(monkeys, false, 10000);
        assert_eq!(res, 2713310158);
    }

    #[test]
    fn test_task_2() {
        let monkeys = parse_monkeys(TASK_FILE);
        let res = keep_away(monkeys, false, 10000);
        println!("{res}");
    }

    #[bench]
    fn bench_take_second_pattern(b: &mut Bencher) {
        b.iter(|| {
            (0..100_000).for_each(|_| {
                // let _a = parse_operation("Operation: new = old * 19".to_string());
            });
        })
    }
}
