use log::debug;
use std::{collections::HashSet, fmt::Display};

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

const INPUT: &str = include_str!("../input.txt");
const EXAMPLE: &str = include_str!("../example.txt");

fn main() {
    aoc::run_with_bench(INPUT,EXAMPLE,&|aoc| {
        let counts = counts(aoc.read_input_string());
        let part1 = part1(&counts);
        let part2 = part2(&counts);
        (part1, part2 as u32)
    });

    // aoc::run_n_times(1000, INPUT, EXAMPLE, |aoc| {
    //     let counts = counts(aoc.read_input_string());
    //     let part1 = part1(&counts);
    //     let part2 = part2(&counts);
    //     (part1, part2 as u32)
    // });
}

fn counts(input: &str) -> Vec<usize> {
    input.lines().map(count_wins).collect::<Vec<_>>()
}

fn count_wins(line: &str) -> usize {
    let numbers = line.split_once(':').unwrap().1;
    let (winning, having) = numbers.split_once('|').unwrap();
    parse_numbers(having)
        .intersection(&parse_numbers(winning))
        .count()
}

fn parse_numbers(list: &str) -> HashSet<u32> {
    list.split_whitespace()
        .map(|n| n.parse::<u32>().expect("Not a number!"))
        .collect::<HashSet<_>>()
}

pub fn part1(counts: &[usize]) -> u32 {
    counts
        .iter()
        .filter_map(|&count| 2u32.checked_pow(count as u32 - 1))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_with_example() {
        let counts = counts(EXAMPLE);
        let result = part1(&counts);
        assert_eq!(result, 13);
    }

    #[test]
    fn part2_with_example() {
        let counts = counts(EXAMPLE);
        let result = part2(&counts);
        assert_eq!(result, 30);
    }

    #[test]
    fn part1_with_input() {
        let counts = counts(INPUT);
        let result = part1(&counts);
        assert_eq!(result, 28538);
    }

    #[test]
    fn part2_with_input() {
        let counts = counts(INPUT);
        let result = part2(&counts);
        assert_eq!(result, 9425061);
    }
}
