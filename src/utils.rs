use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[allow(unused)]
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[allow(unused)]
pub fn read_lines_vec<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    read_lines(filename).map(|lines| lines.map(Result::unwrap).collect())
}
