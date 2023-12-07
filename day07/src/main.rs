use std::str::FromStr;

use itertools::Itertools;
use log::debug;

const INPUT: &str = include_str!("../input.txt");
const EXAMPLE: &str = include_str!("../example.txt");

fn main() {
    aoc::run_with_bench(INPUT, EXAMPLE, &|aoc| {
        let lines = aoc.read_input_lines();

        (part1(&lines), part2(&lines))
    });
}

fn part1(lines: &[&str]) -> u64 {
    let mut hands = lines
        .iter()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(hand, bid)| {
            let bid = bid.parse::<u64>().expect("Bid not a number");
            (hand.parse::<Hand>().unwrap(), bid)
        })
        .inspect(|(hand, bid)| debug!("{hand:?} {bid}"))
        .collect::<Vec<_>>();
    hands.sort();

    hands
        .iter()
        .for_each(|(hand, bid)| debug!("hand: {hand:?}, bid: {bid}"));

    hands
        .iter()
        .map(|hand| hand.1)
        .enumerate()
        .inspect(|(n, bid)| debug!("{} * {} = {}", n + 1, bid, (n + 1) as u64 * bid))
        .map(|(n, bid)| ((n as u64) + 1) * bid)
        .sum()
}

#[derive(Ord, PartialEq, PartialOrd, Eq, Debug)]
enum Hand {
    HighCard(u8, u8, u8, u8, u8),
    OnePair(u8, u8, u8, u8),
    TwoPair(u8, u8, u8),
    ThreeOfAKind(u8, u8, u8),
    FullHouse(u8, u8),
    FourOfAKind(u8, u8),
    FiveOfAKind(u8),
}

#[derive(Debug)]
enum HandParseError {
    NotFiveCards,
}

impl FromStr for Hand {
    type Err = HandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        debug!("Parsing {s}");
        let mut cards = s.chars().map(get_card_value).collect::<Vec<_>>();
        cards.sort_by(|a, b| b.cmp(a));

        if cards.len() != 5 {
            return Err(HandParseError::NotFiveCards);
        }
        debug!("Parsing {cards:?}");

        let mut runs: Vec<(u8, usize)> = Vec::new();
        for (key, group) in &cards.into_iter().group_by(|card| *card) {
            runs.push((key, group.collect::<Vec<_>>().len()));
        }
        runs.sort_by(|a, b| b.1.cmp(&a.1));
        match (runs.len(), runs[0].1) {
            (1, _) => Ok(Hand::FiveOfAKind(runs[0].0)),
            (2, 4) => Ok(Hand::FourOfAKind(runs[0].0, runs[1].0)),
            (2, _) => Ok(Hand::FullHouse(runs[0].0, runs[1].0)),
            (3, 3) => Ok(Hand::ThreeOfAKind(runs[0].0, runs[1].0, runs[2].0)),
            (3, _) => Ok(Hand::TwoPair(runs[0].0, runs[1].0, runs[2].0)),
            (4, _) => Ok(Hand::OnePair(runs[0].0, runs[1].0, runs[2].0, runs[3].0)),
            _ => Ok(Hand::HighCard(
                runs[0].0, runs[1].0, runs[2].0, runs[3].0, runs[4].0,
            )),
        }
    }
}

fn get_card_value(c: char) -> u8 {
    if c.is_ascii_digit() {
        c.to_digit(10).unwrap() as u8
    } else {
        match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => panic!("Card not found {c}"),
        }
    }
}

fn part2(_lines: &[&str]) -> u64 {
    0
}