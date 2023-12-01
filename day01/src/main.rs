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
    let last_word_index = WORDS
        .keys()
        .filter_map(|word| line.match_indices(word).last())
        .max();

    let first_digit_index = line.chars().position(|c| c.is_ascii_digit());
    let first_digit = first_digit_index.map(|i| {
        line.chars()
            .nth(i)
            .map(|c| (i, c.to_digit(10).unwrap()))
            .unwrap()
    });
    let first = get_value(first_word_index, first_digit, |a, b| a < b);
    let last_digit_index = line
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .rposition(|c| c.is_ascii_digit());
    let last_digit = last_digit_index.map(|i| {
        line.chars()
            .nth(i)
            .map(|c| (i, c.to_digit(10).unwrap()))
            .unwrap()
    });
    let last = get_value(last_word_index, last_digit, |a, b| a > b);

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

fn get_value(
    first_word_index: Option<(usize, &str)>,
    first_digit: Option<(usize, u32)>,
    ordering: fn(usize, usize) -> bool,
) -> u32 {
    let first_word_index = first_word_index.map(|(i, w)| (i, WORDS[w]));
    if let Some((word_pos, word_value)) = first_word_index {
        if let Some((digit_pos, digit_value)) = first_digit {
            if ordering(word_pos, digit_pos) {
                word_value
            } else {
                digit_value
            }
        } else {
            word_value
        }
    } else if let Some((_, digit)) = first_digit {
        digit
    } else {
        panic!("No digit or number word on line.")
    }
}
