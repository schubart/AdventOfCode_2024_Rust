use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Pos = (isize, isize);
type Dir = (isize, isize);
type State = (Pos, Dir);

pub fn part1(input: &str) -> isize {
    let (distances, end) = solve(input);

    *distances
        .iter()
        .filter_map(|((pos, _dir), dist)| (*pos == end).then_some(dist))
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> usize {
    let grid = parse(input);
    let (distances, end) = solve(input);

    let short = *distances
        .iter()
        .filter_map(|((pos, _dir), dist)| (*pos == end).then_some(dist))
        .min()
        .unwrap();

    let mut queue: VecDeque<(State, isize)> = VecDeque::new();

    for (&state @ (pos, _dir), &distance) in &distances {
        if pos == end && distance == short {
            queue.push_back((state, distance));
        }
    }
    let mut visited = HashSet::new();

    while let Some(((pos @ (px, py), (dx, dy)), distance)) = queue.pop_front() {
        if grid[&pos] == '#' {
            continue;
        }
        visited.insert(pos);

        #[rustfmt::skip]
        let prev = [
            (distance - 1,    ((px - dx, py - dy), (dx, dy))),
            (distance - 1000, ((px, py),           (dy, dx))),
            (distance - 1000, ((px, py),           (-dy, -dx))),
        ];

        for (prev_dist, prev_state) in prev {
            if distances.get(&prev_state) == Some(&prev_dist) {
                queue.push_back((prev_state, prev_dist));
            }
        }
    }

    visited.len()
}

fn solve(input: &str) -> (HashMap<State, isize>, Pos) {
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

    while let Some((Reverse(distance), (pos @ (px, py), (dx, dy)))) = queue.pop() {
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
            if next_distance <= *distances.get(&next_state).unwrap_or(&isize::MAX) {
                distances.insert(next_state, next_distance);
                queue.push((Reverse(next_distance), next_state));
            }
        }
    }

    (distances, end)
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
//    assert_eq!(45, part2(include_str!("example.txt")));
    assert_eq!(500, part2(include_str!("input.txt")));
}
