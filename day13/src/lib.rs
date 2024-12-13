type Vec2 = (isize, isize);
type Machine = (Vec2, Vec2, Vec2);

pub fn part1(input: &str, error: isize) -> isize {
    input
        .split("\n\n")
        .map(|lines| parse(lines, error))
        .filter_map(|((ax, ay), (bx, by), (px, py))| {
            let a = div(px * by - py * bx, ax * by - ay * bx)?;
            let b = div(px - a * ax, bx)?;

            Some(a * 3 + b)
        })
        .sum()
}

fn div(x: isize, y: isize) -> Option<isize> {
    (x % y == 0).then_some(x / y)
}

fn parse(lines: &str, error: isize) -> Machine {
    let mut nums = lines
        .split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap());

    let a = (nums.next().unwrap(), nums.next().unwrap());
    let b = (nums.next().unwrap(), nums.next().unwrap());
    let p = (nums.next().unwrap(), nums.next().unwrap());

    (a, b, (p.0 + error, p.1 + error))
}

#[test]
fn test_part1() {
    assert_eq!(480, part1(include_str!("example.txt"), 0));
    assert_eq!(39996, part1(include_str!("input.txt"), 0));
}

#[test]
fn test_part2() {
    let error = 10000000000000;
    assert_eq!(73267584326867, part1(include_str!("input.txt"), error));
}
