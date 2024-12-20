use std::collections::{HashMap, VecDeque};

pub fn part1(input: &str) -> usize {
    let mut grid = parse(input);
    let start = grid
        .iter()
        .find_map(|(&pos, &c)| (c == 'S').then_some(pos))
        .unwrap();
    let end = grid
        .iter()
        .find_map(|(&pos, &c)| (c == 'E').then_some(pos))
        .unwrap();
    grid.insert(start, '.');
    grid.insert(end, '.');
    let grid = grid;

    let mut distances = HashMap::new();
    let mut queue = VecDeque::from([(start, 0)]);
    while let Some((pos, distance)) = queue.pop_front() {
        if distance >= *distances.get(&pos).unwrap_or(&usize::MAX) {
            continue;
        }

        distances.insert(pos, distance);

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next = (pos.0 + dx, pos.1 + dy);
            if grid.get(&next) != Some(&'#') {
                queue.push_back((next, distance + 1))
            }
        }
    }

    dbg!(distances.get(&end));

    0
}

pub fn part2(input: &str) -> usize {
    let grid = parse(input);

    grid.len()
}

fn parse(input: &str) -> HashMap<(isize, isize), char> {
    let mut result = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            result.insert((x as isize, y as isize), c);
        }
    }
    result
}

#[test]
fn test_part1() {
    assert_eq!(1, part1(include_str!("example.txt")));
    //    assert_eq!(2, part1(include_str!("input.txt")));
}

#[test]
fn test_part2() {
    //    assert_eq!(3, part2(include_str!("example.txt")));
    //    assert_eq!(4, part2(include_str!("input.txt")));
}
