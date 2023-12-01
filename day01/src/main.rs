use std::{collections::HashMap, ops::Index};

use log::{debug, info};

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
    let first = line
        .chars()
        .filter(|c| c.is_digit(10))
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap()
        * 10;
    let last = line
        .chars()
        .rev()
        .filter(|c| c.is_digit(10))
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap();
    first + last
}

fn part2(lines: &[String]) {
    let words: HashMap<String, u32> = HashMap::from([
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ]);

    let sum = lines
        .iter()
        .map(|s| parse_line_with_words(s, &words))
        .sum::<u32>();
    info!("Part 2: {sum}");
}

fn parse_line_with_words(line: &str, words: &HashMap<String, u32>) -> u32 {
    let first_word_index = words
        .keys()
        .filter_map(|word| line.match_indices(word).next())
        .min();
    let last_word_index = words
        .keys()
        .filter_map(|word| line.match_indices(word).last())
        .max();

    let first_digit_index = line.chars().position(|c| c.is_digit(10));
    let first_digit = first_digit_index.map(|i| {
        line.chars()
            .nth(i)
            .map(|c| (i, c.to_digit(10).unwrap()))
            .unwrap()
    });
    let first = get_value_first(first_word_index, first_digit, words);
    let last_digit_index = line
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .rposition(|c| c.is_digit(10));
    let last_digit = last_digit_index.map(|i| {
        line.chars()
            .nth(i)
            .map(|c| (i, c.to_digit(10).unwrap()))
            .unwrap()
    });
    let last = get_value_last(last_word_index, last_digit, words);

    debug!(
        "{line} first: {} {:?} {:?} last: {} {:?} {:?}",
        first,
        first_word_index.map(|w| w.0),
        first_word_index.map(|w| w.1),
        last,
        last_word_index.map(|w| w.0),
        last_word_index.map(|w| w.1)
    );

    //let last = line.chars().rev().filter(|c| c.is_digit(10)).next().unwrap().to_digit(10).unwrap();
    //first + last
    first * 10 + last
}

fn get_value_first(
    first_word_index: Option<(usize, &str)>,
    first_digit: Option<(usize, u32)>,
    words: &HashMap<String, u32>,
) -> u32 {
    let first_word_index = first_word_index.map(|(i, w)| (i, words[w]));
    if let Some(word_pos) = first_word_index.map(|w| w.0) {
        if let Some(digit_pos) = first_digit.map(|d| d.0) {
            if word_pos < digit_pos {
                first_word_index.unwrap().1
            } else {
                first_digit.unwrap().1
            }
        } else {
            first_word_index.unwrap().1
        }
    } else {
        if let Some(digit) = first_digit.map(|d| d.1) {
            digit
        } else {
            panic!("No digit or number word on line.")
        }
    }
}

fn get_value_last(
    first_word_index: Option<(usize, &str)>,
    first_digit: Option<(usize, u32)>,
    words: &HashMap<String, u32>,
) -> u32 {
    let first_word_index = first_word_index.map(|(i, w)| (i, words[w]));
    if let Some(word_pos) = first_word_index.map(|w| w.0) {
        if let Some(digit_pos) = first_digit.map(|d| d.0) {
            if word_pos > digit_pos {
                first_word_index.unwrap().1
            } else {
                first_digit.unwrap().1
            }
        } else {
            first_word_index.unwrap().1
        }
    } else {
        if let Some(digit) = first_digit.map(|d| d.1) {
            digit
        } else {
            panic!("No digit or number word on line.")
        }
    }
}
