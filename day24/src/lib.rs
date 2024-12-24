use std::collections::BTreeMap;
use std::collections::HashMap;

type Wire = &'static str;
type States = BTreeMap<Wire, usize>;
type Operation = &'static str;
type Gate = ((Wire, Wire), Operation);
type GatesVec = Vec<(Gate, Wire)>;
type GatesMap = HashMap<Gate, Wire>;
type Fixes = BTreeMap<Wire, Wire>;

pub fn part1(input: &'static str) -> usize {
    let (mut states, mut gates) = parse(input);

    while !gates.is_empty() {
        gates.retain(|(((in1, in2), op), out)| {
            if let (Some(&in1), Some(&in2)) = (states.get(in1), states.get(in2)) {
                // Both inputs are known, apply operation.
                let value = match *op {
                    "AND" => in1 & in2,
                    "OR" => in1 | in2,
                    "XOR" => in1 ^ in2,
                    _ => panic!(),
                };
                states.insert(out, value);

                false // Do not retain this gate.
            } else {
                true // Retain this gate until inputs known.
            }
        });
    }

    states
        .iter()
        .rev()
        .filter_map(|(wire, state)| wire.starts_with('z').then_some(state))
        .copied()
        .reduce(|result, bit| (result << 1) + bit)
        .unwrap()
}

pub fn part2(input: &'static str) -> String {
    let (_states, gates) = parse(input);

    // https://en.wikipedia.org/wiki/Adder_(electronics)
    //
    // The system tries to add two 45-bit binary numbers. It has a half adder
    // (using two gates) for the first pair of input bits and 44 full adders
    // (each using five gates) for the remaining pairs of input bits. Check
    // that input has the expected number of gates.
    assert_eq!(gates.len(), 2 + 44 * 5);

    // Manual solution: Run `check_half_adder` and `check_full_adder` for all
    // bits until some assertion fails. Inspect input, find a wire swap that
    // will make the assertion pass, add it to the list here. Repeat until
    // no more assertions fail.
    let (fixes, gates) = apply_fixes(
        gates,
        &[
            ("z16", "fkb"),
            ("nnr", "rqf"),
            ("rdn", "z31"),
            ("rrn", "z37"),
        ],
    );

    let mut carry = check_half_adder(&gates, 0);
    for bit in 1..=44 {
        carry = check_full_adder(&gates, bit, carry);
    }

    fixes.keys().copied().collect::<Vec<_>>().join(",")
}

fn check_half_adder(gates: &GatesMap, bit: usize) -> Wire {
    let bit = |prefix: char| format!("{prefix}{bit:02}");

    let xor = gates[&(sort(&bit('x'), &bit('y')), "XOR")];
    let carry = gates[&(sort(&bit('x'), &bit('y')), "AND")];
    assert_eq!(xor, bit('z'));

    carry
}

fn check_full_adder(gates: &GatesMap, bit: usize, carry: Wire) -> Wire {
    let bit = |prefix: char| format!("{prefix}{bit:02}");

    let xor1 = gates[&(sort(&bit('x'), &bit('y')), "XOR")];
    let and1 = gates[&(sort(&bit('x'), &bit('y')), "AND")];
    let xor2 = gates[&(sort(xor1, carry), "XOR")];
    let and2 = gates[&(sort(xor1, carry), "AND")];
    let carry = gates[&(sort(and1, and2), "OR")];
    assert_eq!(xor2, bit('z'));

    carry
}

// Normalize order of two wires by sorting them by name.
fn sort<'a>(wire1: &'a str, wire2: &'a str) -> (&'a str, &'a str) {
    if wire1 < wire2 {
        (wire1, wire2)
    } else {
        (wire2, wire1)
    }
}

fn apply_fixes(gates_vec: GatesVec, pairs: &[(Wire, Wire)]) -> (Fixes, GatesMap) {
    // Build map of fixes.
    let mut fixes = Fixes::new();
    for &(a, b) in pairs {
        fixes.insert(a, b);
        fixes.insert(b, a);
    }

    // Apply fixes, build gates map from gates vector.
    let mut gates_map = GatesMap::new();
    for (gate, out) in gates_vec {
        gates_map.insert(gate, *fixes.get(&out).unwrap_or(&out));
    }

    (fixes, gates_map)
}

fn parse(input: &'static str) -> (States, GatesVec) {
    let mut lines = input.lines();

    let mut states = States::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let (node, state) = line.split_once(": ").unwrap();
        states.insert(node, state.parse().unwrap());
    }

    let mut gates = GatesVec::new();
    for line in lines {
        let mut parts = line.split_whitespace();
        let in1 = parts.next().unwrap();
        let op = parts.next().unwrap();
        let in2 = parts.next().unwrap();
        let out = parts.nth(1).unwrap();

        gates.push(((sort(in1, in2), op), out));
    }

    (states, gates)
}

#[test]
fn test_part1() {
    assert_eq!(2024, part1(include_str!("example.txt")));
    assert_eq!(53325321422566, part1(include_str!("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(
        "fkb,nnr,rdn,rqf,rrn,z16,z31,z37",
        part2(include_str!("input.txt"))
    );
}
