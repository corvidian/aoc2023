use std::cmp::min;

use log::{debug, info};

fn main() {
    aoc::init_logging();

    let lines = aoc::read_input_lines();
    let map: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let height = lines.len();
    let width = map[0].len();

    part1(&map, height, width);

    part2(&map, height, width);
}

fn part1(map: &[Vec<char>], height: usize, width: usize) {
    let mut sum: i32 = 0;
    for y in 0..height {
        let mut x = 0;
        while x < width {
            if map[y][x].is_ascii_digit() {
                let (number, x2) = read_number(map, width, y, x);
                let added = check_for_symbols(map, height, width, y, x, x2);
                debug!("y: {y}, x:{x}, {x2}: {number} {added}");
                if added {
                    sum += number
                }
                x = x2;
            }
            x += 1;
        }
    }

    info!("Part 1: {}", sum);
}

fn check_for_symbols(
    map: &[Vec<char>],
    height: usize,
    width: usize,
    y: usize,
    x: usize,
    x2: usize,
) -> bool {
    let start_y = y.saturating_sub(1);
    let end_y = min(y + 1, height - 1);
    let start_x = x.saturating_sub(1);
    let end_x = min(x2 + 1, width - 1);

    map[start_y..end_y + 1].iter().any(|row| {
        row[start_x..end_x + 1]
            .iter()
            .any(|c| !c.is_ascii_digit() && *c != '.')
    })
}

fn read_number(map: &[Vec<char>], width: usize, y: usize, x: usize) -> (i32, usize) {
    let mut x = x;
    let mut chars: Vec<char> = vec![];
    while x < width && map[y][x].is_ascii_digit() {
        chars.push(map[y][x]);
        x += 1;
    }
    (
        chars.iter().collect::<String>().parse::<i32>().unwrap(),
        x - 1,
    )
}

fn part2(map: &[Vec<char>], height: usize, width: usize) {
    let mut sum: i32 = 0;
    for y in 0..height {
        let mut x = 0;
        while x < width {
            if map[y][x] == '*' {
                debug!("Potential cog at {y}, {x}");
                let gear_ratio = gear_ratio(map, height, width, y, x);
                debug!("gear_ratio {gear_ratio:?}");
                sum += gear_ratio;
            }
            x += 1;
        }
    }
    info!("Part 2: {}", sum);
}

fn gear_ratio(map: &[Vec<char>], height: usize, width: usize, y: usize, x: usize) -> i32 {
    let start_y = y.saturating_sub(1);
    let end_y = min(y + 1, height - 1);
    let start_x = x.saturating_sub(1);
    let end_x = min(x + 1, width - 1);

    let mut numbers: Vec<i32> = vec![];

    for y in start_y..=end_y {
        let mut x: usize = start_x;
        while x <= end_x {
            if map[y][x].is_ascii_digit() {
                let mut number_start = x as i32;
                while number_start > 0 && map[y][(number_start - 1) as usize].is_ascii_digit() {
                    number_start -= 1;
                }
                let (number, x2) = read_number(map, width, y, number_start as usize);
                debug!("number: {number}, x2:{x2}");
                numbers.push(number);
                x = x2;
            }
            x += 1;
        }
    }

    if numbers.len() == 2 {
        numbers[0] * numbers[1]
    } else {
        0
    }
}
