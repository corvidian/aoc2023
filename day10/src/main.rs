use log::{debug, trace};
use std::char;
use std::collections::HashSet;
use std::fmt::Display;
use std::slice::Iter;
use Direction::*;
use Pipe::*;

const INPUT: &str = include_str!("../input.txt");
const EXAMPLE: &str = include_str!("../example.txt");

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone, Copy)]
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

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone, Copy)]
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
        dirs.contains(direction)
    }

    fn can_squeeze_through(&self, dir: &Direction, quadrant: &Quadrant) -> bool {
        match self {
            Vertical => {
                !(quadrant.is_left() && *dir == Right || quadrant.is_right() && *dir == Left)
            }
            Horizontal => {
                !(quadrant.is_top() && *dir == Down || quadrant.is_bottom() && *dir == Up)
            }
            TopRight => match quadrant {
                Quadrant::TopRight => ![Left, Down].contains(dir),
                Quadrant::TopLeft => *dir != Right,
                Quadrant::BottomRight => *dir != Up,
                Quadrant::BottomLeft => true,
            },
            TopLeft => match quadrant {
                Quadrant::TopRight => *dir != Left,
                Quadrant::TopLeft => ![Right, Down].contains(dir),
                Quadrant::BottomRight => true,
                Quadrant::BottomLeft => ![Up].contains(dir),
            },
            BottomRight => match quadrant {
                Quadrant::TopRight => *dir != Down,
                Quadrant::TopLeft => true,
                Quadrant::BottomRight => ![Left, Up].contains(dir),
                Quadrant::BottomLeft => *dir != Right,
            },
            BottomLeft => match quadrant {
                Quadrant::TopRight => true,
                Quadrant::TopLeft => *dir != Down,
                Quadrant::BottomRight => *dir != Left,
                Quadrant::BottomLeft => ![Right, Up].contains(dir),
            },
            Start => false,
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

impl TryInto<char> for Pipe {
    type Error = char;

    fn try_into(self) -> Result<char, Self::Error> {
        match self {
            Vertical => Ok('│'),
            Horizontal => Ok('─'),
            TopRight => Ok('└'),
            TopLeft => Ok('┘'),
            BottomLeft => Ok('┐'),
            BottomRight => Ok('┌'),
            Start => Ok('S'),
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

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum Quadrant {
    TopRight,
    TopLeft,
    BottomLeft,
    BottomRight,
}

impl Quadrant {
    fn is_top(&self) -> bool {
        match self {
            Quadrant::TopLeft | Quadrant::TopRight => true,
            Quadrant::BottomLeft | Quadrant::BottomRight => false,
        }
    }

    fn is_bottom(&self) -> bool {
        match self {
            Quadrant::TopLeft | Quadrant::TopRight => false,
            Quadrant::BottomLeft | Quadrant::BottomRight => true,
        }
    }

    fn is_left(&self) -> bool {
        match self {
            Quadrant::TopLeft | Quadrant::BottomLeft => true,
            Quadrant::TopRight | Quadrant::BottomRight => false,
        }
    }
    fn is_right(&self) -> bool {
        match self {
            Quadrant::TopLeft | Quadrant::BottomLeft => false,
            Quadrant::TopRight | Quadrant::BottomRight => true,
        }
    }

    fn next(&self, dir: &Direction) -> (Quadrant, bool) {
        match dir {
            Up => match self {
                Quadrant::TopRight => (Quadrant::BottomRight, true),
                Quadrant::TopLeft => (Quadrant::BottomLeft, true),
                Quadrant::BottomLeft => (Quadrant::TopLeft, false),
                Quadrant::BottomRight => (Quadrant::TopRight, false),
            },
            Down => match self {
                Quadrant::TopRight => (Quadrant::BottomRight, false),
                Quadrant::TopLeft => (Quadrant::BottomLeft, false),
                Quadrant::BottomLeft => (Quadrant::TopLeft, true),
                Quadrant::BottomRight => (Quadrant::TopRight, true),
            },
            Left => match self {
                Quadrant::TopRight => (Quadrant::TopLeft, false),
                Quadrant::TopLeft => (Quadrant::TopRight, true),
                Quadrant::BottomLeft => (Quadrant::BottomRight, true),
                Quadrant::BottomRight => (Quadrant::BottomLeft, false),
            },
            Right => match self {
                Quadrant::TopRight => (Quadrant::TopLeft, true),
                Quadrant::TopLeft => (Quadrant::TopRight, false),
                Quadrant::BottomLeft => (Quadrant::BottomRight, false),
                Quadrant::BottomRight => (Quadrant::BottomLeft, true),
            },
        }
    }
}

fn read_line(line: &str) -> Vec<Option<Pipe>> {
    line.chars().map(|c| c.try_into().ok()).collect()
}

fn main() {
    aoc::run_with_bench(INPUT, EXAMPLE, &|aoc| {
        let lines = aoc.input_lines().map(read_line).collect::<Vec<_>>();

        debug!("{}", visualize(&lines, &HashSet::new()));

        let (start_y, start_x, _) = lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, pipe)| (y, x, pipe)))
            .find(|(_, _, pipe)| pipe == &&Some(Start))
            .unwrap();

        debug!("Start point: {start_y}, {start_x}");

        (
            part1(&lines, start_y, start_x),
            part2(&lines, start_y, start_x),
        )
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
        if let Some((y, x)) = try_move(dir, start_y, start_x, map) {
            let steps = count_steps_from(map, y, x, &start_y, &start_x, dir);
            debug!("{dir:?}: {steps}");
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
    trace!("({y} {x}) ({start_y} {start_x}) {from_dir:?}");
    if y == *start_y && x == *start_x {
        return 0;
    }
    let (dir, (new_y, new_x)) = Direction::iterator()
        .filter(|d| **d != from_dir.rev())
        .find_map(|dir| try_move(dir, y, x, map).map(|c| (dir, c)))
        .unwrap();
    count_steps_from(map, new_y, new_x, start_y, start_x, dir) + 1
}

fn part2(input: &[Vec<Option<Pipe>>], start_y: usize, start_x: usize) -> u32 {
    let mut map = vec![vec![None::<Pipe>; input[0].len() + 2]; input.len() + 2];
    map[start_y + 1][start_x + 1] = Some(Start);

    let (mut y, mut x) = (start_y, start_x);
    let mut dir = Left;
    let mut first = true;
    while (y != start_y || x != start_x) || first {
        let (new_dir, (new_y, new_x)) = Direction::iterator()
            .filter(|d| **d != dir.rev())
            .find_map(|dir| try_move(dir, y, x, input).map(|c| (dir, c)))
            .unwrap();
        dir = *new_dir;
        y = new_y;
        x = new_x;
        map[y + 1][x + 1] = input[y][x];
        first = false;
    }

    debug!("{}", visualize(&map, &HashSet::new()));

    let reachable = dfs_iterative(&map, (0, 0));

    debug!("{}", visualize(&map, &reachable));

    let reachable_coords = reachable
        .into_iter()
        .map(|(y, x, _)| (y, x))
        .collect::<HashSet<_>>();

    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x].is_none() && !reachable_coords.contains(&(y, x)) {
                sum += 1;
            }
        }
    }
    sum
}

fn dfs_iterative(
    map: &[Vec<Option<Pipe>>],
    v: (usize, usize),
) -> HashSet<(usize, usize, Quadrant)> {
    let mut s = vec![];
    let mut discovered = HashSet::new();

    s.push((v.0, v.1, Quadrant::TopLeft));
    while let Some((y, x, q)) = s.pop() {
        if !discovered.contains(&(y, x, q)) {
            discovered.insert((y, x, q));
            trace!("Found {y} {x} {q:?}");
            trace!("{}", visualize(map, &discovered));
            for w in squeezable_edges(map, y, x, &q) {
                s.push(w);
            }
        }
    }
    discovered
}

fn visualize(map: &[Vec<Option<Pipe>>], reached: &HashSet<(usize, usize, Quadrant)>) -> String {
    let mut visual = vec![vec!['.'; map[0].len()]; map.len()];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            visual[y][x] = map[y][x].map(|p| p.try_into().unwrap()).unwrap_or('.')
        }
    }
    for (y, x, _) in reached {
        visual[*y][*x] = 'O';
    }

    let mut chars = vec!['\n'];
    for line in &visual {
        for c in line {
            chars.push(*c);
        }
        chars.push('\n');
    }
    chars.iter().collect()
}

fn try_squeeze(
    dir: &Direction,
    y: usize,
    x: usize,
    quadrant: &Quadrant,
    map: &[Vec<Option<Pipe>>],
) -> Option<(usize, usize, Quadrant)> {
    if let Some(pipe) = map[y][x] {
        if !pipe.can_squeeze_through(dir, quadrant) {
            return None;
        }
    }

    let (next_q, change_coords) = quadrant.next(dir);

    let (new_y, new_x) = if change_coords {
        match dir {
            Up => y.checked_sub(1).map(|y| (y, x)),
            Down => {
                if y < map.len() - 1 {
                    Some((y + 1, x))
                } else {
                    None
                }
            }
            Left => x.checked_sub(1).map(|x| (y, x)),
            Right => {
                if x < map[0].len() - 1 {
                    Some((y, x + 1))
                } else {
                    None
                }
            }
        }?
    } else {
        (y, x)
    };

    Some((new_y, new_x, next_q))
}

fn squeezable_edges(
    map: &[Vec<Option<Pipe>>],
    y: usize,
    x: usize,
    quadrant: &Quadrant,
) -> Vec<(usize, usize, Quadrant)> {
    let reachable = Direction::iterator()
        .filter_map(|dir| try_squeeze(dir, y, x, quadrant, map))
        .collect();
    trace!("From map coords ({y},{x},{quadrant:?}) we can get to: {reachable:?}");
    reachable
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_squeeze_through_horizontal() {
        let pipe = Pipe::Horizontal;
        let cases = [
            (Up, Quadrant::TopRight, true),
            (Up, Quadrant::TopLeft, true),
            (Up, Quadrant::BottomLeft, false),
            (Up, Quadrant::BottomRight, false),
            (Down, Quadrant::TopRight, false),
            (Down, Quadrant::TopLeft, false),
            (Down, Quadrant::BottomLeft, true),
            (Down, Quadrant::BottomRight, true),
            (Left, Quadrant::TopRight, true),
            (Left, Quadrant::TopLeft, true),
            (Left, Quadrant::BottomLeft, true),
            (Left, Quadrant::BottomRight, true),
            (Right, Quadrant::TopRight, true),
            (Right, Quadrant::TopLeft, true),
            (Right, Quadrant::BottomLeft, true),
            (Right, Quadrant::BottomRight, true),
        ];
        for case in cases {
            assert_eq!(case.2, pipe.can_squeeze_through(&case.0, &case.1));
        }
    }

    #[test]
    fn can_squeeze_through_vertical() {
        let pipe = Pipe::Vertical;
        let cases = [
            (Up, Quadrant::TopRight, true),
            (Up, Quadrant::TopLeft, true),
            (Up, Quadrant::BottomLeft, true),
            (Up, Quadrant::BottomRight, true),
            (Down, Quadrant::TopRight, true),
            (Down, Quadrant::TopLeft, true),
            (Down, Quadrant::BottomLeft, true),
            (Down, Quadrant::BottomRight, true),
            (Left, Quadrant::TopRight, false),
            (Left, Quadrant::TopLeft, true),
            (Left, Quadrant::BottomLeft, true),
            (Left, Quadrant::BottomRight, false),
            (Right, Quadrant::TopRight, true),
            (Right, Quadrant::TopLeft, false),
            (Right, Quadrant::BottomLeft, false),
            (Right, Quadrant::BottomRight, true),
        ];
        for case in cases {
            assert_eq!(case.2, pipe.can_squeeze_through(&case.0, &case.1));
        }
    }

    #[test]
    fn can_squeeze_through_topleft() {
        let pipe = Pipe::TopLeft;
        let cases = [
            (Up, Quadrant::TopRight, true),
            (Up, Quadrant::TopLeft, true),
            (Up, Quadrant::BottomLeft, false),
            (Up, Quadrant::BottomRight, true),
            (Down, Quadrant::TopRight, true),
            (Down, Quadrant::TopLeft, false),
            (Down, Quadrant::BottomLeft, true),
            (Down, Quadrant::BottomRight, true),
            (Left, Quadrant::TopRight, false),
            (Left, Quadrant::TopLeft, true),
            (Left, Quadrant::BottomLeft, true),
            (Left, Quadrant::BottomRight, true),
            (Right, Quadrant::TopRight, true),
            (Right, Quadrant::TopLeft, false),
            (Right, Quadrant::BottomLeft, true),
            (Right, Quadrant::BottomRight, true),
        ];
        for case in cases {
            println!("{case:?}");
            assert_eq!(case.2, pipe.can_squeeze_through(&case.0, &case.1));
        }
    }

    #[test]
    fn can_squeeze_through_topright() {
        let pipe = Pipe::TopRight;
        let cases = [
            (Up, Quadrant::TopRight, true),
            (Up, Quadrant::TopLeft, true),
            (Up, Quadrant::BottomLeft, true),
            (Up, Quadrant::BottomRight, false),
            (Down, Quadrant::TopRight, false),
            (Down, Quadrant::TopLeft, true),
            (Down, Quadrant::BottomLeft, true),
            (Down, Quadrant::BottomRight, true),
            (Left, Quadrant::TopRight, false),
            (Left, Quadrant::TopLeft, true),
            (Left, Quadrant::BottomLeft, true),
            (Left, Quadrant::BottomRight, true),
            (Right, Quadrant::TopRight, true),
            (Right, Quadrant::TopLeft, false),
            (Right, Quadrant::BottomLeft, true),
            (Right, Quadrant::BottomRight, true),
        ];
        for case in cases {
            println!("{case:?}");
            assert_eq!(case.2, pipe.can_squeeze_through(&case.0, &case.1));
        }
    }

    #[test]
    fn can_squeeze_through_bottom_left() {
        let pipe = Pipe::BottomLeft;
        let cases = [
            (Up, Quadrant::TopRight, true),
            (Up, Quadrant::TopLeft, true),
            (Up, Quadrant::BottomLeft, false),
            (Up, Quadrant::BottomRight, true),
            (Down, Quadrant::TopRight, true),
            (Down, Quadrant::TopLeft, false),
            (Down, Quadrant::BottomLeft, true),
            (Down, Quadrant::BottomRight, true),
            (Left, Quadrant::TopRight, true),
            (Left, Quadrant::TopLeft, true),
            (Left, Quadrant::BottomLeft, true),
            (Left, Quadrant::BottomRight, false),
            (Right, Quadrant::TopRight, true),
            (Right, Quadrant::TopLeft, true),
            (Right, Quadrant::BottomLeft, false),
            (Right, Quadrant::BottomRight, true),
        ];
        for case in cases {
            println!("{case:?}");
            assert_eq!(case.2, pipe.can_squeeze_through(&case.0, &case.1));
        }
    }

    #[test]
    fn can_squeeze_through_bottom_right() {
        let pipe = Pipe::BottomRight;
        let cases = [
            (Up, Quadrant::TopRight, true),
            (Up, Quadrant::TopLeft, true),
            (Up, Quadrant::BottomLeft, true),
            (Up, Quadrant::BottomRight, false),
            (Down, Quadrant::TopRight, false),
            (Down, Quadrant::TopLeft, true),
            (Down, Quadrant::BottomLeft, true),
            (Down, Quadrant::BottomRight, true),
            (Left, Quadrant::TopRight, true),
            (Left, Quadrant::TopLeft, true),
            (Left, Quadrant::BottomLeft, true),
            (Left, Quadrant::BottomRight, false),
            (Right, Quadrant::TopRight, true),
            (Right, Quadrant::TopLeft, true),
            (Right, Quadrant::BottomLeft, false),
            (Right, Quadrant::BottomRight, true),
        ];
        for case in cases {
            println!("{case:?}");
            assert_eq!(case.2, pipe.can_squeeze_through(&case.0, &case.1));
        }
    }
}
