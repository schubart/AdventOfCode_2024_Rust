pub fn part1() -> usize {
    let (mut list1, mut list2) = parse();

    list1.sort_unstable();
    list2.sort_unstable();

    list1
        .iter()
        .zip(list2.iter())
        .map(|(&x1, &x2)| x1.abs_diff(x2))
        .sum()
}

pub fn part2() -> usize {
    let (list1, list2) = parse();

    // Input is only 1000 lines, so don't bother builing a histogram map.
    list1
        .iter()
        .map(|&x1| x1 * list2.iter().filter(|&&x2| x1 == x2).count())
        .sum()
}

fn parse() -> (Vec<usize>, Vec<usize>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in include_str!("input.txt").lines() {
        let mut parts = line.split_whitespace();
        list1.push(parts.next().unwrap().parse().unwrap());
        list2.push(parts.next().unwrap().parse().unwrap());
        assert!(parts.next().is_none());
    }

    (list1, list2)
}

#[test]
fn test_part1() {
    assert_eq!(936063, part1());
}

#[test]
fn test_part2() {
    assert_eq!(23150395, part2());
}
