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

const WORDS: [(u32, &str); 18] = [
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
    (1, "1"),
    (2, "2"),
    (3, "3"),
    (4, "4"),
    (5, "5"),
    (6, "6"),
    (7, "7"),
    (8, "8"),
    (9, "9"),
];

fn part2(lines: &[String]) {
    let sum = lines.iter().map(|s| parse_line_with_words(s)).sum::<u32>();
    info!("Part 2: {sum}");
}

fn parse_line_with_words(line: &str) -> u32 {
    let indexes = WORDS
        .iter()
        .flat_map(|(i, word)| line.match_indices(word).map(|w| (w.0, *i)))
        .collect::<Vec<_>>();

    let first = indexes.iter().min().expect("No digit on line").1;
    let last = indexes.iter().max().expect("No digit on line").1;

    debug!(
        "{line} first: {} last: {} indexes: {:?}",
        first, last, indexes
    );

    first * 10 + last
}
