use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[allow(unused)]
pub fn read_lines<P>(filename: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("error reading file");
    io::BufReader::new(file).lines().map(Result::unwrap)
}
