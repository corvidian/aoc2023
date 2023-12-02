use log::{debug, info};
use regex::Regex;

fn main() {
    aoc::init_logging();

    let maxes = aoc::input_lines()
        .map(|line| parse_line(&line))
        .collect::<Vec<[u32; 3]>>();
    debug! {"Maxes: {maxes:?}"};

    info!("Part 1: {}", part1(&maxes));
    info!("Part 2: {}", part2(&maxes));
}

fn part1(maxes: &[[u32; 3]]) -> usize {
    maxes
        .iter()
        .enumerate()
        .filter(|(_, [r, g, b])| *r <= 12 && *g <= 13 && *b <= 14)
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(maxes: &[[u32; 3]]) -> u32 {
    maxes.iter().map(|[r, g, b]| r * g * b).sum()
}

fn parse_line(line: &str) -> [u32; 3] {
    debug! {"{line}"};
    let max_red = find_max_for_color(line, "red");
    let max_blue = find_max_for_color(line, "blue");
    let max_green = find_max_for_color(line, "green");

    [max_red, max_green, max_blue]
}

fn find_max_for_color(line: &str, color: &str) -> u32 {
    let formatted = format!(r"(\d+) {color}");
    let re = Regex::new(formatted.as_str()).unwrap();

    re.captures_iter(line)
        .map(|c| c.extract())
        .map(|(_, [red])| red.parse::<u32>().unwrap())
        .max()
        .expect("No color {color} on line")
}
