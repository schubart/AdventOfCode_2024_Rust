use memoize::memoize;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

type Position = (isize, isize);
type Button = char;
type Cost = usize;

const NUM_PAD: &str = "789\n456\n123\n 0A";
const DIR_PAD: &str = " ^A\n<v>\n";

pub fn solve(input: &str, robots: usize) -> Cost {
    input
        .lines()
        .map(|line| {
            let sequence = format!("A{line}"); // Initially point at "A" on num pad.
            let cost: usize = sequence
                .chars()
                .zip(sequence.chars().skip(1))
                .map(|(from, to)| get_cost(from, to, robots, NUM_PAD))
                .sum();
            let numeric: usize = line.strip_suffix('A').unwrap().parse().unwrap();

            cost * numeric
        })
        .sum()
}

#[memoize]
fn get_cost(from: Button, to: Button, robots: usize, pad: &'static str) -> Cost {
    // No robots involved? Just push the target button.
    // Already pointing at target button? Just push it.
    if robots == 0 || from == to {
        return 1;
    }

    // Figure out layout of the pad the robot is pointing at (numeric or directional).
    let (valid_positions, from_pos, to_pos) = parse_pad(pad, from, to);

    // https://en.wikipedia.org/wiki/Dijkstra's_algorithm
    // Search space is:
    // * The position the robot is pointing at (on a numeric or directional pad), and
    // * the last button that was pressed on the robot's directional pad.
    let mut queue = BinaryHeap::from([(Reverse(0), (from_pos, 'A'))]);
    let mut costs = HashMap::new();

    while let Some((Reverse(cost), state @ ((x, y), button))) = queue.pop() {
        let best_cost = costs.entry(state).or_insert(Cost::MAX);
        if cost < *best_cost {
            *best_cost = cost;

            for (next_button, next_pos) in [
                ('<', (x - 1, y)),
                ('>', (x + 1, y)),
                ('^', (x, y - 1)),
                ('v', (x, y + 1)),
                ('A', (x, y)),
            ] {
                if valid_positions.contains(&next_pos) {
                    queue.push((
                        Reverse(cost + get_cost(button, next_button, robots - 1, DIR_PAD)),
                        (next_pos, next_button),
                    ));
                }
            }
        }
    }

    // End state: Robot points at desired position and "A" was pressed on its directional pad.
    costs[&(to_pos, 'A')]
}

fn parse_pad(
    pad: &'static str,
    from: Button,
    to: Button,
) -> (HashSet<Position>, Position, Position) {
    let mut from_pos = None;
    let mut to_pos = None;
    let mut grid = HashSet::new();
    for (y, line) in pad.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = (x as isize, y as isize);
            if c == from {
                from_pos = Some(pos);
            }
            if c == to {
                to_pos = Some(pos);
            }
            if c != ' ' {
                grid.insert(pos);
            }
        }
    }

    (grid, from_pos.unwrap(), to_pos.unwrap())
}

#[test]
fn test_part1() {
    assert_eq!(126384, solve(include_str!("example.txt"), 2 + 1));
    assert_eq!(188384, solve(include_str!("input.txt"), 2 + 1));
}

#[test]
fn test_part2() {
    assert_eq!(232389969568832, solve(include_str!("input.txt"), 25 + 1));
}
