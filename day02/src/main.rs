use log::{debug, info};
use regex::Regex;

fn main() {
    aoc::init_logging();

    let lines = aoc::read_input_lines();

    part1(&lines);

    part2(&lines);
}

fn part1(lines: &[String]) {
    let maxes = lines
        .iter()
        .map(|line| parse_line(line))
        .collect::<Vec<[u32; 3]>>();
    debug! {"{maxes:?}"};

    let sum: usize = maxes
        .iter()
        .enumerate()
        .filter(|(_, [r, g, b])| *r < 13 && *g < 14 && *b < 15)
        .map(|(i, _)| i + 1)
        .sum();

    info!("Part 1: {sum}");
}

fn parse_line(line: &str) -> [u32; 3] {
    debug! {"{line}"};
    let re_red = Regex::new(r"(\d+) red").unwrap();
    let mut max_red = 0u32;
    let re_blue = Regex::new(r"(\d+) blue").unwrap();
    let mut max_blue = 0u32;
    let re_green = Regex::new(r"(\d+) green").unwrap();
    let mut max_green = 0u32;
    for (_, [red]) in re_red.captures_iter(line).map(|c| c.extract()) {
        let red = red.parse::<u32>().unwrap();
        debug!("Red: {red}");
        if red > max_red {
            max_red = red;
        }
    }
    debug!("Max Red: {max_red}");

    for (_, [green]) in re_green.captures_iter(line).map(|c| c.extract()) {
        let green = green.parse::<u32>().unwrap();
        debug!("green: {green}");
        if green > max_green {
            max_green = green;
        }
    }
    debug!("Max green: {max_green}");

    for (_, [blue]) in re_blue.captures_iter(line).map(|c| c.extract()) {
        let blue = blue.parse::<u32>().unwrap();
        debug!("blue: {blue}");
        if blue > max_blue {
            max_blue = blue;
        }
    }
    debug!("Max blue: {max_blue}");

    [max_red, max_green, max_blue]
}

fn part2(lines: &[String]) {
    let maxes = lines
        .iter()
        .map(|line| parse_line(line))
        .collect::<Vec<[u32; 3]>>();
    debug! {"Maxes: {maxes:?}"};
    let sum: u32 = maxes.iter().map(|[r, g, b]| r * g * b).sum();

    info!("Part 2: {sum}");
}
