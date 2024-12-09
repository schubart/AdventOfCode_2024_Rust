// use itertools::Itertools;
use std::collections::HashMap;
use std::iter::repeat_n;

type Id = u16;

pub fn solve(input: &str) -> usize {
    let (mut disk, _) = parse(input);

    while disk.last() == Some(&None) {
        disk.truncate(disk.len() - 1);
    }

    for next in 0..(disk.len()) {
        if disk.get(next) == Some(&None) {
            disk.swap_remove(next);

            while disk.last() == Some(&None) {
                disk.truncate(disk.len() - 1);
            }
        }
    }

    disk.iter()
        .enumerate()
        .filter_map(|(pos, id)| id.map(|id| id as usize * pos))
        .sum()
}

pub fn solve2(input: &str) -> usize {
    let (mut disk, lengths) = parse(input);
    let &id = lengths.keys().max().unwrap();

    'outer: for id in (0..=id).rev() {
        let len = lengths[&id];

        for i in 0..disk.len() {
            if (0..len).all(|offset| disk.get(i + offset) == Some(&None)) {
                for (x, item) in disk.iter_mut().enumerate() {
                    if *item == Some(id) {
                        if x < i {
                            continue 'outer;
                        }
                        *item = None;
                    }
                }
                for j in 0..len {
                    disk[i + j] = Some(id);
                }

                continue 'outer;
            }
        }
    }

    disk.iter()
        .enumerate()
        .filter_map(|(pos, id)| id.map(|id| id as usize * pos))
        .sum()
}

fn parse(input: &str) -> (Vec<Option<Id>>, HashMap<Id, usize>) {
    let mut chars = input.trim().chars();
    let mut disk = Vec::new();
    let mut lengths = HashMap::new();

    let mut id = 0;
    while let Some(c) = chars.next() {
        let len = c.to_digit(10).unwrap() as usize;
        disk.extend(repeat_n(Some(id), len));
        lengths.insert(id, len);
        id += 1;

        let c = chars.next().unwrap_or('0');
        let len = c.to_digit(10).unwrap() as usize;
        disk.extend(repeat_n(None, len));
    }

    (disk, lengths)
}

#[test]
fn test_part1() {
    assert_eq!(1928, solve(include_str!("example.txt")));
    assert_eq!(6332189866718, solve(include_str!("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(2858, solve2(include_str!("example.txt")));
    assert_eq!(6353648390778, solve2(include_str!("input.txt")));
}
