use log::{debug, info};
use phf::phf_map;

fn main() {
    aoc::init_logging();

    let lines = aoc::read_input_lines();

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &[String]) {
    let sum = lines.iter().map(|s| parse_line(&s)).sum::<u32>();
    info!("Part 1: {sum}");
}

fn parse_line(line: &str) -> u32 {
    let first = first_digit(line.chars());
    let last = first_digit(line.chars().rev());
    first * 10 + last
}

fn first_digit(mut chars: impl Iterator<Item = char>) -> u32 {
    chars
        .find(|c| c.is_ascii_digit())
        .and_then(|c| c.to_digit(10))
        .unwrap()
}

static WORDS: phf::Map<&'static str, u32> = phf_map! {
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
};

const DIGITS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn part2(lines: &[String]) {
    let sum = lines.iter().map(|s| parse_line_with_words(&s)).sum::<u32>();
    info!("Part 2: {sum}");
}

fn parse_line_with_words(line: &str) -> u32 {
    let mut word_indexes = WORDS
        .keys()
        .flat_map(|word| line.match_indices(word))
        .map(|word| (word.0, WORDS[word.1]))
        .collect::<Vec<_>>();
    let mut digit_indexes = line
        .match_indices(DIGITS)
        .map(|digit| (digit.0, digit.1.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();
    word_indexes.append(&mut digit_indexes);

    let first = word_indexes.iter().min().expect("No digit on line").1;
    let last = word_indexes.iter().max().expect("No digit on line").1;

    debug!("{line} first: {} last: {} ", first, last,);

    first * 10 + last
}
