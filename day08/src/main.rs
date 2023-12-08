use std::collections::HashMap;

use log::{debug, info};
use num::Integer;

const INPUT: &str = include_str!("../input.txt");
const EXAMPLE: &str = include_str!("../example.txt");

fn main() {
    aoc::run_with_bench(INPUT, EXAMPLE, &|aoc| {
        let lines = aoc.read_input_lines();

        (0, part2(&lines))
    });
}

fn part1(lines: &[&str]) -> u32 {
    let directions = lines[0];
    debug!("{directions:?}");
    let mut dirs = directions.chars().cycle();
    let nodes = lines[2..]
        .iter()
        .map(|line| (&line[0..=2], (&line[7..=9], &line[12..=14])))
        .collect::<HashMap<_, _>>();
    debug!("{nodes:?}");

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

    return steps;
}

fn part2(lines: &[&str]) -> u64 {
    let directions = lines[0];
    debug!("{directions:?}");
    let mut dirs = directions.chars().cycle();
    let nodes = lines[2..]
        .iter()
        .map(|line| (&line[0..=2], (&line[7..=9], &line[12..=14])))
        .collect::<HashMap<_, _>>();
    debug!("{nodes:?}");

    let mut steps: u64 = 0;
    let mut current = nodes
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
                debug!("{c} {next_dir:?}");
                if let Some('L') = next_dir {
                    c = nodes[c].0
                } else {
                    c = nodes[c].1
                }

                steps += 1;
            }
            steps
        })
        .inspect(|c| info!("{c}"))
        .collect::<Vec<u64>>();

    info!("{cycles:?}");

    let lcm = cycles.into_iter().reduce(|a, b| a.lcm(&b)).unwrap();
    /*
    while !current.iter().all(|c| c.ends_with('Z')) {
                let next_dir = dirs.next();
        if steps % 1000000 == 0 {debug!("{steps} {current:?} {next_dir:?}")}
        //debug!("{current:?} {next_dir:?}");
        if let Some('L') = next_dir {
            current = current.iter().map(|&c| nodes[c].0).collect::<Vec<_>>();
        } else {
            current = current.iter().map(|&c| nodes[c].1).collect::<Vec<_>>();

        }

        steps +=1;
    }


    return steps
     */
    lcm
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_with_example() {
        let lines = EXAMPLE.lines().collect::<Vec<_>>();
        assert_eq!(part1(&lines), 0);
    }

    #[test]
    fn part1_with_input() {
        let lines = INPUT.lines().collect::<Vec<_>>();
        assert_eq!(part1(&lines), 0);
    }

    #[test]
    fn part2_with_example() {
        let lines = EXAMPLE.lines().collect::<Vec<_>>();
        assert_eq!(part2(&lines), 0);
    }

    #[test]
    fn part2_with_input() {
        let lines = INPUT.lines().collect::<Vec<_>>();
        assert_eq!(part2(&lines), 0);
    }
}
