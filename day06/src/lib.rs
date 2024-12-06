use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

type Scalar = i16;
type Direction = (Scalar, Scalar);
type Position = (Scalar, Scalar);
type Positions = HashSet<Position>;
type Grid = HashMap<Position, char>;

pub fn part1(input: &str) -> usize {
    let (grid, start) = parse(input);

    visit(&grid, start).len()
}

pub fn part2(input: &str) -> usize {
    let (grid, start) = parse(input);

    visit(&grid, start)
        .par_iter()
        .filter(|&pos| grid[pos] == '.')
        .filter(|&pos| is_loop(&grid, start, *pos))
        .count()
}

fn visit(grid: &Grid, start: Position) -> Positions {
    let mut pos = start;
    let mut dir = (0, -1);
    let mut visited = HashSet::from([pos]);

    loop {
        let next_pos = (pos.0 + dir.0, pos.1 + dir.1);

        match grid.get(&next_pos) {
            Some('#') => {
                dir = turn_right(dir);
            }
            Some(_) => {
                pos = next_pos;
                visited.insert(pos);
            }
            _ => {
                return visited;
            }
        };
    }
}

fn is_loop(grid: &Grid, start: Position, obstacle: Position) -> bool {
    let mut pos = start;
    let mut dir = (0, -1);
    let mut seen = HashSet::from([(pos, dir)]);

    loop {
        let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
        let next_char = if next_pos == obstacle {
            Some(&'#')
        } else {
            grid.get(&next_pos)
        };

        match next_char {
            Some('#') => {
                dir = turn_right(dir);
                if !seen.insert((pos, dir)) {
                    return true;
                }
            }
            Some(_) => {
                pos = next_pos;
            }
            _ => {
                return false;
            }
        };
    }
}

fn turn_right(direction: Direction) -> Direction {
    match direction {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => unreachable!(),
    }
}

fn parse(input: &str) -> (Grid, Position) {
    let mut grid = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert((x as Scalar, y as Scalar), c);
        }
    }

    let start = grid
        .iter()
        .find_map(|(&pos, &c)| (c == '^').then_some(pos))
        .unwrap();

    (grid, start)
}

#[test]
fn test_part1() {
    assert_eq!(41, part1(include_str!("example.txt")));
    assert_eq!(5030, part1(include_str!("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(6, part2(include_str!("example.txt")));
    assert_eq!(1928, part2(include_str!("input.txt")));
}
