use std::collections::HashSet;
use std::collections::VecDeque;

pub fn part1(input: &str, size: i8, time: usize) -> usize {
    let positions = parse(input);
    distance(&positions[0..time], size).unwrap()
}

pub fn part2(input: &str, size: i8) -> String {
    let positions = parse(input);
    // Binary search for first time at which there is no more path:
    let times: Vec<usize> = (0..positions.len()).collect();
    let time = times.partition_point(|&time| distance(&positions[1..time], size).is_some());
    let (x, y) = positions[time - 1];
    format!("{x},{y}")
}

fn distance(positions: &[(i8, i8)], size: i8) -> Option<usize> {
    let mut seen: HashSet<(i8, i8)> = positions.iter().copied().collect();
    let mut queue = VecDeque::from([((0, 0), 0)]);

    while let Some(((x, y), distance)) = queue.pop_front() {
        if x == size && y == size {
            return Some(distance);
        }

        if x >= 0 && x <= size && y >= 0 && y <= size && seen.insert((x, y)) {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                queue.push_back(((x + dx, y + dy), distance + 1));
            }
        }
    }

    None
}

fn parse(input: &str) -> Vec<(i8, i8)> {
    input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect()
}

#[test]
fn test_part1() {
    assert_eq!(22, part1(include_str!("example.txt"), 6, 12));
    assert_eq!(232, part1(include_str!("input.txt"), 70, 1024));
}

#[test]
fn test_part2() {
    assert_eq!("6,1", part2(include_str!("example.txt"), 6));
    assert_eq!("44,64", part2(include_str!("input.txt"), 70));
}
