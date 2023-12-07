use itertools::Itertools;
use log::debug;
use std::cmp::Ord;
use std::fmt::Debug;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");
const EXAMPLE: &str = include_str!("../example.txt");

fn main() {
    aoc::run_with_bench(INPUT, EXAMPLE, &|aoc| {
        let lines = aoc.read_input_lines();

        (
            get_sum_of_bids::<HandWithJacks>(&lines),
            get_sum_of_bids::<HandWithJokers>(&lines),
        )
    });
}

fn get_sum_of_bids<T>(lines: &[&str]) -> u64
where
    T: Hand + FromStr + Debug + Ord,
    <T as FromStr>::Err: Debug,
{
    debug!("Start of new part");
    let mut hands = lines
        .iter()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(hand, bid)| {
            let bid = bid.parse::<u64>().expect("Bid not a number");
            (hand.parse::<T>().unwrap(), bid)
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
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

trait Hand {
    fn new(rank: HandRank, cards: Vec<u8>) -> Self;
    fn get_card_value(c: char) -> Result<u8, HandParseError>;
}

#[derive(Ord, PartialEq, PartialOrd, Eq, Debug)]
struct HandWithJokers(HandRank, Vec<u8>);

impl Hand for HandWithJokers {
    fn new(rank: HandRank, cards: Vec<u8>) -> Self {
        Self(rank, cards)
    }

    fn get_card_value(c: char) -> Result<u8, HandParseError> {
        get_card_value(c, 1)
    }
}

impl FromStr for HandWithJokers {
    type Err = HandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = from_str(s, Self::get_card_value)?;
        Ok(Self(result.0, result.1))
    }
}

#[derive(Ord, PartialEq, PartialOrd, Eq, Debug)]
struct HandWithJacks(HandRank, Vec<u8>);
impl Hand for HandWithJacks {
    fn new(rank: HandRank, cards: Vec<u8>) -> Self {
        Self(rank, cards)
    }

    fn get_card_value(c: char) -> Result<u8, HandParseError> {
        get_card_value(c, 11)
    }
}

impl FromStr for HandWithJacks {
    type Err = HandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = from_str(s, Self::get_card_value)?;
        Ok(Self(result.0, result.1))
    }
}

impl HandRank {
    fn get_rank(sorted_cards: &[u8]) -> HandRank {
        let mut runs: Vec<usize> = Vec::new();
        for (_, group) in &sorted_cards.iter().group_by(|card| *card) {
            runs.push(group.count());
        }
        runs.sort_by(|a, b| b.cmp(&a));
        match (runs.len(), runs[0]) {
            (1, _) => HandRank::FiveOfAKind,
            (2, 4) => HandRank::FourOfAKind,
            (2, _) => HandRank::FullHouse,
            (3, 3) => HandRank::ThreeOfAKind,
            (3, _) => HandRank::TwoPair,
            (4, _) => HandRank::OnePair,
            _ => HandRank::HighCard,
        }
    }
}

#[derive(Debug)]
enum HandParseError {
    NotFiveCards,
    CardParseError(&'static str),
}

fn get_card_value(c: char, j_value: u8) -> Result<u8, HandParseError> {
    match (c.is_ascii_digit(), c) {
        (true, _) => Ok(c.to_digit(10).unwrap() as u8),
        (false, 'A') => Ok(14),
        (false, 'K') => Ok(13),
        (false, 'Q') => Ok(12),
        (false, 'J') => Ok(j_value),
        (false, 'T') => Ok(10),
        _ => Err(HandParseError::CardParseError("Card not found {c}")),
    }
}

fn from_str(
    s: &str,
    card_parser: fn(char) -> Result<u8, HandParseError>,
) -> Result<(HandRank, Vec<u8>), HandParseError> {
    debug!("Parsing {s}");
    let cards = s.chars().map(card_parser).collect::<Result<Vec<_>, _>>()?;
    if cards.len() != 5 {
        return Err(HandParseError::NotFiveCards);
    }

    let mut sorted_cards = cards.clone();
    sorted_cards.sort();

    debug!("Parsing {sorted_cards:?}");

    let jokers = sorted_cards.iter().take_while(|c| **c == 1).count();
    debug!("Number of jokers: {jokers}");

    let rank = match jokers {
        1 => find_highest_for_joker_position(&sorted_cards, &sorted_cards[jokers..], 0),
        2 => {
            let mut highest = HandRank::HighCard;
            for card1 in &sorted_cards[jokers..] {
                let mut replaced1 = sorted_cards.clone();
                replaced1[0] = *card1;
                let rank = find_highest_for_joker_position(&replaced1, &sorted_cards[jokers..], 1);
                if rank > highest {
                    highest = rank;
                }
            }
            highest
        }
        3 => {
            if sorted_cards[3] == sorted_cards[4] {
                HandRank::FiveOfAKind
            } else {
                HandRank::FourOfAKind
            }
        }
        4 | 5 => HandRank::FiveOfAKind,
        _ => HandRank::get_rank(&sorted_cards),
    };
    Ok((rank, cards))
}

fn find_highest_for_joker_position(
    sorted_cards: &[u8],
    non_jokers: &[u8],
    joker_pos: usize,
) -> HandRank {
    non_jokers
        .iter()
        .map(|card| {
            let mut replaced = sorted_cards.to_vec();
            replaced[joker_pos] = *card;
            replaced.sort();
            HandRank::get_rank(&replaced)
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_with_example() {
        let lines = EXAMPLE.lines().collect::<Vec<_>>();
        assert_eq!(get_sum_of_bids::<HandWithJacks>(&lines), 6440);
    }

    #[test]
    fn part1_with_input() {
        let lines = INPUT.lines().collect::<Vec<_>>();
        assert_eq!(get_sum_of_bids::<HandWithJacks>(&lines), 247823654);
    }

    #[test]
    fn part2_with_example() {
        let lines = EXAMPLE.lines().collect::<Vec<_>>();
        assert_eq!(get_sum_of_bids::<HandWithJokers>(&lines), 5905);
    }

    #[test]
    fn part2_with_input() {
        let lines = INPUT.lines().collect::<Vec<_>>();
        assert_eq!(get_sum_of_bids::<HandWithJokers>(&lines), 245461700);
    }
}
