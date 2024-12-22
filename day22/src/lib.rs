use itertools::iterate;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Sequence = (i8, i8, i8, i8);

pub fn part1(input: &str) -> isize {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .map(|start| iterate(start, next).nth(2000).unwrap())
        .sum()
}

pub fn part2(input: &str) -> isize {
    let mut totals = HashMap::new();

    for start in input.lines().map(|line| line.parse().unwrap()) {
        let prices = iterate(start, next).map(|x| x % 10);
        let diffs = prices.clone().tuple_windows().map(|(a, b)| (b - a) as i8);

        let mut seen = HashSet::<Sequence>::new();
        for (seq, bananas) in diffs.take(2000).tuple_windows().zip(prices.skip(4)) {
            if seen.insert(seq) {
                *totals.entry(seq).or_default() += bananas;
            }
        }
    }

    totals.values().max().copied().unwrap()
}

#[allow(clippy::trivially_copy_pass_by_ref, clippy::let_and_return)]
const fn next(secret: &isize) -> isize {
    let secret = *secret;
    let secret = secret ^ ((secret * 64) % 16777216);
    let secret = secret ^ ((secret / 32) % 16777216);
    let secret = secret ^ ((secret * 2048) % 16777216);
    secret
}

#[test]
fn test_part1() {
    assert_eq!(37327623, part1(include_str!("example.txt")));
    assert_eq!(20071921341, part1(include_str!("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(23, part2("1\n2\n3\n2024"));
    assert_eq!(2242, part2(include_str!("input.txt")));
}
