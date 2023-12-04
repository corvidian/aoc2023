use std::collections::HashSet;

use log::info;

fn main() {
    aoc::init_logging();

    let lines = aoc::read_input_lines();

    let counts = lines
        .iter()
        .map(|line| line.split_once(':').unwrap().1)
        .map(|line| line.split_once('|').unwrap())
        .map(|(winning, having)| (parse_numbers(winning), parse_numbers(having)))
        .map(|(winning, having)| having.intersection(&winning).count())
        .collect::<Vec<_>>();

    part1(&counts);
    part2(&counts);
}

fn parse_numbers(list: &str) -> HashSet<u32> {
    list.trim()
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| n.parse::<u32>().expect("Not a number!"))
        .collect::<HashSet<_>>()
}

fn part1(counts: &[usize]) {
    let cards: u32 = counts
        .iter()
        .filter(|&count| *count > 0)
        .map(|&count| 2u32.pow(count as u32 - 1))
        .sum();

    info!("Part 1: {cards}");
}

fn part2(counts: &[usize]) {
    let mut cards: Vec<(usize, usize)> = counts.iter().map(|&count| (count, 1)).collect::<Vec<_>>();

    for i in 0..cards.len() {
        for y in i + 1..i + 1 + cards[i].0 {
            cards[y] = (cards[y].0, cards[y].1 + cards[i].1)
        }
    }

    let sum: usize = cards.iter().map(|card| card.1).sum();
    info!("Part 2: {sum}");
}
