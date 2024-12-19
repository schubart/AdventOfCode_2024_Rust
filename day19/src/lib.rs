pub fn part1(input: &str) -> isize {
    solve(input, isize::signum)
}

pub fn part2(input: &str) -> isize {
    solve(input, std::convert::identity)
}

fn solve(input: &str, summarize: fn(isize) -> isize) -> isize {
    let mut lines = input.lines();

    let towels = lines.next().unwrap().split(", ");
    let towels = towels.map(str::as_bytes).collect::<Vec<_>>();

    lines
        .skip(1)
        .map(str::as_bytes)
        .map(|target| {
            let mut counts = vec![0; target.len() + 1];
            counts[0] = 1;

            for prefix_len in 0..target.len() {
                for towel in &towels {
                    if target[prefix_len..].starts_with(towel) {
                        counts[prefix_len + towel.len()] += counts[prefix_len];
                    }
                }
            }

            counts
        })
        .map(|counts| counts.last().copied().unwrap())
        .map(summarize)
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(6, part1(include_str!("example.txt")));
    assert_eq!(296, part1(include_str!("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(16, part2(include_str!("example.txt")));
    assert_eq!(619970556776002, part2(include_str!("input.txt")));
}
