use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[cfg(feature = "log")]
pub mod log;

#[cfg(feature = "log")]
pub fn init_logging() {
    log::init_logging(log::LevelFilter::Info, log::LevelFilter::Debug)
}

#[cfg(not(feature = "log"))]
pub mod nolog;

#[cfg(not(feature = "log"))]
pub fn init_logging() {
    nolog::init().expect("Logger not initialized");
}

pub fn read_input_lines() -> Vec<String> {
    read_lines(get_filename())
}

pub fn read_input_string() -> String {
    fs::read_to_string(get_filename()).expect("File not found!")
}

pub fn read_and_split(pattern: &str) -> (String, String) {
    let input = read_input_string();
    let a = input.split_once(pattern).expect("Split pattern not found");
    (a.0.to_owned(), a.1.to_owned())
}

fn get_filename() -> String {
    use std::env;
    env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string())
}

pub fn input_lines() -> impl Iterator<Item = String> {
    iter_lines(get_filename())
}

fn iter_lines<P>(filename: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename).expect("File not found!");
    BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error reading line"))
}

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename).expect("File not found!");
    BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error reading line"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_lines_works() {
        let result = read_lines("example.txt");
        assert_eq!(result, vec!["1", "2", "3"]);
    }
}
