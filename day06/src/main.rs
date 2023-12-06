use aoc::parse_numbers;

const INPUT: &str = include_str!("../input.txt");
const EXAMPLE: &str = include_str!("../example.txt");

fn main() {
    aoc::run_with_bench(INPUT, EXAMPLE, &|aoc| {
        let lines = aoc.read_input_lines();

        (part1(&lines), part2(&lines))
    });
}

fn part1(lines: &[&str]) -> u32 {
    let times: Vec<_> = parse_numbers(lines[0].split_once(':').unwrap().1);
    let distances: Vec<_> = parse_numbers(lines[1].split_once(':').unwrap().1);
    times
        .iter()
        .zip(distances)
        .map(|(time, distance)| count_win_strategies(*time, distance))
        .product()
}

fn count_win_strategies(time: u64, record_distance: u64) -> u32 {
    // The equation for checking if a particular time of button-pressing leads to a new record is:
    // button_time * (time - button_time) > record_distance
    // Solving for button_time gives us:
    // time/2 - sqrt((time/2)^2 - record_distance) < button_time < time/2 + sqrt((time/2)^2 - record_distance)

    // Solve the roots of the paraboloid:
    let b = time as f64;
    let c = record_distance as f64;
    let half_b = b / 2f64;
    let root = (half_b * half_b - c).sqrt();
    let x1 = half_b - root;
    let x2 = half_b + root;

    // Count only inside the roots, excluding the roots if they are integer:
    let start = x1.floor() as u64 + 1;
    let end = x2.ceil() as u64 - 1;

    // Include both start and end, so + 1
    (end - start + 1) as u32
}

fn part2(lines: &[&str]) -> u32 {
    let time = concat_numbers(lines[0].split_once(':').unwrap().1);
    let distance = concat_numbers(lines[1].split_once(':').unwrap().1);
    count_win_strategies(time, distance)
}

fn concat_numbers(list: &str) -> u64 {
    list.replace(' ', "").parse::<u64>().expect("Not a number!")
}
