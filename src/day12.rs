use std::iter;

use itertools::Itertools;

use aoc_runner_derive::aoc;

fn validate_springs(springs: &[u8], seqs: &[usize]) -> bool {
    let mut edges = vec![0];
    edges.extend(
        (1..)
            .zip(springs.windows(2))
            .filter(|(_, v)| v[0] != v[1])
            .map(|(i, _)| i),
    );
    edges.push(springs.len());

    let groups: Vec<usize> = edges
        .windows(2)
        .filter(|v| springs[v[0]] == b'#')
        .map(|v| v[1] - v[0])
        .collect();

    groups.len() == seqs.len() && groups.iter().zip(seqs.iter()).all(|(a, b)| a == b)
}

#[aoc(day12, part1)]
pub fn p1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (springs, seqs) = line.split_once(' ').unwrap();
            let seqs: Vec<usize> = seqs.split(',').map(|s| s.parse().unwrap()).collect();

            let existing_broken = springs.chars().filter(|c| *c == '#').count();
            let mystery_indices: Vec<usize> = springs
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '?')
                .map(|(i, _)| i)
                .collect();

            let expected_broken: usize = seqs.iter().sum();

            let springs = springs.replace('?', ".");
            let bytes: Vec<u8> = Vec::from(springs.as_bytes());

            mystery_indices
                .iter()
                .combinations(expected_broken - existing_broken)
                .map(|choices| {
                    let mut bytes = bytes.clone();
                    for choice in choices {
                        bytes[*choice] = b'#';
                    }
                    bytes
                })
                .filter(|bytes| validate_springs(bytes, &seqs))
                .count()
        })
        .sum()
}

#[aoc(day12, part2)]
pub fn p2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (springs, seqs) = line.split_once(' ').unwrap();
            let seqs: Vec<usize> = iter::repeat(seqs.split(',').map(|s| s.parse().unwrap()))
                .take(5)
                .flatten()
                .collect();

            let springs: String = (0..9)
                .map(|i| if i % 2 == 0 { springs } else { "?" })
                .collect();

            let existing_broken = springs.chars().filter(|c| *c == '#').count();
            let mystery_indices: Vec<usize> = springs
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '?')
                .map(|(i, _)| i)
                .collect();

            let expected_broken: usize = seqs.iter().sum();

            let springs = springs.replace('?', ".");
            let bytes: Vec<u8> = Vec::from(springs.as_bytes());

            mystery_indices
                .iter()
                .combinations(expected_broken - existing_broken)
                .map(|choices| {
                    let mut bytes = bytes.clone();
                    for choice in choices {
                        bytes[*choice] = b'#';
                    }
                    bytes
                })
                .filter(|bytes| validate_springs(bytes, &seqs))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_validate_springs() {
        let inputs: Vec<(&[u8], &[usize])> = vec![
            (b"#.#.###", &[1, 1, 3]),
            (b".#...#....###.", &[1, 1, 3]),
            (b".#.###.#.######", &[1, 3, 1, 6]),
            (b"####.#...#...", &[4, 1, 1]),
            (b"#....######..#####.", &[1, 6, 5]),
            (b".###.##....#", &[3, 2, 1]),
        ];

        for input in inputs {
            assert!(validate_springs(input.0, input.1))
        }
    }

    #[test]
    fn test_part1() {
        let input: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

        let expected_output = 21;
        assert_eq!(expected_output, p1(&input));
        assert!(false);
    }

    #[test]
    fn test_part2() {
        let input: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

        let expected_output = 525152;
        assert_eq!(expected_output, p2(&input));
    }
}
