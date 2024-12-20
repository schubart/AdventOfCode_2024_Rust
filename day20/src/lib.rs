use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve(input: &str, max_cheat: i16, min_saving: i16) -> usize {
    // Parse input.
    let mut track = HashSet::new();
    let mut start = None;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = (x as i16, y as i16);
            if c == 'S' {
                start = Some(pos);
            }
            if c != '#' {
                track.insert(pos);
            }
        }
    }

    // Breadth-first search: Distance from start for every point on track.
    let mut queue = VecDeque::from([(start.unwrap(), 0)]);
    let mut distances = HashMap::new();

    while let Some(((x, y), distance)) = queue.pop_front() {
        let max_distance = distances.entry((x, y)).or_insert(i16::MAX);
        if distance < *max_distance {
            *max_distance = distance;

            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let next = (x + dx, y + dy);
                if track.contains(&next) {
                    queue.push_back((next, distance + 1));
                }
            }
        }
    }

    // Pre-calaculate relative positions of cheat candidates.
    let mut candidates = Vec::new();
    for dx in -max_cheat..=max_cheat {
        for dy in -max_cheat..=max_cheat {
            let distance = dx.abs() + dy.abs();
            if distance > 1 && distance <= max_cheat {
                candidates.push((dx, dy));
            }
        }
    }

    // Try cheating from every point on track...
    distances
        .par_iter()
        .map(|(&(x, y), &dist1)| {
            let mut result = 0;

            // ...to every cheat candidate.
            for &(dx, dy) in &candidates {
                if let Some(&dist2) = distances.get(&(x + dx, y + dy)) {
                    if dist1 < dist2 {
                        let saving = dist2 - dist1 - (dx.abs() + dy.abs());
                        if saving >= min_saving {
                            result += 1;
                        }
                    }
                }
            }

            result
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(44, solve(include_str!("example.txt"), 2, 1));
    assert_eq!(1338, solve(include_str!("input.txt"), 2, 100));
}

#[test]
fn test_part2() {
    assert_eq!(285, solve(include_str!("example.txt"), 20, 50));
    assert_eq!(975376, solve(include_str!("input.txt"), 20, 100));
}
