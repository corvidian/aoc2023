use aoc::parse_numbers;
use log::{debug, info};

const INPUT: &str = include_str!("../input.txt");
const EXAMPLE: &str = include_str!("../example.txt");

fn main() {
    aoc::run_with_bench(INPUT, EXAMPLE, &|aoc| {
        let lines: Vec<Vec<i64>> = aoc
            .input_lines()
            .map(|line| parse_numbers::<i64, Vec<_>>(line))
            .collect();

        debug!("{lines:?}");

        (part1(&lines), part2(&lines))
    });
}

fn part1(lines: &[Vec<i64>]) -> i64 {
     lines.iter().map(|line| history(line)).map(predict).sum()
}

fn predict(history: Vec<Vec<i64>>) -> i64 {
    let mut y = history.len()-1;
    let mut prediction = 0i64;
    
    while y > 0 {
        prediction = history[y-1].last().unwrap() + prediction;
        //debug!("prediction: {prediction}");
        y-=1;
    }
    prediction
}

fn history(line: &[i64]) -> Vec<Vec<i64>> {
    let mut history = vec![line.to_vec()];
    let mut i: usize = 0;
    debug!("{line:?}");
    while !history[i].iter().all(|a| *a == 0) {
        let next = next_line(&history[i]);
        debug!("{next:?}");
        history.push(next);
        i = i + 1;
    }
    history
}

fn next_line(line: &[i64]) -> Vec<i64> {
    line.windows(2).map(|a| a[1] - a[0]).collect()
}

fn part2(lines: &[Vec<i64>]) -> i64 {
    lines.iter().map(|line| history(line)).map(extrapolate_history).sum()
}

fn extrapolate_history(history: Vec<Vec<i64>>) -> i64 {
    let mut y = history.len()-1;
    let mut prediction = 0i64;
    
    while y > 0 {
        let head = history[y-1].first().unwrap();
        debug!("{head} - {prediction}  = {}", head-prediction );
        prediction = head - prediction ;
        debug!("prediction: {prediction}");
        y-=1;
    }
    prediction
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
        assert_eq!(part1(&lines), 0);
    }

    #[test]
    fn part2_with_example() {
        let lines: Vec<Vec<i64>> = EXAMPLE
            .lines()
            .map(|line| parse_numbers::<i64, Vec<_>>(line))
            .collect();
        assert_eq!(part2(&lines), 0);
    }

    #[test]
    fn part2_with_input() {
        let lines: Vec<Vec<i64>> = INPUT
            .lines()
            .map(|line| parse_numbers::<i64, Vec<_>>(line))
            .collect();
        assert_eq!(part2(&lines), 0);
    }
}
