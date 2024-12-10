use std::collections::HashMap;
use std::collections::HashSet;

type Direction = (isize, isize);
type Position = (isize, isize);
type Positions = Vec<Position>;
type Grid = HashMap<Position, u32>;

const DIRECTIONS: [Direction; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn solve1(input: &str) -> usize {
    let grid = parse(input);

    grid.iter()
        .filter_map(|(&pos, &height)| (height == 0).then_some(pos))
        .map(|pos| goals(&grid, pos))
        .map(|positions| HashSet::<_>::from_iter(positions).len())
        .sum()
}

pub fn solve2(input: &str) -> usize {
    let grid = parse(input);

    grid.iter()
        .filter_map(|(&pos, &height)| (height == 0).then_some(pos))
        .map(|pos| goals(&grid, pos).len())
        .sum()
}

fn goals(grid: &Grid, pos: Position) -> Positions {
    let height = grid[&pos];
    if height == 9 {
        Vec::from([pos])
    } else {
        DIRECTIONS
            .iter()
            .map(|dir| (pos.0 + dir.0, pos.1 + dir.1))
            .filter(|next| grid.get(next) == Some(&(height + 1)))
            .flat_map(|next| goals(grid, next))
            .collect()
    }
}

fn parse(input: &str) -> Grid {
    let mut result = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            result.insert((x as isize, y as isize), c.to_digit(10).unwrap());
        }
    }
    result
}

#[test]
fn test_part1() {
    assert_eq!(36, solve1(include_str!("example.txt")));
    assert_eq!(782, solve1(include_str!("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(81, solve2(include_str!("example.txt")));
    assert_eq!(1694, solve2(include_str!("input.txt")));
}
