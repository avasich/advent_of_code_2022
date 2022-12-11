use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, Lines};
use std::iter::Map;
use std::path::Path;

type InputLinesIterator = Map<Lines<BufReader<File>>, fn(Result<String, Error>) -> String>;

#[allow(unused)]
pub fn read_lines<P>(filename: P) -> InputLinesIterator
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("error reading file");
    io::BufReader::new(file).lines().map(Result::unwrap)
}
