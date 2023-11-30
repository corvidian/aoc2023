use log::{debug, info};

fn main() {
    aoc::init_logging();

    let lines = aoc::read_input_lines();

    part1(&lines);

    part2(&lines);
}

fn part1(lines: &[String]) {
    info!("Lines in part 1: {}", lines.len());
}

fn part2(lines: &[String]) {
    debug!("Lines in part 2: {}", lines.len());
}
