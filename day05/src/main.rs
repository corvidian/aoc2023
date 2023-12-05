use log::debug;

const INPUT: &str = include_str!("../input.txt");
const EXAMPLE: &str = include_str!("../example.txt");

fn main() {
    aoc::run_with_bench(INPUT, EXAMPLE, &|aoc| {
        let lines = aoc.read_input_lines();

        (part1(&lines), part2(&lines))
    });
}

fn part1(lines: &[&str]) -> u32 {
    let (seeds, groups) = parse_input(lines);

    seeds
        .iter()
        .map(|seed| map_seed(*seed, &groups))
        .inspect(|dest| debug!("Destination: {dest}"))
        .min()
        .unwrap()
}

fn parse_input(lines: &[&str]) -> (Vec<u32>, Vec<Vec<Vec<u32>>>) {
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
            //debug!("Seed {seed} maps to: {}", destination + seed - source);
            return destination + seed - source;
        }
    }
    //debug!("Seed stays: {seed}");
    seed
}

fn parse_numbers(list: &str) -> Vec<u32> {
    list.split_whitespace()
        .map(|n| n.parse::<u32>().expect("Not a number!"))
        .collect::<Vec<_>>()
}

fn part2(lines: &[&str]) -> u32 {
    let (seeds, groups) = parse_input(lines);

    let mut min = u32::MAX;
    seeds.chunks_exact(2).for_each(|pair| {
        let new_seeds = expand_seeds(pair);
        for seed in new_seeds {
            let mapped = map_seed(seed, &groups);
            if mapped < min {
                min = mapped
            }
        }
    });
    min
}

fn expand_seeds(orig_seeds: &[u32]) -> Vec<u32> {
    let mut new_seeds = vec![];

    for i in orig_seeds[0]..orig_seeds[0] + orig_seeds[1] {
        new_seeds.push(i);
    }
    debug!("for seeds in {:?}, {}", orig_seeds, new_seeds.len());

    new_seeds
}
