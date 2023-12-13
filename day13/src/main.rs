use std::cmp::min;

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
        let group2 = group.into_iter().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let vertical_value = check_mirror_vertical(&group2);
        let horizontal_value = check_mirror_horizontal(&group2);
        //debug!("{vertical_value:?}");
        //debug!("{:?}", horizontal_value.map(|v|v*100));
        if let Some(v) = vertical_value {
            sum += v as u64;
        }
        if let Some(h) = horizontal_value {
            sum += (h as u64)*100;
        }
    }
    sum
}

fn check_mirror_vertical(pattern: &[Vec<char>]) -> Option<usize> {
    for x in 1..pattern[0].len() {
        if check_mirror_vertical_for_x(pattern, x) {
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

fn check_mirror_horizontal(pattern: &[Vec<char>]) -> Option<usize> {
    for y in 1..pattern.len() {
        if check_mirror_horizontal_for_y(pattern, y) {
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
        let group2 = group.into_iter().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let vertical_value = check_mirror_vertical(&group2);
        let horizontal_value = check_mirror_horizontal(&group2);
        //debug!("{vertical_value:?}");
        //debug!("{:?}", horizontal_value.map(|v|v*100));
        if let Some(h) = vertical_value {
            sum += h as u64;
        }
        if let Some(v) = horizontal_value {
            sum += (v as u64)*100;
        }
    }
    sum
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
