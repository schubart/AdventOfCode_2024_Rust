use regex::Regex;

pub fn part1(conditional: bool) -> usize {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut enabled = true;
    let mut result = 0;

    for captures in re.captures_iter(include_str!("input.txt")) {
        match captures.get(0).unwrap().as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ if !conditional || enabled => {
                let f1: usize = captures.get(1).unwrap().as_str().parse().unwrap();
                let f2: usize = captures.get(2).unwrap().as_str().parse().unwrap();

                result += f1 * f2;
            }
            _ => (),
        }
    }

    result
}

#[test]
fn test_part1() {
    assert_eq!(187194524, part1(false));
}

#[test]
fn test_part2() {
    assert_eq!(127092535, part1(true));
}
