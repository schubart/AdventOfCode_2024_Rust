use std::collections::HashMap;
use std::collections::HashSet;

type Scalar = i8;
type Position = (Scalar, Scalar);
type Grid = HashMap<Position, char>;
type Antennas = HashMap<char, Vec<Position>>;

pub fn solve(input: &str) -> usize {
    let (grid, antennas) = parse(input);

    let mut antinodes = HashSet::new();
    for (antenna, positions) in antennas {
        for (x1, y1) in &positions {
            for (x2, y2) in &positions {
                let (dx, dy) = (x1 - x2, y1 - y2);
                let antinode = (x1 + dx, y1 + dy);
                if let Some(&c) = grid.get(&antinode) {
                    if c != antenna {
                        antinodes.insert(antinode);
                    }
                }
            }
        }
    }

    antinodes.len()
}

pub fn solve2(input: &str) -> usize {
    let (grid, antennas) = parse(input);

    let mut antinodes = HashSet::new();
    for positions in antennas.values() {
        for (x1, y1) in positions {
            for (x2, y2) in positions {
                if (x1, y1) != (x2, y2) {
                    let (dx, dy) = (x1 - x2, y1 - y2);
                    antinodes.extend(
                        (0..)
                            .map(|count| (x1 + dx * count, y1 + dy * count))
                            .take_while(|antinode| grid.contains_key(antinode)),
                    );
                }
            }
        }
    }

    antinodes.len()
}

fn parse(input: &str) -> (Grid, Antennas) {
    let mut grid = Grid::new();
    let mut antennas = Antennas::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = (x as Scalar, y as Scalar);
            if c != '.' {
                antennas.entry(c).or_default().push(pos);
            }
            grid.insert(pos, c);
        }
    }

    (grid, antennas)
}

#[test]
fn test_part1() {
    assert_eq!(14, solve(include_str!("example.txt")));
    assert_eq!(256, solve(include_str!("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(34, solve2(include_str!("example.txt")));
    assert_eq!(1005, solve2(include_str!("input.txt")));
}
