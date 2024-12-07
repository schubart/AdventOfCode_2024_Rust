use rayon::prelude::*;

pub fn solve(input: &str, op_count: usize) -> usize {
    input
        .par_lines()
        .filter_map(|line| {
            let (target, parts) = line.split_once(':').unwrap();
            let target: usize = target.parse().unwrap();
            let parts: Vec<usize> = parts
                .split_whitespace()
                .map(|part| part.parse().unwrap())
                .collect();

            // n ** (m - 1) candidates for n operators and m parts.
            let candidate_count = op_count.pow((parts.len() - 1) as u32);
            (0..=candidate_count)
                .map(|mut candidate| {
                    let mut result = parts[0];
                    for &part in parts.iter().skip(1) {
                        result = match candidate % op_count {
                            0 => result + part,
                            1 => result * part,
                            2 => concat(result, part),
                            _ => unimplemented!(),
                        };
                        candidate /= op_count;
                    }
                    result
                })
                .find(|&result| result == target)
        })
        .sum()
}

const fn concat(mut left: usize, right: usize) -> usize {
    let mut remainder = right;
    while remainder != 0 {
        remainder /= 10;
        left *= 10;
    }
    left + right
}

#[test]
fn test_part1() {
    assert_eq!(3749, solve(include_str!("example.txt"), 2));
    assert_eq!(5512534574980, solve(include_str!("input.txt"), 2));
}

#[test]
fn test_part2() {
    assert_eq!(11387, solve(include_str!("example.txt"), 3));
    assert_eq!(328790210468594, solve(include_str!("input.txt"), 3));
}
