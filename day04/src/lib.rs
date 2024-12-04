use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let chars = parse(input);

    let mut result = 0;
    // Search from every point.
    for (x, y) in chars.keys() {
        // Search in eight directions (nine, really, but this does not matter).
        for dx in -1..=1 {
            for dy in -1..=1 {
                // "XMAS" found at this point, in this direction?
                let found = "XMAS".chars().enumerate().all(|(offset, c)| {
                    let offset = offset as isize;
                    chars.get(&(x + dx * offset, y + dy * offset)) == Some(&c)
                });

                if found {
                    result += 1;
                }
            }
        }
    }
    result
}

pub fn part2(input: &str) -> usize {
    let chars = parse(input);

    let is_ms = |p1, p2| {
        let mut chars = [chars.get(&p1), chars.get(&p2)];
        chars.sort_unstable();
        chars == [Some(&'M'), Some(&'S')]
    };

    chars
        .iter()
        .filter_map(|(&(x, y), &c)| (c == 'A').then_some((x, y)))
        .filter(|&(x, y)| is_ms((x - 1, y - 1), (x + 1, y + 1)))
        .filter(|&(x, y)| is_ms((x - 1, y + 1), (x + 1, y - 1)))
        .count()
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
    assert_eq!(18, part1(include_str!("example.txt")));
    assert_eq!(2534, part1(include_str!("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(9, part2(include_str!("example.txt")));
    assert_eq!(1866, part2(include_str!("input.txt")));
}
