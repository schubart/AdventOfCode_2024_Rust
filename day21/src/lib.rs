use std::collections::HashSet;
use std::collections::VecDeque;

type State<const N: usize> = (u8, [char; N]); // Number buttons pushed, bots pointing

pub fn part1<const N: usize>(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<_>>();

            let state: State<N> = (0, ['A'; N]);

            let mut seen = HashSet::new();
            let mut queue = VecDeque::from([(state, 0)]);

            let pushes = loop {
                let (state, count) = queue.pop_front().unwrap();

                if !seen.insert(state) {
                    continue;
                }

                if state.0 as usize == chars.len() {
                    break count; // Pushed the right buttons.
                }

                for b in ['<', '>', '^', 'v', 'A'] {
                    if let Some(next) = next::<N>(state, b, chars[state.0 as usize]) {
                        queue.push_back((next, count + 1));
                    }
                }
            };

            let num = chars[0..(chars.len() - 1)]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            num * pushes
        })
        .sum()
}

fn next<const N: usize>(mut state: State<N>, button: char, next_num: char) -> Option<State<N>> {
    let mut trigger = button;
    for x in (1..N).rev() {
        if trigger != 'A' {
            state.1[x] = directions(state.1[x], trigger)?;
            return Some(state);
        }
        trigger = state.1[x];
    }

    /*
        if state.1[2] != 'A' {
            state.1[1] = directions(state.1[1], state.1[2])?;
            return Some(state);
        }
    */
    if state.1[1] != 'A' {
        state.1[0] = numbers(state.1[0], state.1[1])?;
        return Some(state);
    }

    if state.1[0] == next_num {
        state.0 += 1;
    } else {
        return None;
    }

    Some(state)
}

fn numbers(current: char, button: char) -> Option<char> {
    match (current, button) {
        ('7', '>') => Some('8'),
        ('7', 'v') => Some('4'),

        ('8', '<') => Some('7'),
        ('8', 'v') => Some('5'),
        ('8', '>') => Some('9'),

        ('9', '<') => Some('8'),
        ('9', 'v') => Some('6'),

        ('4', '^') => Some('7'),
        ('4', '>') => Some('5'),
        ('4', 'v') => Some('1'),

        ('5', '^') => Some('8'),
        ('5', '<') => Some('4'),
        ('5', '>') => Some('6'),
        ('5', 'v') => Some('2'),

        ('6', '^') => Some('9'),
        ('6', '<') => Some('5'),
        ('6', 'v') => Some('3'),

        ('1', '^') => Some('4'),
        ('1', '>') => Some('2'),

        ('2', '^') => Some('5'),
        ('2', '<') => Some('1'),
        ('2', '>') => Some('3'),
        ('2', 'v') => Some('0'),

        ('3', '^') => Some('6'),
        ('3', '<') => Some('2'),
        ('3', 'v') => Some('A'),

        ('0', '^') => Some('2'),
        ('0', '>') => Some('A'),

        ('A', '^') => Some('3'),
        ('A', '<') => Some('0'),

        _ => None,
    }
}

fn directions(current: char, button: char) -> Option<char> {
    match (current, button) {
        ('^', '>') => Some('A'),
        ('^', 'v') => Some('v'),

        ('A', '<') => Some('^'),
        ('A', 'v') => Some('>'),

        ('<', '>') => Some('v'),

        ('v', '^') => Some('^'),
        ('v', '<') => Some('<'),
        ('v', '>') => Some('>'),

        ('>', '^') => Some('A'),
        ('>', '<') => Some('v'),

        _ => None,
    }
}

//#[test]
//fn test_part1() {
//    assert_eq!(126384, part1::<{ 2 + 1 }>(include_str!("example.txt")));
//    assert_eq!(188384, part1::<{ 2 + 1 }>(include_str!("input.txt")));
//}

#[test]
fn test_part2() {
    //    assert_eq!(3, part2(include_str!("example.txt")));
    assert_eq!(4, part1::<{25 + 1}>("879A"));
}
