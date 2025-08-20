use std::cmp::{min, max};

use log::{debug, info};
use itertools::Itertools;
use rayon::iter::ParallelIterator;


const INPUT: &str = include_str!("../input.txt");
const EXAMPLE: &str = include_str!("../example.txt");

fn main() {
    aoc::run_with_bench(INPUT, EXAMPLE, &|aoc| {
        let lines = aoc.read_input_lines();

        (part1(&lines), part2(&lines))
    });
}

fn part1(lines: &[&str]) -> u64 {
    
0
}

fn check_mirror_vertical(pattern: &[Vec<char>], must_include_column: usize) -> Option<usize> {
    let start = max(1, must_include_column/2+must_include_column%2);
    let end = min(pattern[0].len(), (pattern[0].len()+must_include_column)/2);
    for x in start..end {
        if check_mirror_vertical_for_x(pattern, x) {
            debug!("{start}..{end}, must: {must_include_column}, dividor: {x}, pattern len: {}",pattern.len());
            return Some(x);
        }
    }
    None
}

fn check_mirror_vertical_for_x(pattern: &[Vec<char>], dividor:usize) -> bool {
    // Double D
    let dd = dividor +dividor;
    for y in 0..pattern.len() {
        for x in dividor..min(dd, pattern[y].len()) {
            let anti_x = dd - x - 1;
            if pattern[y][x] != pattern[y][anti_x] {return false;}
        }
    }
    true
}

fn check_mirror_horizontal(pattern: &[Vec<char>], must_include_row: usize) -> Option<usize> {
    let start = max(1, must_include_row/2+must_include_row%2);
    let end = min(pattern.len(), (pattern.len()+must_include_row)/2+1);
    for y in start..end {
        if check_mirror_horizontal_for_y(pattern, y) {
            debug!("{start}..{end}, must: {must_include_row}, dividor: {y}, pattern len: {}",pattern.len());
            return Some(y);
        }
    }
    None
}

fn check_mirror_horizontal_for_y(pattern: &[Vec<char>], dividor:usize) -> bool {
    // Double Dividor
    let dd = dividor + dividor;
    for x in 0..pattern[0].len() {
        for y in dividor..min(dd, pattern.len()) {
            let anti_y = dd - y - 1;
            if pattern[y][x] != pattern[anti_y][x] {return false;}
        }
    }
    true
}


fn part2(lines: &[&str]) -> u64 {
    
    let groups = lines.into_iter().group_by(|a| a.is_empty())
    .into_iter()
    .filter(|(empty, _)| !empty)
    .map(|(_, group)| group.collect::<Vec<_>>())
    .collect::<Vec<_>>();

    let mut sum: u64 = 0;
    
    for group in groups {
        for &line in &group {
            debug!("{line}");
        }
        let group = group.into_iter().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        sum += find_smudge(group);
    }
    sum
}

fn find_smudge(mut group: Vec<Vec<char>>) -> u64{
    for y in 0..group.len() {
        for x in 0..group[y].len() {
            let c = group[y][x];
            group[y][x] = if c=='.' {'#'} else {'.'};
            let vertical_value = check_mirror_vertical(&group, x);
            debug!("{y},{x}");
            if let Some(h) = vertical_value {
                for line in &group {
                        debug!("{}",line.iter().collect::<String>());
                    }
                    debug!("{vertical_value:?}");
                    return h as u64;
            }
            let horizontal_value = check_mirror_horizontal(&group, y);
            if let Some(v) = horizontal_value {
                for line in &group {
                    debug!("{}",line.iter().collect::<String>());
                }
                debug!("{:?}", horizontal_value.map(|v|v*100));
                return (v as u64)*100;
            }
            group[y][x] = c;
        }
    }
    0
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
