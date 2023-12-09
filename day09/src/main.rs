use aoc::parse_numbers;
use log::debug;

const INPUT: &str = include_str!("../input.txt");
const EXAMPLE: &str = include_str!("../example.txt");

fn main() {
    aoc::run_with_bench(INPUT, EXAMPLE, &|aoc| {
        let lines: Vec<Vec<i64>> = aoc
            .input_lines()
            .map(parse_numbers::<i64, Vec<_>>)
            .collect();

        debug!("{lines:?}");

        (part1(&lines), part2(&lines))
    });
}

fn part1(lines: &[Vec<i64>]) -> i64 {
    lines.iter().map(|line| history(line)).map(predict).sum()
}

fn predict(history: Vec<Vec<i64>>) -> i64 {
    history
        .iter()
        .rev()
        .fold(0, |prediction, line| line.last().unwrap() + prediction)
}

fn history(line: &[i64]) -> Vec<Vec<i64>> {
    let mut history = vec![line.to_vec()];
    let mut i: usize = 0;
    debug!("{line:?}");
    while !history[i].iter().all(|a| *a == 0) {
        let next = next_line(&history[i]);
        debug!("{next:?}");
        history.push(next);
        i += 1;
    }
    history
}

fn next_line(line: &[i64]) -> Vec<i64> {
    line.windows(2).map(|a| a[1] - a[0]).collect()
}

fn part2(lines: &[Vec<i64>]) -> i64 {
    lines.iter().map(|line| history(line)).map(go_back).sum()
}

fn go_back(history: Vec<Vec<i64>>) -> i64 {
    history
        .iter()
        .rev()
        .fold(0, |prediction, line| line.first().unwrap() - prediction)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_with_example() {
        let lines: Vec<Vec<i64>> = EXAMPLE
            .lines()
            .map(|line| parse_numbers::<i64, Vec<_>>(line))
            .collect();
        assert_eq!(part1(&lines), 114);
    }

    #[test]
    fn part1_with_input() {
        let lines: Vec<Vec<i64>> = INPUT
            .lines()
            .map(|line| parse_numbers::<i64, Vec<_>>(line))
            .collect();
        assert_eq!(part1(&lines), 1877825184);
    }

    #[test]
    fn part2_with_example() {
        let lines: Vec<Vec<i64>> = EXAMPLE
            .lines()
            .map(|line| parse_numbers::<i64, Vec<_>>(line))
            .collect();
        assert_eq!(part2(&lines), 2);
    }

    #[test]
    fn part2_with_input() {
        let lines: Vec<Vec<i64>> = INPUT
            .lines()
            .map(|line| parse_numbers::<i64, Vec<_>>(line))
            .collect();
        assert_eq!(part2(&lines), 1108);
    }
}
