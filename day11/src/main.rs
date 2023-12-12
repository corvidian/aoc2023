use std::{collections::HashSet, hash::Hash, usize};

use log::debug;

const INPUT: &str = include_str!("../input.txt");
const EXAMPLE: &str = include_str!("../example.txt");

const MULTIPLIER: usize = 1000000;

fn main() {
    aoc::run_with_bench(INPUT, EXAMPLE, &|aoc| {
        let mut lines = aoc
            .input_lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let columns = (0..lines[0].len()).collect::<HashSet<usize>>();
        let positions = lines
            .iter()
            .flat_map(|line| {
                line.iter()
                    .enumerate()
                    .filter(|(_, c)| **c == '#')
                    .map(|(i, _)| i)
            })
            .collect::<HashSet<_>>();
        let empty_columns = columns
            .difference(&positions)
            .cloned()
            .collect::<HashSet<_>>();
        debug!("empty_columns: {empty_columns:?}");

        let rows = (0..lines[0].len()).collect::<HashSet<usize>>();
        let positions = lines
            .iter()
            .enumerate()
            .filter(|(_, line)| line.contains(&'#'))
            .map(|(i, _)| i)
            .collect::<HashSet<_>>();
        let empty_rows = rows.difference(&positions).cloned().collect::<HashSet<_>>();
        debug!("empty_rows: {empty_rows:?}");

        debug!("{}", visualize(&lines));

        (
            part1(&mut lines.clone(), &empty_columns, &empty_rows),
            part2(&mut lines, &empty_columns, &empty_rows),
        )
    });
}

fn visualize(map: &[Vec<char>]) -> String {
    let mut chars = vec!['\n'];
    for line in map {
        for c in line {
            chars.push(*c);
        }
        chars.push('\n');
    }
    chars.iter().collect()
}

fn part1(
    map: &mut [Vec<char>],
    empty_columns: &HashSet<usize>,
    empty_rows: &HashSet<usize>,
) -> u64 {
    sum_distances(map, empty_columns, empty_rows, &2)
}

fn sum_distances(
    map: &mut [Vec<char>],
    empty_columns: &HashSet<usize>,
    empty_rows: &HashSet<usize>,
    multiplier: &usize,
) -> u64 {
    let mut sum = 0u64;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '#' {
                for dist in distances_from(
                    map,
                    Coords::new(y, x),
                    empty_columns,
                    empty_rows,
                    multiplier,
                ) {
                    sum += dist;
                }
                map[y][x] = '.';
                debug!("sum: {sum}");
                debug!("{}", visualize(&map));
            }
        }
    }
    sum
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Coords {
    y: usize,
    x: usize,
}

impl Coords {
    fn new(y: usize, x: usize) -> Coords {
        Coords { y, x }
    }
}

fn distances_from(
    map: &[Vec<char>],
    root: Coords,
    empty_columns: &HashSet<usize>,
    empty_rows: &HashSet<usize>,
    multiplier: &usize,
) -> Vec<u64> {
    let mut dists = vec![];
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '#' {
                let dist = distance(
                    &root,
                    &Coords::new(y, x),
                    empty_columns,
                    empty_rows,
                    multiplier,
                );
                debug!("{} {} -> {} {}: {}", root.y, root.x, y, x, dist);
                dists.push(dist)
            }
        }
    }
    dists
}

fn distance(
    start: &Coords,
    end: &Coords,
    empty_columns: &HashSet<usize>,
    empty_rows: &HashSet<usize>,
    multiplier: &usize,
) -> u64 {
    let expanded_rows = empty_rows
        .iter()
        .filter(|y| (start.y.min(end.y)..start.y.max(end.y)).contains(y))
        .count();
    let expanded_columns = empty_columns
        .iter()
        .filter(|x| (start.x.min(end.x)..start.x.max(end.x)).contains(x))
        .count();

    debug!("expanded_rows {expanded_rows:?}");
    debug!("expanded_columns {expanded_columns:?}");

    (start.y.abs_diff(end.y)
        + expanded_rows * (multiplier-1)
        + start.x.abs_diff(end.x)
        + expanded_columns * (multiplier-1)) as u64
}

fn part2(
    map: &mut [Vec<char>],
    empty_columns: &HashSet<usize>,
    empty_rows: &HashSet<usize>,
) -> u64 {
    sum_distances(map, empty_columns, empty_rows, &MULTIPLIER)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_with_example() {
        let lines = EXAMPLE.lines().collect::<Vec<_>>();
        assert_eq!(part1(&lines), 0);
    }

    #[test]
    fn part1_with_input() {
        let lines = INPUT.lines().collect::<Vec<_>>();
        assert_eq!(part1(&lines), 0);
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
