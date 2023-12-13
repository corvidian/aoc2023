use itertools::Itertools;
use log::{debug, info};
use rayon::prelude::*;
use std::usize;

const INPUT: &str = include_str!("../input.txt");
const EXAMPLE: &str = include_str!("../example.txt");

fn main() {
    aoc::run_with_bench(INPUT, EXAMPLE, &|aoc| {
        let lines = aoc.read_input_lines();

        (part1(&lines), part2(&lines))
    });
}

fn part1(lines: &[&str]) -> u64 {
    let result = lines
        .par_iter()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(springs, groups)| (springs, parse_numbers(groups)))
        //.inspect(|(springs, groups)| debug!("{springs:?} {groups:?}"))
        .map(|(springs, groups)| guess_springs(springs, &groups))
        //.inspect(|count| debug!("Count: {count}"))
        .sum();
    info!("Part 1: {result}");
    result
}

fn part2(lines: &[&str]) -> u64 {
    lines
        .par_iter()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(springs, groups)| {
            (
                (springs.to_owned() + "?")
                    .repeat(5)
                    .chars()
                    .dropping_back(1)
                    .collect::<String>(),
                (groups.to_owned() + ",")
                    .repeat(5)
                    .chars()
                    .dropping_back(1)
                    .collect::<String>(),
            )
        })
        .map(|(springs, groups)| (springs, parse_numbers(&groups)))
        .inspect(|(springs, groups)| debug!("{springs:?} {groups:?}"))
        .map(|(_springs, _groups)| guess_springs(&_springs, &_groups)) // guess_springs(&springs, &groups))
        .inspect(|count| debug!("Count: {count}"))
        .sum()
}

fn guess_springs(springs: &str, groups: &[usize]) -> u64 {
    let size = springs.len();
    let empties = size - groups.iter().sum::<usize>();
    (0..empties)
        .filter(|i| !springs[0..*i].contains('#'))
        .map(|i| recursive(springs, groups, &mut vec![i], i, i, &empties))
        .sum()
}

fn recursive(
    springs: &str,
    groups: &[usize],
    spaces: &[usize],
    number_of_spaces: usize,
    number_of_all_springs: usize,
    empties: &usize,
) -> u64 {
    //debug!("Recursive: springs: {springs}, groups: {groups:?}, spaces: {spaces:?}, number_of_spaces: {number_of_spaces}, number_of_all_springs: {number_of_all_springs}, empties: {empties}");

    if spaces.len() == groups.len() {
        let reconstructed = (0..spaces.len())
            .flat_map(|i| vec![vec!['.'; spaces[i]], vec!['#'; groups[i]]])
            .flatten()
            .collect::<Vec<char>>();

        let matches = springs.chars().enumerate().all(|(i, c)| match c {
            '?' => true,
            '#' => reconstructed.len() > i && reconstructed[i] == '#',
            '.' => reconstructed.len() <= i || reconstructed[i] == '.',
            _ => panic!("Unknown spring {c}"),
        });

        //debug!("matches: {matches}, reconstructed {reconstructed:?}");

        if matches {
            return 1;
        } else {
            return 0;
        }

        //return 1;
    }
    let next_group = groups[spaces.len() - 1];
    if springs[number_of_all_springs..number_of_all_springs + next_group].contains('.') {
        /*
        let reconstructed = (0..spaces.len())
            .flat_map(|i| vec![vec!['.'; spaces[i]], vec!['#'; groups[i]]])
            .flatten()
            .collect::<Vec<char>>();
        debug!("{reconstructed:?}");

        debug!("1st branch Pruning branches starting with {spaces:?} + {i}");
        debug!(
            "{:?},{}",
            number_of_all_springs..number_of_all_springs + next_group,
            &springs[number_of_all_springs..=number_of_all_springs + i]
        );
        */
        return 0;
    }
    let mut spaces = spaces.to_vec();
    (1..=(empties - number_of_spaces))
        .map(|i| {
            if springs[number_of_all_springs + next_group..number_of_all_springs + next_group + i]
                .contains('#')
            {
                /*

                                let reconstructed = (0..spaces.len())
                                .flat_map(|i| vec![vec!['.'; spaces[i]], vec!['#'; groups[i]]])
                                .flatten()
                                .collect::<Vec<char>>();
                            debug!("{reconstructed:?}");
                                debug!("2nd branch Pruning branches starting with {spaces:?} + {i}");
                                debug!(
                                    "{}, {}, {:?},{}",
                                    (number_of_all_springs + next_group),
                                    number_of_all_springs + next_group + i,
                                    number_of_all_springs + next_group..number_of_all_springs + next_group + i,
                                    &springs[number_of_all_springs + next_group
                                        ..number_of_all_springs + next_group + i]
                                );
                */
                0
            } else {
                spaces.push(i);
                let a = recursive(
                    springs,
                    groups,
                    &spaces,
                    number_of_spaces + i,
                    number_of_all_springs + i + groups[spaces.len() - 2],
                    empties,
                );
                spaces.pop();
                a
            }
        })
        .sum()
}

fn parse_numbers(groups: &str) -> Vec<usize> {
    groups
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PERFECT_RECORD: &str = include_str!("../perfect-records.txt");

    #[test]
    fn part1_with_example() {
        let lines = EXAMPLE.lines().collect::<Vec<_>>();
        assert_eq!(part1(&lines), 21);
    }

    #[test]
    fn part1_with_input() {
        let lines = INPUT.lines().collect::<Vec<_>>();
        assert_eq!(part1(&lines), 6852);
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
