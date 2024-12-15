use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    let (mut grid, mut pos, moves) = parse(input);

    for m in moves.chars() {
        let dir = match m {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!("{m:?}"),
        };

        let next = (pos.0 + dir.0, pos.1 + dir.1);
        let tile = grid.get(&next);

        match tile {
            None | Some(&'#') => (),
            Some(&'.') => pos = next,
            Some(&'O') => {
                let mut offset = 2;
                loop {
                    let target = (pos.0 + offset * dir.0, pos.1 + offset * dir.1);
                    if grid.get(&target) == Some(&'.') {
                        grid.insert(next, '.');
                        grid.insert(target, 'O');
                        pos = next;
                        break;
                    } else if grid.get(&target) == Some(&'O') {
                        offset += 1;
                    } else {
                        break;
                    }
                }
            }
            _ => panic!(),
        }
    }

    grid.iter()
        .filter_map(|((x, y), c)| (c == &'O').then_some(x + 100 * y))
        .sum::<isize>() as usize
}

fn parse(input: &str) -> (HashMap<(isize, isize), char>, (isize, isize), String) {
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

pub fn solve2(input: &str) -> usize {
    let (mut grid, mut pos, moves) = parse2(input);

    for m in moves.chars() {
        //        print(&grid, pos);

        let dir = match m {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!("{m:?}"),
        };

        let next = (pos.0 + dir.0, pos.1 + dir.1);
        let tile = grid.get(&next);

        match tile {
            None | Some(&'#') => (),
            Some(&'.') => pos = next,
            Some(&'[') | Some(&']') => {
                if m == '<' || m == '>' {
                    let mut offset = 2;
                    loop {
                        let target = (pos.0 + offset * dir.0, pos.1 + offset * dir.1);
                        match grid.get(&target) {
                            Some(&'.') => {
                                for i in (2..=offset).rev() {
                                    let to = (pos.0 + i * dir.0, pos.1);
                                    let from = (pos.0 + (i - 1) * dir.0, pos.1);
                                    let tile = grid[&from];
                                    grid.insert(to, tile);
                                }
                                grid.insert(next, '.');
                                pos = next;
                                break;
                            }
                            Some(&'[') | Some(&']') => offset += 1,
                            None | Some(&'#') => break,
                            _ => panic!(),
                        }
                    }
                } else {
                    let set = HashSet::from([pos]);
                    if push(&mut grid, set, dir.1) {
                        pos = next;
                    }
                }
            }
            _ => panic!(),
        }
        //        eprintln!("{m}");
        //        print(&grid, pos);
    }

    grid.iter()
        .filter_map(|((x, y), c)| (c == &'[').then_some(x + 100 * y))
        .sum::<isize>() as usize
}

fn push(
    grid: &mut HashMap<(isize, isize), char>,
    set: HashSet<(isize, isize)>,
    dir_y: isize,
) -> bool {
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

fn print(grid: &HashMap<(isize, isize), char>, pos: (isize, isize)) {
    let max_x = grid.keys().map(|k| k.0).max().unwrap();
    let max_y = grid.keys().map(|k| k.1).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            eprint!("{}", if (x, y) == pos { '@' } else { grid[&(x, y)] });
        }
        eprintln!();
    }
}

fn parse2(input: &str) -> (HashMap<(isize, isize), char>, (isize, isize), String) {
    let mut lines = input.lines();

    let mut start = (0, 0);
    let mut grid = HashMap::new();
    let mut y = 0;
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        for (x, c) in line.chars().enumerate() {
            let pos = (2 * x as isize, y as isize);
            if c == '@' {
                grid.insert(pos, '.');
                grid.insert((pos.0 + 1, pos.1), '.');
                start = pos;
            } else if c == '#' {
                grid.insert(pos, '#');
                grid.insert((pos.0 + 1, pos.1), '#');
            } else if c == '.' {
                grid.insert(pos, '.');
                grid.insert((pos.0 + 1, pos.1), '.');
            } else if c == 'O' {
                grid.insert(pos, '[');
                grid.insert((pos.0 + 1, pos.1), ']');
            }
        }
        y += 1;
    }

    let moves = lines.collect();

    (grid, start, moves)
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
