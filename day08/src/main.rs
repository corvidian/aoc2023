use std::collections::HashMap;

use log::debug;
use num::Integer;

const INPUT: &str = include_str!("../input.txt");
const EXAMPLE2: &str = include_str!("../example2.txt");

fn main() {
    aoc::run_with_bench(INPUT, EXAMPLE2, &|aoc| {
        let lines = aoc.read_input_lines();

        let (directions, nodes) = parse(&lines);
        debug!("{directions:?}");
        debug!("{nodes:?}");

        (part1(directions, &nodes), part2(directions, &nodes))
    });
}

fn parse<'a>(lines: &'a [&str]) -> (&'a str, HashMap<&'a str, (&'a str, &'a str)>) {
    let directions = lines[0];

    let nodes = lines[2..]
        .iter()
        .map(|line| (&line[0..=2], (&line[7..=9], &line[12..=14])))
        .collect::<HashMap<_, _>>();

    (directions, nodes)
}

fn part1(directions: &str, nodes: &HashMap<&str, (&str, &str)>) -> u64 {
    count_steps_to_z("AAA", directions, nodes)
}

fn part2(directions: &str, nodes: &HashMap<&str, (&str, &str)>) -> u64 {
    let start_keys = nodes
        .keys()
        .filter(|key| key.ends_with('A'))
        .map(|a| a)
        .collect::<Vec<_>>();
    debug!("Start values: {start_keys:?}");

    let cycles = start_keys
        .into_iter()
        .map(|start| count_steps_to_z(start, directions, nodes))
        .inspect(|c| debug!("{c}"))
        .collect::<Vec<u64>>();

    debug!("{cycles:?}");

    cycles.into_iter().reduce(|a, b| a.lcm(&b)).unwrap()
}

fn count_steps_to_z(start_key: &str, directions: &str, nodes: &HashMap<&str, (&str, &str)>) -> u64 {
    let mut dirs = directions.chars().cycle();
    let mut steps: u64 = 0;
    let mut current = start_key;
    while !current.ends_with('Z') {
        if let Some('L') = dirs.next() {
            current = nodes[current].0
        } else {
            current = nodes[current].1
        }
        steps += 1;
    }
    steps
}

#[cfg(test)]
mod tests {
    const EXAMPLE1: &str = include_str!("../example1.txt");

    use super::*;
    #[test]
    fn part1_with_example() {
        let lines = EXAMPLE1.lines().collect::<Vec<_>>();
        let (dir, nodes) = parse(&lines);
        assert_eq!(part1(dir, &nodes), 6);
    }

    #[test]
    fn part1_with_input() {
        let lines = INPUT.lines().collect::<Vec<_>>();
        let (dir, nodes) = parse(&lines);
        assert_eq!(part1(dir, &nodes), 20221);
    }

    #[test]
    fn part2_with_example() {
        let lines = EXAMPLE2.lines().collect::<Vec<_>>();
        let (dir, nodes) = parse(&lines);
        assert_eq!(part2(dir, &nodes), 6);
    }

    #[test]
    fn part2_with_input() {
        let lines = INPUT.lines().collect::<Vec<_>>();
        let (dir, nodes) = parse(&lines);
        assert_eq!(part2(dir, &nodes), 14616363770447);
    }
}
