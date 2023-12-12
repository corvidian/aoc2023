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

fn count_permutations(lines: &[&str]) {
    let mut max = 0;
    let mut sum: u64 = 0;
    for line in lines {
        let count = line.chars().filter(|c| *c == '?').count();
        if count > max {
            max = count
        }
        sum += 1u64 << count;
    }
    debug!("Sum of permutations: {sum}");
    debug!("Max number of ?: {max}");
}

fn part1(lines: &[&str]) -> u64 {
    lines
        .iter()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(springs, groups)| (springs, parse_numbers(groups)))
        //.inspect(|(springs, groups)| debug!("{springs:?} {groups:?}"))
        .map(|(springs, groups)| guess_springs(springs, groups))
        .sum()
}

fn guess_springs(springs: &str, groups: Vec<usize>) -> u64 {
    let missing_indices = springs
        .char_indices()
        .filter(|(_, c)| *c == '?')
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    recursive(
        &springs.chars().collect::<Vec<_>>(),
        &missing_indices,
        &groups,
    )
}

fn recursive(springs: &[char], missing_indices: &[usize], correct_groups: &[usize]) -> u64 {
    if missing_indices.is_empty() {
        let counts = count_groups(springs);
        //debug!("springs: {} ready groups: {counts:?} correct_groups: {correct_groups:?}", springs.iter().collect::<String>());
        if counts == correct_groups {
            1
        } else {
            0
        }
    } else {
        let mut ready_groups = count_groups_until_unknown(springs);
        if ready_groups.len() > correct_groups.len() {
            return 0;
        }
        if !ready_groups.is_empty()
            && ready_groups.last().unwrap() > &correct_groups[ready_groups.len() - 1]
        {
            return 0;
        }
        ready_groups.pop();
        if ready_groups != correct_groups[..ready_groups.len()] {
            //debug!("springs: {} ready groups: {ready_groups:?} correct_groups: {correct_groups:?}", springs.iter().collect::<String>());
            return 0;
        }
        let index = missing_indices[0];
        let mut springs = springs.to_vec();
        springs[index] = '#';
        let with_broken = recursive(&springs, &missing_indices[1..], correct_groups);
        springs[index] = '.';
        let with_working = recursive(&springs, &missing_indices[1..], correct_groups);
        with_broken + with_working
    }
}

fn count_groups_until_unknown(springs: &[char]) -> Vec<usize> {
    springs
        .iter()
        .take_while(|c| **c != '?')
        //.copied()
        .group_by(|c| **c == '#')
        .into_iter()
        .filter(|(broken, _)| *broken)
        .map(|(_, a)| a.count())
        .collect()
}

fn count_groups(springs: &[char]) -> Vec<usize> {
    springs
        .iter()
        .group_by(|c| **c == '#')
        .into_iter()
        .filter(|(broken, _)| *broken)
        .map(|(_, a)| a.count())
        .collect()
}

fn parse_numbers(groups: &str) -> Vec<usize> {
    groups
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn part2(lines: &[&str]) -> u64 {
    let things = lines
        .iter()
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
        .collect::<Vec<_>>();

    let springs = things
        .iter()
        .map(|(springs, _)| &springs[..])
        .collect::<Vec<_>>();
    count_permutations(&springs);

    things
        .iter()
        .map(|(springs, groups)| (springs, parse_numbers(groups)))
        .inspect(|(springs, groups)| debug!("{springs:?} {groups:?}"))
        .map(|(springs, groups)| guess_springs(springs, groups))
        .inspect(|count| debug!("Count: {count}"))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PERFECT_RECORD: &str = include_str!("../perfect-records.txt");

    #[test]
    fn test_count_groups() {
        let pairs: Vec<(Vec<usize>, Vec<usize>)> = PERFECT_RECORD
            .lines()
            .map(|line| line.split_once(' ').unwrap())
            .map(|(springs, groups)| {
                (
                    count_groups(&springs.chars().collect::<Vec<_>>()),
                    parse_numbers(groups),
                )
            })
            .collect();

        for pair in pairs {
            assert_eq!(pair.0, pair.1)
        }
    }

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
