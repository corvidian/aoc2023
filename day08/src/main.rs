use log::{debug, info};

const INPUT: &str = include_str!("../input.txt");
const EXAMPLE: &str = include_str!("../example.txt");

fn main() {
    aoc::run_with_bench(INPUT, EXAMPLE, &|aoc| {
        let lines = aoc.read_input_lines();

        (part1(&lines), part2(&lines))
    });
}

fn part1(lines: &[&str]) -> u32 {
    0
}

fn part2(_lines: &[&str]) -> u32 {
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
