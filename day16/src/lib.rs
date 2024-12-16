use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

type Pos = (isize, isize);
type Dir = (isize, isize);
type State = (Pos, Dir);

pub fn part1(input: &str) -> usize {
    let (distances, _pre, end) = solve(input);

    *distances
        .iter()
        .filter_map(|((pos, _dir), dist)| (*pos == end).then_some(dist))
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> usize {
    let (distances, pre, end) = solve(input);

    let short = *distances
        .iter()
        .filter_map(|((pos, _dir), dist)| (*pos == end).then_some(dist))
        .min()
        .unwrap();

    let states = distances
        .iter()
        .filter(|((pos, _dir), _dist)| *pos == end)
        .filter(|((_pos, _dir), dist)| **dist == short)
        .map(|((pos, dir), _dist)| (*pos, *dir));

    let mut visited = HashSet::new();
    for state in states {
        visit(&pre, state, &mut visited);
    }

    visited.len()
}

fn solve(input: &str) -> (HashMap<State, usize>, HashMap<State, HashSet<State>>, Pos) {
    let grid = parse(input);

    let start = grid
        .iter()
        .find_map(|(&pos, &c)| (c == 'S').then_some(pos))
        .unwrap();
    let end = grid
        .iter()
        .find_map(|(&pos, &c)| (c == 'E').then_some(pos))
        .unwrap();

    let start = (start, (1, 0));

    let mut distances = HashMap::from([(start, 0)]);
    let mut queue = BinaryHeap::from([(Reverse(0), start)]);
    let mut pre: HashMap<State, HashSet<State>> = HashMap::new();

    while let Some((Reverse(distance), (pos @ (px, py), dir @ (dx, dy)))) = queue.pop() {
        if grid[&pos] == '#' {
            continue;
        }

        #[rustfmt::skip]
        let next = [
            (distance + 1,    ((px + dx, py + dy), (dx, dy))),
            (distance + 1000, ((px, py),           (dy, dx))),
            (distance + 1000, ((px, py),           (-dy, -dx))),
        ];

        for (next_distance, next_state) in next {
            if next_distance <= *distances.get(&next_state).unwrap_or(&usize::MAX) {
                distances.insert(next_state, next_distance);
                queue.push((Reverse(next_distance), next_state));
                pre.entry(next_state).or_default().insert((pos, dir));
            }
        }
    }

    (distances, pre, end)
}

fn visit(prev: &HashMap<State, HashSet<State>>, start: State, visited: &mut HashSet<Pos>) {
    visited.insert(start.0);

    if let Some(p) = prev.get(&start) {
        for &next in p {
            visit(prev, next, visited);
        }
    }
}

fn parse(input: &str) -> HashMap<(isize, isize), char> {
    let mut grid = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert((x as isize, y as isize), c);
        }
    }
    grid
}

#[test]
fn test_part1() {
    assert_eq!(7036, part1(include_str!("example.txt")));
    assert_eq!(99460, part1(include_str!("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(45, part2(include_str!("example.txt")));
    assert_eq!(500, part2(include_str!("input.txt")));
}
