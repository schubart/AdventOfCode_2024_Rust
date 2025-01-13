use std::collections::HashMap;
use std::collections::HashSet;

type Dimension = i32;
type Position = (Dimension, Dimension);
type Grid = HashMap<Position, char>;
type Moves = String;
type Direction = (Dimension, Dimension);

pub fn solve(input: &str, stretch: bool) -> Dimension {
    let (mut grid, (mut x, mut y), moves) = parse(input, stretch);

    for m in moves.chars() {
        let (dx, dy) = match m {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!(),
        };

        let moved = try_move(&mut grid, (x, y), (dx, dy));

        if moved {
            (x, y) = (x + dx, y + dy);
        }
    }

    grid.iter()
        .filter_map(|((x, y), c)| (c == &'O' || c == &'[').then_some(x + 100 * y))
        .sum()
}

fn try_move(grid: &mut Grid, pos: Position, (dx, dy): Direction) -> bool {
    if dy == 0 {
        try_move_x(grid, pos, dx)
    } else {
        try_move_y(grid, HashSet::from([pos]), dy)
    }
}

fn try_move_x(grid: &mut Grid, pos: Position, dx: Dimension) -> bool {
    let next = (pos.0 + dx, pos.1);
    let tile = grid[&next];

    let ok = match tile {
        '#' => false,
        '.' => true,
        _ => try_move_x(grid, next, dx),
    };

    if ok {
        grid.insert(next, grid[&pos]);
        grid.insert(pos, '.');
    }

    ok
}

fn try_move_y(grid: &mut Grid, positions: HashSet<Position>, dy: Dimension) -> bool {
    if positions.is_empty() {
        return true;
    }

    let mut new_set = HashSet::new();
    for pos in &positions {
        let next = (pos.0, pos.1 + dy);
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

    if try_move_y(grid, new_set, dy) {
        for pos in positions {
            let tile = grid[&pos];
            grid.insert((pos.0, pos.1 + dy), tile);
            grid.insert(pos, '.');
        }
        return true;
    }

    false
}

fn parse(input: &str, stretch: bool) -> (Grid, Position, Moves) {
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

    let mut start = None;
    let mut grid = HashMap::new();
    let mut y = 0;
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        for (x, mut c) in line.chars().enumerate() {
            let pos = (x as Dimension, y as Dimension);
            if c == '@' {
                start = Some(pos);
                c = '.';
            }
            grid.insert(pos, c);
        }
        y += 1;
    }

    let moves = lines.collect();

    (grid, start.unwrap(), moves)
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
