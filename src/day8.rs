use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use regex::RegexBuilder;

#[derive(Debug)]
pub struct Node {
    left: [char; 3],
    right: [char; 3],
}

#[derive(Debug)]
pub struct Router {
    instructions: Vec<char>,
    network: HashMap<[char; 3], Node>,
}

#[aoc_generator(day8)]
fn create_network(input: &str) -> Router {
    let mut network = HashMap::new();
    let instructions = input.lines().nth(0).unwrap().chars().collect();

    let parser = RegexBuilder::new(r"^([A-Z|1-9]{3}) = \(([A-Z|1-9]{3}), ([A-Z|1-9]{3})\)$")
        .multi_line(true)
        .build()
        .unwrap();

    for capture in parser.captures_iter(input) {
        let key = capture.get(1).unwrap().as_str().chars().collect::<Vec<_>>().try_into().unwrap();
        let left = capture.get(2).unwrap().as_str().chars().collect::<Vec<_>>().try_into().unwrap();
        let right = capture.get(3).unwrap().as_str().chars().collect::<Vec<_>>().try_into().unwrap();
        network.insert(key, Node { left, right });
    }

    Router {
        instructions,
        network,
    }
}

#[aoc(day8, part1)]
pub fn p1(input: &Router) -> usize {
    let mut i = 0;
    let mut current_node = ['A', 'A', 'A'];
    while current_node != ['Z', 'Z', 'Z'] {
        let ii = i % input.instructions.len();
        let direction = input.instructions[ii];

        match direction {
            'L' => current_node = input.network.get(&current_node).unwrap().left,
            'R' => current_node = input.network.get(&current_node).unwrap().right,
            _ => panic!("Unsupported direction!"),
        }

        i += 1;
    }

    i
}

#[aoc(day8, part2)]
pub fn p2(input: &Router) -> usize {
    let mut current_nodes: Vec<&[char; 3]> = input
        .network
        .keys()
        .filter(|s| s[2] == 'A')
        .map(|s| s)
        .collect();

    let num_zs_to_find = current_nodes.len();

    let mut first_found_zs = Vec::new();

    let mut i: usize = 0;
    while first_found_zs.len() < num_zs_to_find {
        let ii = i % input.instructions.len();
        let direction = input.instructions[ii];

        for node in current_nodes.iter() {
            if node[2] == 'Z' {
                first_found_zs.push(i)
            }
        }
        current_nodes.retain(|s| s[2] != 'Z');

        for current_node in current_nodes.iter_mut() {
            match direction {
                'L' => *current_node = &input.network.get(*current_node).unwrap().left,
                'R' => *current_node = &input.network.get(*current_node).unwrap().right,
                _ => panic!("Unsupported direction!"),
            }
        }

        i += 1;
    }
    first_found_zs
        .into_iter()
        .reduce(|acc, e| (acc * e) / gcd(acc, e))
        .unwrap()
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    //turns out the branchless version of this operation is like 25% faster
    while a != b {
        a -= (a > b) as usize * b;
        b -= (b > a) as usize * a;
    }
    return a;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

        let expected_output = 6;
        assert_eq!(expected_output, p1(&create_network(input)));
    }

    #[test]
    fn test_part2() {
        let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

        let expected_output = 6;
        assert_eq!(expected_output, p2(&create_network(input)));
    }
}
