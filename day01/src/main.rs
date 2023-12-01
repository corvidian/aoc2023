use std::cmp::Ordering;

use log::{debug, info};
use phf::phf_map;

fn main() {
    aoc::init_logging();

    let lines = aoc::read_input_lines();

    part1(&lines);

    part2(&lines);
}

fn part1(lines: &[String]) {
    let sum = lines.iter().map(|s| parse_line(s)).sum::<u32>();
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

fn part2(lines: &[String]) {
    let sum = lines.iter().map(|s| parse_line_with_words(s)).sum::<u32>();
    info!("Part 2: {sum}");
}

fn parse_line_with_words(line: &str) -> u32 {
    let first_word_index = WORDS
        .keys()
        .filter_map(|word| line.match_indices(word).next())
        .min();
    let first_digit_index = line
        .chars()
        .position(|c| c.is_ascii_digit())
        .expect("No digit on line");
    let first_digit = get_digit(line, first_digit_index);
    let first = get_value(first_word_index, first_digit, Ordering::Less);

    let last_word_index = WORDS
        .keys()
        .filter_map(|word| line.match_indices(word).last())
        .max();
    let last_digit_index = line
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .rposition(|c| c.is_ascii_digit())
        .expect("No digit on line");
    let last_digit = get_digit(line, last_digit_index);
    let last = get_value(last_word_index, last_digit, Ordering::Greater);

    debug!(
        "{line} first: {} {:?} {:?} last: {} {:?} {:?}",
        first,
        first_word_index.map(|w| w.0),
        first_word_index.map(|w| w.1),
        last,
        last_word_index.map(|w| w.0),
        last_word_index.map(|w| w.1)
    );

    first * 10 + last
}

fn get_digit(line: &str, digit_index: usize) -> (usize, u32) {
    line.chars()
        .nth(digit_index)
        .map(|c| (digit_index, c.to_digit(10).unwrap()))
        .unwrap()
}

fn get_value(word: Option<(usize, &str)>, digit: (usize, u32), ordering: Ordering) -> u32 {
    word.filter(|(i, _)| i.cmp(&digit.0) == ordering)
        .map(|(_, value)| WORDS[value])
        .unwrap_or(digit.1)
}
