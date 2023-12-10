use log::{debug, info};
use std::fmt::Display;
use std::slice::Iter;
use Direction::*;
use Pipe::*;

const INPUT: &str = include_str!("../input.txt");
const EXAMPLE: &str = include_str!("../example.txt");

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [Up, Down, Left, Right];
        DIRECTIONS.iter()
    }

    fn rev(&self) -> Direction {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
enum Pipe {
    Vertical,
    Horizontal,
    TopRight,
    TopLeft,
    BottomLeft,
    BottomRight,
    Start,
}

impl Pipe {
    fn is_connected(&self, direction: &Direction) -> bool {
        let dirs = match self {
            Vertical => [Up, Down],
            Horizontal => [Left, Right],
            TopRight => [Up, Right],
            TopLeft => [Up, Left],
            BottomLeft => [Down, Left],
            BottomRight => [Down, Right],
            Start => return true,
        };
        if dirs.contains(&direction) {
            true
        } else {
            false
        }
    }
}

impl TryFrom<char> for Pipe {
    type Error = char;

    fn try_from(chr: char) -> Result<Self, Self::Error> {
        match chr {
            '|' => Ok(Vertical),
            '-' => Ok(Horizontal),
            'L' => Ok(TopRight),
            'J' => Ok(TopLeft),
            '7' => Ok(BottomLeft),
            'F' => Ok(BottomRight),
            'S' => Ok(Start),
            _ => Err(chr),
        }
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Vertical => '│',
            Horizontal => '─',
            TopRight => '└',
            TopLeft => '┘',
            BottomLeft => '┐',
            BottomRight => '┌',
            Start => 'S',
        };

        write!(f, "{s}")
    }
}

fn read_line(line: &str) -> Vec<Option<Pipe>> {
    line.chars().map(|c| c.try_into().ok()).collect()
}

fn show_line(line: &[Option<Pipe>]) -> String {
    line.iter()
        .map(|o| match o {
            Some(pipe) => format!("{pipe}"),
            None => ".".to_string(),
        })
        .collect()
}

fn main() {
    aoc::run_with_bench(INPUT, EXAMPLE, &|aoc| {
        let lines = aoc.input_lines().map(read_line).collect::<Vec<_>>();

        for line in &lines {
            debug!("{}", show_line(&line));
        }
        let (start_y, start_x, _) = lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, pipe)| (y, x, pipe)))
            .find(|(_, _, pipe)| pipe == &&Some(Start))
            .unwrap();

        debug!("{start_y}, {start_x}");

        (part1(&lines, start_y, start_x), part2(&lines))
    });
}

fn try_move(
    dir: &Direction,
    y: usize,
    x: usize,
    map: &[Vec<Option<Pipe>>],
) -> Option<(usize, usize)> {
    if let Some(pipe) = &map[y][x] {
        if !pipe.is_connected(dir) {
            return None;
        }
        let (new_y, new_x) = match dir {
            Up => {
                if y == 0 {
                    return None;
                } else {
                    (y - 1, x)
                }
            }
            Down => {
                if y >= map.len() - 1 {
                    return None;
                } else {
                    (y + 1, x)
                }
            }
            Left => {
                if x == 0 {
                    return None;
                } else {
                    (y, x - 1)
                }
            }
            Right => {
                if x >= map[0].len() - 1 {
                    return None;
                } else {
                    (y, x + 1)
                }
            }
        };

        if let Some(pipe) = &map[new_y][new_x] {
            if pipe.is_connected(&dir.rev()) {
                return Some((new_y, new_x));
            }
        }
        None
    } else {
        None
    }
}

fn part1(map: &[Vec<Option<Pipe>>], start_y: usize, start_x: usize) -> u32 {
    for dir in Direction::iterator() {
        if let Some((y,x)) = try_move(dir, start_y, start_x, map) {
            let steps = count_steps_from(map, y, x, &start_y, &start_x, dir);
            debug!("{dir:?}: {steps}" );
            return steps / 2 + steps % 2;
        }
    }

    0
}

fn count_steps_from(
    map: &[Vec<Option<Pipe>>],
    y: usize,
    x: usize,
    start_y: &usize,
    start_x: &usize,
    from_dir: &Direction,
) -> u32 {
    //debug!("({y} {x}) ({start_y} {start_x}) {from_dir:?}");
    if y == *start_y && x == *start_x {
        return 0;
    }
    let (dir, (new_y, new_x)) = Direction::iterator()
        .filter(|d| **d != from_dir.rev())
        .find_map(|dir| try_move(dir, y, x, map).map(|c| (dir, c)))
        .unwrap();
    count_steps_from(map, new_y, new_x, start_y, start_x, dir) + 1
}

fn part2(_lines: &[Vec<Option<Pipe>>]) -> u32 {
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
