use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

type Coordinate = (i32, i32);

#[derive(Debug)]
struct Point {
    xy: Coordinate,
    height: u32,
    distance: RefCell<u32>,
}

impl Point {
    fn new(h: char, c: Coordinate) -> Self {
        let height = match h {
            'a'..='z' => u32::from(h) - u32::from('a'),
            'S' => 0,
            'E' => u32::from('z') - u32::from('a'),
            _ => panic!("unknown height notation '{h}'"),
        };

        Self {
            xy: c,
            height,
            distance: RefCell::new(u32::MAX),
        }
    }
}

struct Map {
    map: HashMap<Coordinate, Point>,
    start: Coordinate,
    end: Coordinate,
}

impl Map {
    fn from_file(filename: &str) -> Self {
        let mut map = HashMap::new();
        let mut start = None;
        let mut end = None;

        crate::utils::read_lines(filename)
            .enumerate()
            .for_each(|(y, line)| {
                line.chars().enumerate().for_each(|(x, h)| {
                    let c = (x as i32, y as i32);
                    map.insert(c, Point::new(h, c));
                    if h == 'S' {
                        start = Some(c);
                    } else if h == 'E' {
                        end = Some(c);
                    }
                });
            });

        Self {
            map,
            start: start.expect("map should have a start"),
            end: end.expect("map should have an end"),
        }
    }

    fn adjacent(&self, (x, y): Coordinate) -> impl Iterator<Item = &Point> {
        [(x - 1, y), (x, y + 1), (x, y - 1), (x + 1, y)]
            .map(|(x, y)| self.map.get(&(x, y)))
            .into_iter()
            .flatten()
    }

    fn accessible_from(&self, c: Coordinate) -> impl Iterator<Item = &Point> {
        let point = self.map.get(&c).unwrap();
        self.adjacent(point.xy)
            .filter(|p| p.height <= point.height + 1)
    }

    fn find_path(&mut self, from: Coordinate) -> Option<u32> {
        self.map.iter().for_each(|(_, p)| {
            *p.distance.borrow_mut() = u32::MAX;
        });

        let start = self.map.get(&from).unwrap();
        *start.distance.borrow_mut() = 0;
        let mut q = VecDeque::from([start]);

        while let Some(point) = q.pop_front() {
            if point.xy == self.end {
                return Some(*point.distance.borrow());
            }

            let adjacent = self.accessible_from(point.xy);
            let dist = *point.distance.borrow();

            for p in adjacent {
                let d = *p.distance.borrow();
                if d > dist + 1 {
                    *p.distance.borrow_mut() = dist + 1;
                    q.push_back(p);
                }
            }
        }
        None
    }

    fn find_shortest_path(&mut self) -> Option<u32> {
        let starts: Vec<_> = self
            .map
            .iter()
            .filter(|(_, p)| p.height == 0)
            .map(|(c, _)| *c)
            .collect();

        starts.into_iter().flat_map(|c| self.find_path(c)).min()
    }
}

pub fn path_from_start(filename: &str) -> Option<u32> {
    let mut map = Map::from_file(filename);
    map.find_path(map.start)
}

pub fn shortest_path(filename: &str) -> Option<u32> {
    let mut map = Map::from_file(filename);
    map.find_shortest_path()
}

#[cfg(test)]
mod d05_tests {
    use super::*;

    static EXAMPLE_MY_FILE: &str = "./inputs/day_12/example_my.txt";
    static EXAMPLE_1_FILE: &str = "./inputs/day_12/example_1.txt";
    static TASK_FILE: &str = "./inputs/day_12/task.txt";

    #[test]
    fn run_my_example() {
        let res = path_from_start(EXAMPLE_MY_FILE);
        println!("{res:?}");
    }

    #[test]
    fn example_1_test() {
        let res = path_from_start(EXAMPLE_1_FILE);
        assert_eq!(res, Some(31));
    }

    #[test]
    fn run_part_one() {
        let res = path_from_start(TASK_FILE);
        println!("{res:?}");
    }

    #[test]
    fn example_2_test() {
        let res = shortest_path(EXAMPLE_1_FILE);
        assert_eq!(res, Some(29));
    }

    #[test]
    fn run_part_two() {
        let res = shortest_path(TASK_FILE);
        println!("{res:?}");
    }
}
