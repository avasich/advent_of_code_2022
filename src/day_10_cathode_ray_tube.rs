enum Command {
    Noop,
    Addx(i32),
}

fn parse_cycle(cycle: String) -> Command {
    let mut tokens = cycle.split_whitespace();
    match tokens.next() {
        Some("noop") => Command::Noop,
        Some("addx") => {
            let x = tokens
                .next()
                .expect("command addx should have an argument")
                .parse()
                .unwrap();
            Command::Addx(x)
        }
        _ => panic!("cannot parse token"),
    }
}

#[allow(unused)]
fn strengths(lines: Vec<String>) -> Vec<i32> {
    let adds = lines.into_iter().map(parse_cycle).flat_map(|c| match c {
        Command::Noop => vec![0],
        Command::Addx(x) => vec![0, x],
    });
    let mut register = 1;
    let mut res = vec![];
    for (i, add) in adds.enumerate() {
        let i = 1 + i as i32;
        if (i as i32 - 20) % 40 == 0 {
            res.push(register * i as i32);
        }
        register += add;
    }
    res
}

#[cfg(test)]
mod d10_test {
    use crate::utils::read_lines_vec;

    use super::*;

    #[test]
    fn test_example_1() {
        let lines = read_lines_vec("./inputs/day_10/part_1_example.txt")
            .expect("unable to read first example input");
        let res = strengths(lines);
        assert_eq!(res, vec![420, 1140, 1800, 2940, 2880, 3960]);
    }

    #[test]
    fn test_task_1() {
        let lines =
            read_lines_vec("./inputs/day_10/task.txt").expect("unable to read first example input");
        let res = strengths(lines);
        let sum: i32 = res.iter().sum();
        println!("{sum}");
    }
}
