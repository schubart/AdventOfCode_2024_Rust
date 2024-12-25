pub fn solve(input: &str) -> usize {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for block in input.split("\n\n") {
        let mut counts = vec![-1; 5];
        for line in block.lines() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    counts[x] += 1;
                }
            }
        }

        if block.starts_with('#') {
            locks.push(counts);
        } else {
            keys.push(counts);
        }
    }

    let mut result = 0;
    for key in &keys {
        for lock in &locks {
            if key.iter().zip(lock.iter()).all(|(k, l)| k + l <= 5) {
                result += 1;
            }
        }
    }

    result
}

#[test]
fn test_part1() {
    assert_eq!(3, solve(include_str!("example.txt")));
    assert_eq!(3155, solve(include_str!("input.txt")));
}
