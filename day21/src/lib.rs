//use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<_>>();

            let _num = chars[0..(chars.len() - 1)]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            0
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(126384, part1(include_str!("example.txt")));
    // assert_eq!(2, part1(include_str!("input.txt")));
    // not: 198336 (it's too high)
}

#[test]
fn test_part2() {
    //    assert_eq!(3, part2(include_str!("example.txt")));
    //    assert_eq!(4, part2(include_str!("input.txt")));
}
