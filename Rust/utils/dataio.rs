use std::fs::File;
use std::io::{self, *, BufReader};
use std::path::Path;
use std::str::FromStr;
use std::fmt::Debug;


fn read_lines<T> (filename: T) 
    -> io::Result<io::Lines<io::BufReader<File>>>
    where T: AsRef<Path>, {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

pub fn read_data<T> (filename:&str) -> Vec<T> 
    where T: FromStr,
    <T as FromStr>::Err: Debug, {
    let mut rv: Vec<T> = vec![];
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            rv.push(line.parse().unwrap());
        }
    }
    rv
}