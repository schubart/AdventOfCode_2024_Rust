use memoize::memoize;

pub fn solve(input: &str, blink: usize) -> usize {
    input
        .split_whitespace()
        .map(|stone| stone.parse().unwrap())
        .map(|stone| evolve(blink, stone))
        .sum()
}

#[memoize]
fn evolve(blink: usize, stone: usize) -> usize {
    if blink == 0 {
        1
    } else {
        let blink = blink - 1;
        
        if stone == 0 {
            evolve(blink, 1)
        } else if let Some((s1, s2)) = split(stone) {
            evolve(blink, s1) + evolve(blink, s2)
        } else {
            evolve(blink, stone * 2024)
        }
    }
}

const fn split(stone: usize) -> Option<(usize, usize)> {
    let digits = stone.ilog10() + 1;
    
    if digits % 2 == 0 {
        let divisor = 10_usize.pow(digits / 2);
        Some((stone / divisor, stone % divisor))
    } else {
        None
    }
}

#[test]
fn test_part1() {
    assert_eq!(55312, solve(include_str!("example.txt"), 25));
    assert_eq!(183248, solve(include_str!("input.txt"), 25));
}

#[test]
fn test_part2() {
    assert_eq!(218811774248729, solve(include_str!("input.txt"), 75));
}
