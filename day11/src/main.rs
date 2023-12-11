use std::collections::{HashSet, VecDeque};

use log::{debug, info};

const INPUT: &str = include_str!("../input.txt");
const EXAMPLE: &str = include_str!("../example.txt");

fn main() {
    aoc::run_with_bench(INPUT, EXAMPLE, &|aoc| {
        let lines = aoc
            .input_lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let columns = (0..lines[0].len()).collect::<HashSet<usize>>();
        let positions = lines
            .iter()
            .flat_map(|line| {
                line.iter()
                    .enumerate()
                    .filter(|(_, c)| **c == '#')
                    .map(|(i, _)| i)
            })
            .collect::<HashSet<_>>();
        let empty_columns = columns.difference(&positions).collect::<HashSet<_>>();
        debug!("empty_columns: {empty_columns:?}");

        let rows = (0..lines[0].len()).collect::<HashSet<usize>>();
        let positions = lines
            .iter()
            .enumerate()
            .filter(|(_, line)| line.contains(&'#'))
            .map(|(i, _)| i)
            .collect::<HashSet<_>>();
        let empty_rows = rows.difference(&positions).collect::<HashSet<_>>();
        debug!("empty_rows: {empty_rows:?}");

        let mut map =
            vec![vec!['.'; lines[0].len() + empty_columns.len()]; lines.len() + empty_rows.len()];

        let mut map_y = 0;
        for orig_y in 0..lines.len() {
            if empty_rows.contains(&orig_y) {
                map_y += 1
            };
            let mut map_x = 0;
            for orig_x in 0..lines[orig_y].len() {
                if empty_columns.contains(&orig_x) {
                    map_x += 1
                };
                map[map_y][map_x] = lines[orig_y][orig_x];
                map_x += 1;
            }
            map_y += 1;
        }
        debug!("{}", visualize(&map));

        (part1(&mut map), part2(&map))
    });
}

fn visualize(map: &[Vec<char>]) -> String {
    let mut chars = vec!['\n'];
    for line in map {
        for c in line {
            chars.push(*c);
        }
        chars.push('\n');
    }
    chars.iter().collect()
}

fn part1(map: &mut [Vec<char>]) -> u32 {
    let mut sum = 0u32;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '#' {
                for dist in bfs(map, Coords::new(y, x)) {
                    sum += dist;
                }
                map[y][x] = '.';
            }
        }
    }
    sum
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Coords {
    y: usize,
    x: usize,
}

impl Coords {
    fn new(y: usize, x: usize) -> Coords {
        Coords { y, x }
    }

    fn with_y(&self, new_y: usize) -> Coords {
        Coords {
            y: new_y,
            x: self.x,
        }
    }

    fn with_x(&self, new_x: usize) -> Coords {
        Coords {
            y: self.y,
            x: new_x,
        }
    }
}

fn bfs(map: &[Vec<char>], root: Coords) -> Vec<u32> {
    let mut dists = vec![];
    let mut Q = VecDeque::new();
    let mut explored = HashSet::new();
    Q.push_back((root, 0));
    explored.insert(root);
    while let Some(v) = Q.pop_front() {
        if map[v.0.y][v.0.x] == '#' {
            debug!("{} {} -> {} {}: {}", root.y, root.x, v.0.y, v.0.x, v.1);
            dists.push(v.1)
        }
        for w in adjacent_edges(map, &v.0) {
            if !explored.contains(&w) {
                explored.insert(w);
                Q.push_back((w, v.1 + 1));
            }
        }
    }
    debug!("dists count: {}", dists.len());
    dists
}

fn adjacent_edges(map: &[Vec<char>], v: &Coords) -> Vec<Coords> {
    let mut coords = vec![];
    if v.y != 0 {
        coords.push(v.with_y(v.y - 1));
    }
    if v.y < map.len() - 1 {
        coords.push(v.with_y(v.y + 1));
    }
    if v.x != 0 {
        coords.push(v.with_x(v.x - 1));
    }
    if v.x < map[v.y].len() - 1 {
        coords.push(v.with_x(v.x + 1));
    }
    coords
}

fn part2(_lines: &[Vec<char>]) -> u32 {
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
