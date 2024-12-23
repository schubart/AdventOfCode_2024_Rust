use std::collections::BTreeSet;
use std::collections::HashMap;

type Node = &'static str;
type Nodes = BTreeSet<Node>;
type Graph = HashMap<Node, Nodes>;
type Clique = Vec<Node>;

pub fn part1(input: &'static str) -> usize {
    let mut graph = Graph::new();
    for line in input.lines() {
        let (from, to) = line.split_once('-').unwrap();

        // Make all vertices go from smaller to greater node.
        if from < to {
            graph.entry(from).or_default().insert(to);
            graph.entry(to).or_default();
        } else {
            graph.entry(from).or_default();
            graph.entry(to).or_default().insert(from);
        }
    }

    let mut result = 0;
    for n1 in graph.keys() {
        for n2 in &graph[n1] {
            for n3 in &graph[n2] {
                if graph[n1].contains(n3) && [n1, n2, n3].iter().any(|n| n.starts_with('t')) {
                    result += 1;
                }
            }
        }
    }

    result
}

pub fn part2(input: &'static str) -> String {
    let mut graph = Graph::new();
    let mut nodes = Nodes::new();
    for line in input.lines() {
        let (from, to) = line.split_once('-').unwrap();
        graph.entry(from).or_default().insert(to);
        graph.entry(to).or_default().insert(from);
        nodes.insert(from);
        nodes.insert(to);
    }

    let mut result = Clique::new();

    // Non-recursive version of
    // https://en.wikipedia.org/wiki/Bron–Kerbosch_algorithm
    let mut stack = Vec::from([(Clique::new(), nodes, Nodes::new())]);
    while let Some((clique, mut potential, mut excluded)) = stack.pop() {
        if potential.is_empty() && excluded.is_empty() {
            if clique.len() > result.len() {
                result = clique;
            }
        } else {
            for node in potential.clone() {
                let neighbours = &graph[&node];
                stack.push((
                    union(&clique, node),
                    intersection(&potential, neighbours),
                    intersection(&excluded, neighbours),
                ));

                potential.remove(&node);
                excluded.insert(node);
            }
        }
    }

    result.join(",")
}

fn union(nodes: &Clique, node: Node) -> Clique {
    let mut result = nodes.clone();
    result.push(node);
    result
}

fn intersection(nodes1: &Nodes, nodes2: &Nodes) -> Nodes {
    nodes1.intersection(nodes2).copied().collect()
}

#[test]
fn test_part1() {
    assert_eq!(7, part1(include_str!("example.txt")));
    assert_eq!(1108, part1(include_str!("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!("co,de,ka,ta", part2(include_str!("example.txt")));
    assert_eq!(
        "ab,cp,ep,fj,fl,ij,in,ng,pl,qr,rx,va,vf",
        part2(include_str!("input.txt"))
    );
}
