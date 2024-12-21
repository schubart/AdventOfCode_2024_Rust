use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let num_buttons = HashMap::<char, (isize, isize)>::from([
        ('7', (0, 0)),
        ('8', (1, 0)),
        ('9', (2, 0)),
        ('4', (0, 1)),
        ('5', (1, 1)),
        ('6', (2, 1)),
        ('1', (0, 2)),
        ('2', (1, 2)),
        ('3', (2, 2)),
        ('0', (1, 3)),
        ('A', (2, 3)),
    ]);

    let mut num = HashMap::<(char, char), Vec<char>>::new();
    for (&c1, (x1, y1)) in num_buttons.iter() {
        for (&c2, (x2, y2)) in num_buttons.iter() {
            let mut seq = Vec::new();
            if y2 < y1 {
                for _ in 0..(y1 - y2) {
                    seq.push('^');
                }
            }
            if x1 < x2 {
                for _ in 0..(x2 - x1) {
                    seq.push('>');
                }
            }
            if x2 < x1 {
                for _ in 0..(x1 - x2) {
                    seq.push('<');
                }
            }
            if y1 < y2 {
                for _ in 0..(y2 - y1) {
                    seq.push('v');
                }
            }
            seq.push('A');
            num.insert((c1, c2), seq);
        }
    }

    let dir_buttons = HashMap::<char, (isize, isize)>::from([
        ('^', (1, 0)),
        ('A', (2, 0)),
        ('<', (0, 1)),
        ('v', (1, 1)),
        ('>', (2, 1)),
    ]);

    let mut dir = HashMap::<(char, char), Vec<char>>::new();
    for (&c1, (x1, y1)) in dir_buttons.iter() {
        for (&c2, (x2, y2)) in dir_buttons.iter() {
            let mut seq = Vec::new();
            if y1 < y2 {
                for _ in 0..(y2 - y1) {
                    seq.push('v');
                }
            }
            if x1 < x2 {
                for _ in 0..(x2 - x1) {
                    seq.push('>');
                }
            }
            if y2 < y1 {
                for _ in 0..(y1 - y2) {
                    seq.push('^');
                }
            }
            if x2 < x1 {
                for _ in 0..(x1 - x2) {
                    seq.push('<');
                }
            }
            seq.push('A');
            dir.insert((c1, c2), seq);
        }
    }

    input
        .lines()
        .map(|line| {
            dbg!(line);
            //                          num  dir  dir
            let mut state = ['A', 'A', 'A'];

            let chars = line.chars().collect::<Vec<_>>();

            let mut seq = String::new();
            let mut count = 0;
            for &c1 in &chars {
                for &c2 in &num[&(state[0], c1)] {
                    for &c3 in &dir[&(state[1], c2)] {
                        for &c4 in &dir[&(state[2], c3)] {
                            count += 1;
                            seq.push(c4);
                        }
                        state[2] = c3;
                    }
                    state[1] = c2;
                }
                state[0] = c1;
            }
            /*
            dbg!(count);
            dbg!("<A^A>^^AvvvA".len());
            dbg!("v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len());
            dbg!("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len());
            */

            let num = chars[0..(chars.len() - 1)]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            dbg!(seq);
            num * count
        })
        .sum()
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
    assert_eq!(126384, part1(include_str!("example.txt")));
    // assert_eq!(2, part1(include_str!("input.txt")));
    // not: 198336 (it's too high)
}

#[test]
fn test_part2() {
    //    assert_eq!(3, part2(include_str!("example.txt")));
    //    assert_eq!(4, part2(include_str!("input.txt")));
}
