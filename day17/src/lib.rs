use std::collections::VecDeque;

type Registers = (usize, usize, usize);
type Program = Vec<usize>;

pub fn part1(input: &str) -> String {
    let (registers, program) = parse(input);

    run(registers, &program)
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

pub fn part2(input: &str) -> usize {
    let (mut registers, program) = parse(input);

    // For my input, one output value is produced for every 3-bit group in
    // register A. Try finding register A values that produce just the last
    // desired output value, then keep adding 3-bit groups if they produce
    // more of the desired output.

    let mut queue: VecDeque<usize> = (0..8).collect();
    loop {
        registers.0 = queue.pop_front().unwrap();
        let out = run(registers, &program);

        if out == program {
            break registers.0;
        }

        if program.ends_with(&out) {
            queue.extend((0..8).map(|bits| (registers.0 << 3) + bits));
        }
    }
}

#[allow(clippy::assign_op_pattern)]
fn run(registers: Registers, program: &Program) -> Vec<usize> {
    let (mut a, mut b, mut c) = registers;
    let mut pc = 0;
    let mut out = Vec::new();

    while let (Some(&op), Some(&literal)) = (program.get(pc), program.get(pc + 1)) {
        pc += 2;

        let combo = match literal {
            0..=3 => literal,
            4 => a,
            5 => b,
            6 => c,
            _ => panic!("{literal}"),
        };

        match op {
            0 => a = a >> combo,                         // adv
            6 => b = a >> combo,                         // bdv
            7 => c = a >> combo,                         // cdv
            1 => b = b ^ literal,                        // bxl
            4 => b = b ^ c,                              // bxc
            2 => b = combo % 8,                          // bst
            3 => pc = if a != 0 { literal } else { pc }, // jnz
            5 => out.push(combo % 8),                    // out
            _ => panic!("{op}"),
        }
    }

    out
}

fn parse(input: &str) -> (Registers, Program) {
    let mut nums = input
        .split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap());

    let registers = (
        nums.next().unwrap(),
        nums.next().unwrap(),
        nums.next().unwrap(),
    );
    let program = nums.collect();

    (registers, program)
}

#[test]
fn test_part1() {
    assert_eq!("4,6,3,5,6,3,5,2,1,0", part1(include_str!("example.txt")));
    assert_eq!("5,1,4,0,5,1,0,2,6", part1(include_str!("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(202322936867370, part2(include_str!("input.txt")));
}
