use std::cmp::Reverse;

pub fn solve(may_fix: bool) -> usize {
    include_str!("input.txt")
        .lines()
        .map(str::split_whitespace)
        .map(|levels| levels.map(|level| level.parse().unwrap()))
        .map(Iterator::collect::<Vec<usize>>)
        .filter(|levels| is_safe(levels) || may_fix && can_fix(levels))
        .count()
}

fn can_fix(levels: &[usize]) -> bool {
    (0..levels.len()).any(|index| {
        let mut reduced_levels = levels.to_vec();
        reduced_levels.remove(index);
        is_safe(&reduced_levels)
    })
}

fn is_safe(levels: &[usize]) -> bool {
    let sorted = levels.is_sorted() || levels.is_sorted_by_key(Reverse);
    let diffs_in_range = levels
        .iter()
        .zip(levels.iter().skip(1))
        .map(|(&level1, &level2)| level1.abs_diff(level2))
        .all(|diff| (1..=3).contains(&diff));

    sorted && diffs_in_range
}

#[test]
fn test_part1() {
    assert_eq!(483, solve(false));
}

#[test]
fn test_part2() {
    assert_eq!(528, solve(true));
}
