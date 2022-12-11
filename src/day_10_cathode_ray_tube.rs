fn parse_cycle(cycle: String) -> Vec<i32> {
    let mut tokens = cycle.split_whitespace();
    match tokens.next() {
        Some("noop") => vec![0],
        Some("addx") => {
            let x = tokens
                .next()
                .expect("command addx should have an argument")
                .parse()
                .unwrap();
            vec![0, x]
        }
        _ => panic!("cannot parse token"),
    }
}

#[allow(unused)]
fn strengths(lines: Vec<String>) -> i32 {
    let adds = lines.into_iter().flat_map(parse_cycle);

    let mut register = 1;
    let mut res = 0;
    for (i, add) in adds.enumerate() {
        let i = 1 + i as i32;
        if (i - 20) % 40 == 0 {
            res += register * i;
        }
        register += add;
    }
    res
}

#[allow(unused)]
fn crt(lines: Vec<String>) {
    let adds = lines.into_iter().flat_map(parse_cycle);

    let mut register = 1;
    for (i, add) in adds.enumerate() {
        let px = (i % 40) as i32;

        if (px - register).abs() <= 1 {
            print!("#");
        } else {
            print!(".")
        }
        if px == 39 {
            println!();
        }
        register += add;
    }
}

#[cfg(test)]
mod d10_test {
    extern crate test;

    use test::Bencher;

    use crate::utils::read_lines;

    use super::*;

    #[test]
    fn test_example_1() {
        let lines = read_lines("./inputs/day_10/part_1_example.txt").collect();
        let res = strengths(lines);
        assert_eq!(res, 13140);
    }

    #[test]
    fn test_task_1() {
        let lines = read_lines("./inputs/day_10/task.txt").collect();
        let sum = strengths(lines);
        println!("{sum}");
    }

    #[test]
    fn test_task_2() {
        let lines = read_lines("./inputs/day_10/task.txt").collect();
        crt(lines);
    }

    #[bench]
    fn bench_1(b: &mut Bencher) {
        let lines: Vec<_> = read_lines("./inputs/day_10/task.txt").collect();

        b.iter(|| {
            (0..1000).for_each(|_| {
                let _x = strengths(lines.clone());
            })
        })
    }
}
