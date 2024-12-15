#![allow(clippy::cast_sign_loss)]

use std::collections::HashMap;
use std::collections::HashSet;

type Point = (isize, isize);
type Grid = HashMap<Point, char>;
type Moves = String;

pub fn solve(input: &str) -> usize {
    let (mut grid, mut pos, moves) = parse(input, false);

    for m in moves.chars() {
        if m == '<' {
            pos = move_x(&mut grid, pos, -1);
        } else if m == '>' {
            pos = move_x(&mut grid, pos, 1);
        } else {
            let dir = match m {
                //                '<' => (-1, 0),
                //                '>' => (1, 0),
                '^' => (0, -1),
                'v' => (0, 1),
                _ => panic!("{m:?}"),
            };

            let next = (pos.0 + dir.0, pos.1 + dir.1);
            let tile = grid[&next];

            match tile {
                '#' => (),
                '.' => pos = next,
                'O' => {
                    let mut offset = 2;
                    loop {
                        let target = (pos.0 + offset * dir.0, pos.1 + offset * dir.1);
                        if grid[&target] == '.' {
                            grid.insert(next, '.');
                            grid.insert(target, 'O');
                            pos = next;
                            break;
                        } else if grid[&target] == 'O' {
                            offset += 1;
                        } else {
                            break;
                        }
                    }
                }
                _ => panic!(),
            }
        }
    }

    grid.iter()
        .filter_map(|((x, y), c)| (c == &'O').then_some(x + 100 * y))
        .sum::<isize>() as usize
}

fn move_x(grid: &mut Grid, pos: Point, dir: isize) -> Point {
    if move_x_(grid, pos, dir) {
        (pos.0 + dir, pos.1)
    } else {
        pos
    }
}

fn move_x_(grid: &mut Grid, pos: Point, dir: isize) -> bool {
    let next = (pos.0 + dir, pos.1);
    let tile = grid[&next];

    let ok = match tile {
        '#' => false,
        '.' => true,
        _ => move_x_(grid, next, dir),
    };

    if ok {
        grid.insert(next, grid[&pos]);
        grid.insert(pos, '.');
    }

    ok
}

pub fn solve2(input: &str) -> usize {
    let (mut grid, mut pos, moves) = parse(input, true);

    for m in moves.chars() {
        if m == '<' {
            pos = move_x(&mut grid, pos, -1);
        } else if m == '>' {
            pos = move_x(&mut grid, pos, 1);
        } else {
            let dir = match m {
//                '<' => (-1, 0),
//                '>' => (1, 0),
                '^' => (0, -1),
                'v' => (0, 1),
                _ => panic!("{m:?}"),
            };

            let next = (pos.0 + dir.0, pos.1 + dir.1);
            let set = HashSet::from([pos]);
            if push(&mut grid, set, dir.1) {
                pos = next;
            }
        }
    }

    grid.iter()
        .filter_map(|((x, y), c)| (c == &'[').then_some(x + 100 * y))
        .sum::<isize>() as usize
}

fn push(grid: &mut Grid, set: HashSet<(isize, isize)>, dir_y: isize) -> bool {
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
            _ => panic!(),
        }
    }

    if push(grid, new_set, dir_y) {
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
    assert_eq!(10092, solve(include_str!("example.txt")));
    assert_eq!(1552879, solve(include_str!("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(9021, solve2(include_str!("example.txt")));
    assert_eq!(1561175, solve2(include_str!("input.txt")));
}
