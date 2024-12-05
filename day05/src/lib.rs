use std::cmp::Ordering::Equal;
use std::cmp::Ordering::Less;
use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    let (pairs, updates) = parse(input);

    updates
        .iter()
        .filter(|pages| pages.is_sorted_by(|&a, &b| !pairs.contains(&(b, a))))
        .map(|pages| pages[pages.len() / 2])
        .sum()
}

pub fn part2(input: &str) -> usize {
    let (pairs, mut updates) = parse(input);

    updates
        .iter_mut()
        .filter(|pages| !pages.is_sorted_by(|&a, &b| !pairs.contains(&(b, a))))
        .map(|pages| {
            pages.sort_by(|&a, &b| if pairs.contains(&(a, b)) { Less } else { Equal });
            pages[pages.len() / 2]
        })
        .sum()
}

fn parse(input: &str) -> (HashSet<(usize, usize)>, Vec<Vec<usize>>) {
    let mut lines = input.lines();

    let mut pairs: HashSet<(usize, usize)> = HashSet::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }

        let (l, r) = line.split_once('|').unwrap();
        pairs.insert((l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()));
    }

    let updates = lines
        .map(|line| line.split(',').map(|page| page.parse().unwrap()).collect())
        .collect();

    (pairs, updates)
}

#[test]
fn test_part1() {
    assert_eq!(143, part1(include_str!("example.txt")));
    assert_eq!(5588, part1(include_str!("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(123, part2(include_str!("example.txt")));
    assert_eq!(5331, part2(include_str!("input.txt")));
}
