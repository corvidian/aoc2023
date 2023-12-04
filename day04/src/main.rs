use std::collections::BTreeSet;

use log::{debug, info};

fn main() {
    aoc::init_logging();

    let lines = aoc::read_input_lines();

    part1(&lines);

    part2(&lines);
}

fn part1(lines: &[String]) {
    let cards: u32 = lines
        .iter()
        .map(|line| line.split_once(':').unwrap().1)
        .map(|line| line.split_once('|').unwrap())
        .map(|(winning, having)| (parse_numbers(winning), parse_numbers(having)))
        .map(|(winning, having)| having.intersection(&winning).count())
        .filter(|count| *count > 0)
        .map(|count| 2u32.pow(count as u32 - 1))
        .sum();

    info!("Part 1: {}", cards);
}

fn parse_numbers(list: &str) -> BTreeSet<u32> {
    list.trim()
        .split(' ')
        .filter(|n| *n != "")
        .map(|n| n.parse::<u32>().expect("Not a number!"))
        .collect::<BTreeSet<_>>()
}

fn part2(lines: &[String]) {
    let mut cards: Vec<(usize, usize)> = lines
        .iter()
        .map(|line| line.split_once(':').unwrap().1)
        .map(|line| line.split_once('|').unwrap())
        .map(|(winning, having)| (parse_numbers(winning), parse_numbers(having)))
        .map(|(winning, having)| having.intersection(&winning).count())
        .map(|count| (count, 1))
        .collect::<Vec<_>>();

    for i in 0..cards.len() {
        debug!("i: {i}");
        for _ in 0..cards[i].1 {
            if cards[i].0 > 0 {
                for y in i + 1..i + 1 + cards[i].0 {
                    cards[y] = (cards[y].0, cards[y].1 + 1)
                }
            }
        }
    }

    println!("{cards:?}");

    let sum: usize = cards.iter().map(|card| card.1).sum();
    info!("Part 2: {sum}");
}
