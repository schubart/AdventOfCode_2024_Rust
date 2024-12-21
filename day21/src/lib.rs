use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let grid = parse(input);

    grid.len()
}

pub fn part2(input: &str) -> usize {
    let grid = parse(input);

    grid.len()
}

fn parse(input: &str) -> HashMap<(isize, isize), char> {
    let mut result = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            result.insert((x as isize, y as isize), c);
        }
    }
    result
}

#[test]
fn test_part1() {
    assert_eq!(1, part1(include_str!("example.txt")));
//    assert_eq!(2, part1(include_str!("input.txt")));
}

#[test]
fn test_part2() {
//    assert_eq!(3, part2(include_str!("example.txt")));
//    assert_eq!(4, part2(include_str!("input.txt")));
}
