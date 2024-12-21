use std::collections::HashSet;
use std::collections::VecDeque;

type State = ([Option<char>; 4], [char; 3]); // Number buttons pushed, bots pointing

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<_>>();

            let state: State = ([None, None, None, None], ['A', 'A', 'A']);

            let mut seen = HashSet::new();
            let mut queue = VecDeque::from([(state, 0)]);

            let pushes = loop {
                let (state, count) = queue.pop_front().unwrap();
                
                if !seen.insert(state) {
                    continue;
                }

                eprintln!("{state:?}");

                if chars
                    .iter()
                    .zip(state.0.iter().copied())
                    .any(|(&b1, b2)| b2.is_some() && b2 != Some(b1))
                {
                    continue; // Pushed a wrong button.
                }

                if state.0
                    == [
                        Some(chars[0]),
                        Some(chars[1]),
                        Some(chars[2]),
                        Some(chars[3]),
                    ]
                {
                    break count; // Pushed the right buttons.
                }

                for b in ['<', '>', '^', 'v', 'A'] {
                    if let Some(next) = next(state, b) {
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

fn next(mut state: State, button: char) -> Option<State> {
    if button == 'A' {
        if state.1[2] == 'A' {
            if state.1[1] == 'A' {
                state.0 = match (state.0[0], state.0[1], state.0[2], state.0[3]) {
                    (None, None, None, None) => [Some(state.1[0]), None, None, None],
                    (x, None, None, None) => [x, Some(state.1[0]), None, None],
                    (x, y, None, None) => [x, y, Some(state.1[0]), None],
                    (x, y, z, None) => [x, y, z, Some(state.1[0])],
                    _ => panic!(),
                };
            } else {
                state.1[0] = numbers(state.1[0], state.1[1])?;
            }
        } else {
            state.1[1] = directions(state.1[1], state.1[2])?;
        }
    } else {
        state.1[2] = directions(state.1[2], button)?;
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

/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
 */

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

/*
    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
 */

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
