const INPUT: &str = include_str!("../input.txt");
const EXAMPLE: &str = include_str!("../example.txt");

fn main() {
    aoc::run_with_bench(INPUT, EXAMPLE, &|aoc| {
        let lines = aoc.read_input_lines();

        (part1(&lines), part2(&lines))
    });
}

fn part1(lines: &[&str]) -> u32 {
    let times = parse_numbers(lines[0].split_once(':').unwrap().1);
    let distances = parse_numbers(lines[1].split_once(':').unwrap().1);
    times
        .iter()
        .zip(distances)
        .map(|(time, distance)| count_win_strategies(*time, distance))
        .product()
}

fn count_win_strategies(time: u64, record_distance: u64) -> u32 {
    (1..time)
        .filter(|button_time| (button_time * (time - button_time)) > record_distance)
        .count() as u32
}

fn parse_numbers(list: &str) -> Vec<u64> {
    list.split_whitespace()
        .map(|n| n.parse::<u64>().expect("Not a number!"))
        .collect::<Vec<_>>()
}

fn part2(lines: &[&str]) -> u32 {
    let time = concat_numbers(lines[0].split_once(':').unwrap().1);
    let distance = concat_numbers(lines[1].split_once(':').unwrap().1);
    count_win_strategies(time, distance)
}

fn concat_numbers(list: &str) -> u64 {
    list.replace(' ', "").parse::<u64>().expect("Not a number!")
}