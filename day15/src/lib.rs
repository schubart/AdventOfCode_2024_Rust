#![allow(clippy::cast_sign_loss)]

use std::collections::HashMap;
use std::collections::HashSet;

type Point = (isize, isize);
type Grid = HashMap<Point, char>;
type Moves = String;

pub fn solve(input: &str, stretch: bool) -> usize {
    let (mut grid, mut pos, moves) = parse(input, stretch);

    for m in moves.chars() {
        let (dx, dy) = match m {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!(),
        };

        let can_move = if dy == 0 {
            move_x(&mut grid, pos, dx)
        } else {
            move_y(&mut grid, HashSet::from([pos]), dy)
        };

        if can_move {
            pos = (pos.0 + dx, pos.1 + dy);
        }
    }

    grid.iter()
        .filter_map(|((x, y), c)| (c == &'O' || c == &'[').then_some(x + 100 * y))
        .sum::<isize>() as usize
}

fn move_x(grid: &mut Grid, pos: Point, dir: isize) -> bool {
    let next = (pos.0 + dir, pos.1);
    let tile = grid[&next];

    let ok = match tile {
        '#' => false,
        '.' => true,
        _ => move_x(grid, next, dir),
    };

    if ok {
        grid.insert(next, grid[&pos]);
        grid.insert(pos, '.');
    }

    ok
}

fn move_y(grid: &mut Grid, set: HashSet<(isize, isize)>, dir_y: isize) -> bool {
    if set.is_empty() {
        return true;
    }

    let mut new_set = HashSet::new();
    for pos in &set {
        let next = (pos.0, pos.1 + dir_y);
        let tile = grid[&next];

        match tile {
            '#' => return false,
            '.' => (),
            ']' => {
                new_set.insert((next.0 - 1, next.1));
                new_set.insert(next);
            }
            '[' => {
                new_set.insert(next);
                new_set.insert((next.0 + 1, next.1));
            }
            'O' => {
                new_set.insert(next);
            }
            _ => panic!(),
        }
    }

    if move_y(grid, new_set, dir_y) {
        for pos in set {
            let tile = grid[&pos];
            grid.insert((pos.0, pos.1 + dir_y), tile);
            grid.insert(pos, '.');
        }
        return true;
    }

    false
}

fn parse(input: &str, stretch: bool) -> (Grid, Point, Moves) {
    let input = if stretch {
        input
            .replace('#', "##")
            .replace('O', "[]")
            .replace('.', "..")
            .replace('@', "@.")
    } else {
        input.to_string()
    };

    let mut lines = input.lines();

    let mut pos = (0, 0);
    let mut grid = HashMap::new();
    let mut y = 0;
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                pos = (x as isize, y as isize);
                grid.insert((x as isize, y as isize), '.');
            } else {
                grid.insert((x as isize, y as isize), c);
            }
        }
        y += 1;
    }

    let moves = lines.collect();

    (grid, pos, moves)
}

#[test]
fn test_part1() {
    assert_eq!(10092, solve(include_str!("example.txt"), false));
    assert_eq!(1552879, solve(include_str!("input.txt"), false));
}

#[test]
fn test_part2() {
    assert_eq!(9021, solve(include_str!("example.txt"), true));
    assert_eq!(1561175, solve(include_str!("input.txt"), true));
}
