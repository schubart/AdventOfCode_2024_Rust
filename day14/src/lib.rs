use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

type Dimension = i32;
type Position = (Dimension, Dimension);
type Direction = (Dimension, Dimension);
type Robot = (Position, Direction);
type Count = u32;
type Time = i32;

pub fn part1(input: &str, width: Dimension, height: Dimension) -> Count {
    let mut quadrant_counts: HashMap<(Ordering, Ordering), Count> = HashMap::new();

    for line in input.lines() {
        let ((px, py), (vx, vy)) = parse(line);

        let x = (px + 100 * vx).rem_euclid(width);
        let y = (py + 100 * vy).rem_euclid(height);

        // Determine quadrant.
        let cmp_x = x.cmp(&(width / 2));
        let cmp_y = y.cmp(&(height / 2));

        if cmp_x != Ordering::Equal && cmp_y != Ordering::Equal {
            *quadrant_counts.entry((cmp_x, cmp_y)).or_default() += 1;
        }
    }

    quadrant_counts.values().product()
}

pub fn part2(input: &str, width: Dimension, height: Dimension) -> Time {
    let robots: Vec<Robot> = input.lines().map(parse).collect();

    // My original solution looked for this pattern:
    //
    //    .
    //   .#.
    //  .###.
    // .#####.
    //
    // but it turns out the solution also appears when all robots occupy
    // distinct positions for the first time. This condition can be tested
    // more cheaply and easily.

    let mut seen: HashSet<Position> = HashSet::new();

    #[allow(clippy::maybe_infinite_iter)]
    (0..)
        .find(|time| {
            seen.clear(); // Reuse `seen` for speed.
            robots.iter().all(|((px, py), (vx, vy))| {
                let x = (px + time * vx).rem_euclid(width);
                let y = (py + time * vy).rem_euclid(height);

                seen.insert((x, y))
            })
        })
        .unwrap()
}

fn parse(line: &str) -> Robot {
    let mut numbers = line
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap());

    (
        (numbers.next().unwrap(), numbers.next().unwrap()),
        (numbers.next().unwrap(), numbers.next().unwrap()),
    )
}

#[test]
fn test_part1() {
    assert_eq!(12, part1(include_str!("example.txt"), 11, 7));
    assert_eq!(218433348, part1(include_str!("input.txt"), 101, 103));
}

#[test]
fn test_part2() {
    assert_eq!(6512, part2(include_str!("input.txt"), 101, 103));
}
