use std::{collections::HashSet, fmt::Display};

use log::{debug, info};

#[derive(Debug)]
struct Cards {
    winning_numbers: usize,
    number_of_cards: usize,
}

impl Cards {
    pub fn new(winning_numbers: usize) -> Cards {
        Cards {
            winning_numbers,
            number_of_cards: 1,
        }
    }

    pub fn add_cards(&mut self, amount: usize) {
        self.number_of_cards += amount;
    }
}

impl Display for Cards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} cards with {} winning numbers",
            self.number_of_cards, self.winning_numbers
        )
    }
}

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

    info!("Part 1: {}", part1(&counts));
    info!("Part 2: {}", part2(&counts));
}

fn parse_numbers(list: &str) -> HashSet<u32> {
    list.trim()
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| n.parse::<u32>().expect("Not a number!"))
        .collect::<HashSet<_>>()
}

fn part1(counts: &[usize]) -> u32 {
    counts
        .iter()
        .filter(|&count| *count > 0)
        .map(|&count| 2u32.pow(count as u32 - 1))
        .sum()
}

fn part2(counts: &[usize]) -> usize {
    let mut cards: Vec<Cards> = counts
        .iter()
        .map(|&count| Cards::new(count))
        .collect::<Vec<_>>();

    for i in 0..cards.len() {
        debug!("card: {:?}", cards[i]);
        for y in i + 1..i + 1 + cards[i].winning_numbers {
            let amount = cards[i].number_of_cards;
            cards[y].add_cards(amount);
        }
    }

    cards.iter().map(|card| card.number_of_cards).sum()
}
