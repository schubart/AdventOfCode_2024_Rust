use std::collections::{HashMap, VecDeque};

pub fn part1(input: &str, max_cheat: i16) -> usize {
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
        if distance >= *distances.get(&pos).unwrap_or(&i16::MAX) {
            continue;
        }

        distances.insert(pos, distance);

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next = (pos.0 + dx, pos.1 + dy);
            if grid.get(&next) != Some(&'#') {
                queue.push_back((next, distance + 1));
            }
        }
    }

    let mut histogram = HashMap::<i16, usize>::new();
    let area = area(max_cheat);
    for &(x, y) in grid.keys() {
        if let Some(&d1) = distances.get(&(x, y)) {
            for &(dx, dy) in &area {
                if let Some(&d2) = distances.get(&(x + dx, y + dy)) {
                    if d1 < d2 {
                        let saving = d2 - d1 - (dx.abs() + dy.abs());
                        *histogram.entry(saving).or_default() += 1;
                    }
                }
            }
        }
    }

    histogram
        .iter()
        .filter_map(|(&saving, &frequency)| (saving >= 100).then_some(frequency))
        .sum()
}

fn area(max: i16) -> Vec<(i16, i16)> {
    let mut result = Vec::new();
    for dx in -max..=max {
        for dy in -max..=max {
            if dx.abs() + dy.abs() >= 2 && dx.abs() + dy.abs() <= max {
                result.push((dx, dy));
            }
        }
    }
    result
}

#[test]
fn test_area() {
    assert_eq!(8, area(2).len());
}

fn parse(input: &str) -> HashMap<(i16, i16), char> {
    let mut result = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            result.insert((x as i16, y as i16), c);
        }
    }
    result
}

#[test]
fn test_part1() {
    //  assert_eq!(1, part1(include_str!("example.txt")));
    assert_eq!(1338, part1(include_str!("input.txt"), 2));
}

#[test]
fn test_part2() {
    //assert_eq!(3, part1(include_str!("example.txt")));
    assert_eq!(975376, part1(include_str!("input.txt"), 20));
}
