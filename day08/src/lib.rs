use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    let (grid, antennas) = parse(input);

    let mut nodes = HashSet::new();
    for (a, positions) in antennas {
        for p1 in &positions {
            for p2 in &positions {
                let dx = p1.0 - p2.0;
                let dy = p1.1 - p2.1;

                let node1 = (p1.0 + dx, p1.1 + dy);
                let node2 = (p2.0 + dx, p2.1 + dy);

                for node in [node1, node2] {
                    if let Some(&c) = grid.get(&node) {
                        if c != a {
                            nodes.insert(node);
                        }
                    }
                }
            }
        }
    }

    nodes.len()
}

pub fn solve2(input: &str) -> usize {
    let (grid, antennas) = parse(input);
    let mut nodes = HashSet::new();

    for positions in antennas.values() {
        for p1 in positions {
            for p2 in positions {
                if p1 == p2 {
                    continue;
                }
                let dx = p1.0 - p2.0;
                let dy = p1.1 - p2.1;

                let mut count = 1;
                loop {
                    let node = (p1.0 + dx * count, p1.1 + dy * count);
                    if grid.contains_key(&node) {
                        nodes.insert(node);
                    } else {
                        break;
                    }
                    count += 1;
                }

                let mut count = -1;
                loop {
                    let node = (p1.0 + dx * count, p1.1 + dy * count);
                    if grid.contains_key(&node) {
                        nodes.insert(node);
                    } else {
                        break;
                    }
                    count -= 1;
                }
            }
        }
    }

    nodes.len()
}

fn parse(
    input: &str,
) -> (
    HashMap<(isize, isize), char>,
    HashMap<char, Vec<(isize, isize)>>,
) {
    let mut grid = HashMap::new();
    let mut antennas = HashMap::<char, Vec<(isize, isize)>>::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = (x as isize, y as isize);
            if c != '.' {
                antennas.entry(c).or_default().push(pos);
            }
            grid.insert(pos, c);
        }
    }

    (grid, antennas)
}

#[test]
fn test_part1() {
    assert_eq!(14, solve(include_str!("example.txt")));
    assert_eq!(256, solve(include_str!("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(34, solve2(include_str!("example.txt")));
    assert_eq!(1005, solve2(include_str!("input.txt")));
}
