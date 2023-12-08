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

fn part1(directions: &str, nodes: &HashMap<&str, (&str, &str)>) -> u32 {
    let mut dirs = directions.chars().cycle();
    let mut steps: u32 = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        let next_dir = dirs.next();
        debug!("{current} {next_dir:?}");
        if let Some('L') = next_dir {
            current = nodes[current].0
        } else {
            current = nodes[current].1
        }
        steps += 1;
    }

    steps
}

fn part2(directions: &str, nodes: &HashMap<&str, (&str, &str)>) -> u64 {
    let current = nodes
        .keys()
        .filter(|key| key.ends_with('A'))
        .map(|a| a.to_owned())
        .collect::<Vec<_>>();
    debug!("Start values: {current:?}");

    let cycles = current
        .iter()
        .map(|&start| {
            let mut dirs = directions.chars().cycle();
            let mut c = start;
            let mut steps: u64 = 0;
            while !c.ends_with('Z') {
                let next_dir = dirs.next();
                if let Some('L') = next_dir {
                    c = nodes[c].0
                } else {
                    c = nodes[c].1
                }
                steps += 1;
            }
            steps
        })
        .inspect(|c| debug!("{c}"))
        .collect::<Vec<u64>>();

    debug!("{cycles:?}");

    cycles.into_iter().reduce(|a, b| a.lcm(&b)).unwrap()
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
