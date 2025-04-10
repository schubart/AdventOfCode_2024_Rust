use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

type Scalar = isize;
type Position = (Scalar, Scalar);
type Positions = HashSet<Position>;
type Dir = (Scalar, Scalar);
type State = (Position, Dir);
type Cost = isize;
type Costs = HashMap<State, Cost>;
type Count = usize;

pub fn part1(input: &str) -> Cost {
    let (grid, start, end) = parse(input);
    let (_costs, min_cost) = get_costs(&grid, start, end);

    min_cost
}

fn get_costs(grid: &Positions, start: Position, end: Position) -> (Costs, Cost) {
    // https://en.wikipedia.org/wiki/Dijkstra's_algorithm
    let mut costs = HashMap::new();
    let mut queue = BinaryHeap::from([(Reverse(0), (start, (1, 0)))]); // Facing right.

    while let Some((Reverse(cost), state @ ((x, y), (dx, dy)))) = queue.pop() {
        let best_cost = costs.entry(state).or_insert(Cost::MAX);
        if cost < *best_cost && grid.contains(&(x, y)) {
            costs.insert(state, cost);

            #[rustfmt::skip]
            queue.extend([
                (Reverse(cost + 1),    ((x + dx, y + dy), (dx, dy))),   // Forward
                (Reverse(cost + 1000), ((x, y),           (dy, dx))),   // Turn 90 deg
                (Reverse(cost + 1000), ((x, y),           (-dy, -dx))), // Turn -90 deg
            ]);
        }
    }

    // Return lowest cost to end position, ignoring final direction.
    let min_cost = costs
        .iter()
        .filter_map(|((pos, _dir), cost)| (*pos == end).then_some(cost))
        .min()
        .copied()
        .unwrap();

    (costs, min_cost)
}

pub fn part2(input: &str) -> Count {
    let (grid, start, end) = parse(input);
    let (costs, min_cost) = get_costs(&grid, start, end);

    // Breadth-first-search from end back to start.
    let mut queue: VecDeque<(Cost, State)> = costs
        .iter()
        .filter(|&(&(pos, _dir), &cost)| pos == end && cost == min_cost)
        .map(|(&state, &cost)| (cost, state))
        .collect();
    let mut seen = HashSet::new();

    while let Some((cost, (pos @ (px, py), (dx, dy)))) = queue.pop_front() {
        if costs.get(&(pos, (dx, dy))) == Some(&cost) {
            seen.insert(pos);

            // Inverse cost function.
            #[rustfmt::skip]
            queue.extend([
                (cost - 1,    ((px - dx, py - dy), (dx, dy))),   // Backward
                (cost - 1000, ((px, py),           (dy, dx))),   // Turn 90 deg
                (cost - 1000, ((px, py),           (-dy, -dx))), // Turn -90 deg
            ]);
        }
    }

    seen.len()
}

fn parse(input: &str) -> (HashSet<Position>, Position, Position) {
    let mut positions = HashSet::new();
    let mut start = None;
    let mut end = None;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = (x as Scalar, y as Scalar);
            if c != '#' {
                positions.insert(pos);
            }
            if c == 'S' {
                start = Some(pos);
            }
            if c == 'E' {
                end = Some(pos);
            }
        }
    }

    (positions, start.unwrap(), end.unwrap())
}

#[test]
fn test_part1() {
    assert_eq!(7036, part1(include_str!("example1.txt")));
    assert_eq!(11048, part1(include_str!("example2.txt")));
    assert_eq!(99460, part1(include_str!("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(45, part2(include_str!("example1.txt")));
    assert_eq!(64, part2(include_str!("example2.txt")));
    assert_eq!(500, part2(include_str!("input.txt")));
}
