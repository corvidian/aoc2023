use aoc::parse_numbers;
use log::debug;

const INPUT: &str = include_str!("../input.txt");
const EXAMPLE: &str = include_str!("../example.txt");

fn main() {
    aoc::run_with_bench(INPUT, EXAMPLE, &|aoc| {
        let lines = aoc.read_input_lines();
        let (seeds, groups) = parse_input(lines);
        (0, part2(&seeds, &groups))
    });
}

fn _part1(seeds: &[u32], groups: &[Vec<Vec<u32>>]) -> u32 {
    seeds
        .iter()
        .map(|seed| map_seed(*seed, groups))
        .inspect(|dest| debug!("Destination: {dest}"))
        .min()
        .unwrap()
}

fn parse_input(lines: Vec<&str>) -> (Vec<u32>, Vec<Vec<Vec<u32>>>) {
    let seeds = parse_numbers(lines[0].split_once(':').unwrap().1);
    let mut y: usize = 3;
    let mut groups = vec![];
    let mut group = vec![];
    while y < lines.len() {
        if lines[y].chars().next().is_some() {
            group.push(parse_numbers(lines[y]));
            y += 1;
        } else {
            groups.push(group);
            group = vec![];
            y += 2;
        }
    }
    groups.push(group);
    debug!("{groups:?}");
    (seeds, groups)
}

fn map_seed(seed: u32, groups: &[Vec<Vec<u32>>]) -> u32 {
    groups
        .iter()
        .fold(seed, |seed, group| map_seed_in_group(seed, group))
}

fn map_seed_in_group(seed: u32, group: &[Vec<u32>]) -> u32 {
    for rule in group {
        let destination = rule[0];
        let source = rule[1];
        let range = rule[2];
        if source <= seed && seed <= source + range {
            debug!("Seed {seed} maps to: {}", destination + seed - source);
            return destination + seed - source;
        }
    }
    debug!("Seed stays: {seed}");
    seed
}

fn part2(seeds: &[u32], groups: &[Vec<Vec<u32>>]) -> u32 {
    let mut min = u32::MAX;
    seeds.chunks_exact(2).for_each(|pair| {
        debug!("for seeds in {:?}", pair);
        for seed in pair[0]..pair[0] + pair[1] {
            let mapped = map_seed(seed, groups);
            debug!("Destination: {mapped}");

            if mapped < min {
                debug!("New min: {mapped} replaces old one {min}");
                min = mapped
            }
        }
    });
    min
}

pub fn map_seed_range_in_group(seeds: (u32, u32), group: &[Vec<u32>]) -> Vec<(u32, u32)> {
    let (seed_start, seed_end) = seeds;
    for rule in group {
        let destination = rule[0];
        let source = rule[1];
        let range = rule[2];

        if source <= seed_start && source + range >= seed_end {
            println!("Whole seed range of ({seed_start},{seed_end}) is inside rule ({source}, {range}), so moving seeds to ({},{})", destination + seed_start - source, destination + seed_end - source);
            return vec![(
                destination + seed_start - source,
                destination + seed_end - source,
            )];
        }
        if source >= seed_start && source + range <= seed_end {
            println!("Middle of seed range ({seed_start},{seed_end}) is completely inside rule ({source}, {range}), splitting to three");

            return vec![(
                destination + seed_start - source,
                destination + seed_end - source,
            )];
        }
    }
    println!(
        "Seeds ({seed_start},{seed_end}) didn't match any rules in {:?}",
        group
    );
    vec![]
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn one_step_split_whole_range_inside_rule() {
        let seeds = (79u32, 93);
        let group = vec![vec![50u32, 98, 2], vec![52, 50, 48]];

        let result = map_seed_range_in_group(seeds, &group);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], (81, 95));
    }

    #[test]
    fn map_seeds_with_middle_part_inside_rule() {
        let seeds = 55..68;
        let group = vec![vec![50u32, 60, 2]];

        let result = seeds
            .map(|seed| map_seed_in_group(seed, &group))
            .collect::<Vec<_>>();

        assert_eq!(result.len(), 13);
        assert_eq!(
            result,
            &[55, 56, 57, 58, 59, 50, 51, 52, 63, 64, 65, 66, 67]
        );
    }

    #[test]
    fn one_step_split_middle_part_inside_rule() {
        let seeds = (55, 68);
        let group = vec![vec![50u32, 60, 2]];

        let result = map_seed_range_in_group(seeds, &group);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], (81, 95));
    }
}
