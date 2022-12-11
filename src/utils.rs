use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, Lines};
use std::iter::Map;
use std::path::Path;

#[allow(unused)]
pub fn read_lines<P>(
    filename: P,
) -> Map<Lines<BufReader<File>>, fn(Result<String, Error>) -> String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("error reading file");
    io::BufReader::new(file).lines().map(Result::unwrap)
}
