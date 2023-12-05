use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Duration;
use log::info;

#[cfg(feature = "log")]
pub mod log_config;

#[cfg(feature = "log")]
pub fn init_logging() {
    log_config::init_logging(log::LevelFilter::Info, log::LevelFilter::Debug)
}

#[cfg(not(feature = "log"))]
pub mod nolog;

#[cfg(not(feature = "log"))]
pub fn init_logging() {
    nolog::init().expect("Logger not initialized");
}

pub struct Aoc {
    input: &'static str,
    example: &'static str,
}

impl Aoc {
    pub fn init(input: &'static str, example: &'static str) -> Aoc {
        init_logging();
        Aoc { input, example }
    }

    pub fn read_input_string(&self) -> &str {
        self.get_input()
    }

    pub fn input_lines(&self) -> impl Iterator<Item = &str> {
        self.get_input().lines()
    }

    pub fn read_input_lines(&self) -> Vec<&'static str> {
        self.get_input().lines().collect()
    }

    fn get_input(&self) -> &'static str {
        use std::env;
        let input = env::args()
            .nth(1)
            .unwrap_or_else(|| "input.txt".to_string());

        if input.starts_with("input") {
            self.input
        } else {
            self.example
        }
    }
}

pub fn run_with_bench<F>(input: &'static str, example: &'static str, f: &F) -> Duration
where
    F: Fn(&Aoc) -> (u32, u32),
{
    let aoc = Aoc::init(input, example);
    let now = ::std::time::Instant::now();
    let (part1, part2) = f(&aoc);
    let elapsed = now.elapsed();

    info!("Part 1: {part1}");
    info!("Part 2: {part2}");

    
        info!("Time: {:.3?}", elapsed);
    elapsed
}

fn benchmark<F>(aoc: &Aoc, f: &F) -> Duration where F: Fn(&Aoc) -> (u32, u32), {
    let now = ::std::time::Instant::now();
    let (_,_) = f(aoc);
    now.elapsed()
}

pub fn run_n_times<F>(n: usize, input: &'static str, example: &'static str, f: F)
where
    F: Fn(&Aoc) -> (u32, u32),
{
    let aoc = Aoc::init(input, example);
    let mut elapsed = benchmark(&aoc, &f);
    for _ in 1..n {
        let dur = benchmark(&aoc, &f);
        elapsed += dur;
    }
    info!("Time: {:.3?}", elapsed);
    info!("Per execution: {:.3?}", elapsed/1000)
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
