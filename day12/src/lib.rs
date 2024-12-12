use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Scalar = isize;
type Point = (Scalar, Scalar);
type Grid = HashMap<Point, char>;

pub fn part1(input: &str) -> usize {
    let mut grid = parse(input);

    let mut result = 0;
    while !grid.is_empty() {
        result += find_area(&mut grid);
    }

    result
}

fn find_area(grid: &mut Grid) -> usize {
    let (&pos, &c) = grid.iter().next().unwrap();

    let mut seen = std::collections::HashSet::new();
    let mut queue = VecDeque::from([pos]);
    while let Some(pos) = queue.pop_front() {
        if grid.remove(&pos).is_some() {
            seen.insert(pos);
            for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let next = (pos.0 + dir.0, pos.1 + dir.1);
                if grid.get(&next) == Some(&c) {
                    queue.push_back(next);
                }
            }
        }
    }

    let area = seen.len();

    let circ = seen
        .iter()
        .map(|pos| {
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .map(|dir| (pos.0 + dir.0, pos.1 + dir.1))
                .map(|next| if seen.contains(&next) { 0 } else { 1 })
                .sum::<usize>()
        })
        .sum::<usize>();

    area * circ
}

pub fn part2(input: &str) -> usize {
    let mut grid = parse(input);

    let mut result = 0;
    while !grid.is_empty() {
        result += find_area2(&mut grid);
    }

    result
}

fn find_area2(grid: &mut Grid) -> usize {
    let (&pos, &c) = grid.iter().next().unwrap();

    let mut seen = std::collections::HashSet::new();
    let mut queue = VecDeque::from([pos]);
    while let Some(pos) = queue.pop_front() {
        if grid.remove(&pos).is_some() {
            seen.insert(pos);
            for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let next = (pos.0 + dir.0, pos.1 + dir.1);
                if grid.get(&next) == Some(&c) {
                    queue.push_back(next);
                }
            }
        }
    }

    let mut hs1 = HashSet::new();
    let mut hs2 = HashSet::new();
    let mut vs1 = HashSet::new();
    let mut vs2 = HashSet::new();

    for &pos in seen.iter() {
        let next = (pos.0 - 1, pos.1);
        if !seen.contains(&next) {
            vs1.insert(next);
        }

        let next = (pos.0 + 1, pos.1);
        if !seen.contains(&next) {
            vs2.insert(pos);
        }

        let next = (pos.0, pos.1 - 1);
        if !seen.contains(&next) {
            hs1.insert(next);
        }

        let next = (pos.0, pos.1 + 1);
        if !seen.contains(&next) {
            hs2.insert(pos);
        }
    }

    let inv_hs1 = hs1.iter().map(|&(x, y)| (y, x)).collect::<HashSet<_>>();
    let inv_hs2 = hs2.iter().map(|&(x, y)| (y, x)).collect::<HashSet<_>>();
    let fenc = runs(&vs1) + runs(&inv_hs1) + runs(&vs2) + runs(&inv_hs2);

    dbg!(c, fenc);
    seen.len() * fenc
}

fn runs(vs: &HashSet<Point>) -> usize {
    let min_x = vs.iter().map(|p| p.0).min().unwrap();
    let max_x = vs.iter().map(|p| p.0).max().unwrap();

    let mut result = 0;
    for x in min_x..=max_x {
        let mut ys = vs
            .iter()
            .filter_map(|p| (p.0 == x).then_some(p.1))
            .collect::<Vec<_>>();

        if ys.is_empty() {
            continue;
        }
        ys.sort_unstable();

        let gaps = ys
            .iter()
            .zip(ys.iter().skip(1))
            .filter(|(&y1, &y2)| y2 != y1 + 1)
            .count();

        result += gaps + 1;
    }

    result
}

fn parse(input: &str) -> Grid {
    let mut grid = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert((x as Scalar, y as Scalar), c);
        }
    }
    grid
}

#[test]
fn test_part1() {
    assert_eq!(140, part1(include_str!("example.txt")));
    assert_eq!(1450816, part1(include_str!("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(80, part2(include_str!("example.txt")));
    assert_eq!(865662, part2(include_str!("input.txt")));
}
