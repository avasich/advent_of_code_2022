use itertools::Itertools;
use std::cell::RefCell;

type Worry = i64;

struct Monkey {
    items: RefCell<Vec<Worry>>,
    operation: Box<dyn Fn(Worry) -> Worry>,
    test_divider: Worry,
    monkey_if_true: usize,
    monkey_if_false: usize,
}

impl Monkey {
    fn inspect_items(&self) -> usize {
        self.items
            .borrow_mut()
            .iter_mut()
            .for_each(|it| *it = (self.operation)(*it) / 3);
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

#[allow(unused)]
fn keep_away(mut monkeys: Vec<Monkey>) -> usize {
    let rounds = 20;
    let mut activity = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for (i, m) in monkeys.iter().enumerate() {
            activity[i] += m.inspect_items();
            for it in m.items.borrow_mut().iter() {
                let to = m.dispatch_item(*it);
                monkeys[to].items.borrow_mut().push(*it);
            }
            m.items.borrow_mut().clear();
        }
    }

    activity
        .iter()
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .fold(1, |a, b| a * b)
}

#[allow(unused)]
fn parse_monkeys(filename: &str) -> Vec<Monkey> {
    fn take_second(line: String, delimiter: &str) -> String {
        line.split(delimiter)
            .skip(1)
            .take(1)
            .next()
            .unwrap()
            .to_string()
    }

    fn parse_items(line: String) -> Vec<Worry> {
        take_second(line, ":")
            .split(',')
            .filter(|item| !item.is_empty())
            .map(|item| item.trim().parse().unwrap())
            .collect()
    }

    fn parse_operation(line: String) -> Box<dyn Fn(Worry) -> Worry> {
        let operation_line = take_second(line, "new =");
        let operation_tokens: Vec<_> = operation_line.split_whitespace().collect();
        match operation_tokens.as_slice() {
            &["old", "+", "old"] => Box::new(|x| x + x),
            &["old", "*", "old"] => Box::new(|x| x * x),
            &["old", "+", a] | &[a, "+", "old"] => {
                let a: Worry = a.parse().unwrap();
                Box::new(move |x| x + a)
            }
            &["old", "*", a] | &[a, "*", "old"] => {
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
        .expect("error reading file")
        .map(Result::unwrap)
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
mod d11_test {
    extern crate test;
    use super::*;
    use test::Bencher;

    static TASK_FILE: &str = "./inputs/day_11/task.txt";
    static EXAMPLE_FILE_1: &str = "./inputs/day_11/part_1_example.txt";

    #[test]
    fn test_example_1() {
        let monkeys = parse_monkeys(EXAMPLE_FILE_1);
        let res = keep_away(monkeys);
        assert_eq!(res, 10605);
    }

    #[test]
    fn test_task_1() {
        let monkeys = parse_monkeys(TASK_FILE);
        let res = keep_away(monkeys);
        println!("{res}");
    }

    #[bench]
    fn bench_smth(b: &mut Bencher) {
        b.iter(|| {
            (0..10_000).for_each(|_| {

            });
        })
    }
}
