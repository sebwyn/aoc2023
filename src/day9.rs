use aoc_runner_derive::aoc;

#[aoc(day9, part1)]
pub fn p1(input: &str) -> i64 {
    let sequence_data: Vec<Vec<i64>> = input
        .lines()
        .map(|line| line.split(' ').map(|n| n.parse::<i64>().unwrap()).collect())
        .collect();

    sequence_data.iter().map(|sequence| {
        let mut difference_sequences = vec![sequence.clone()];
        while  difference_sequences.last().unwrap().iter().any(|n| *n != 0) {
            let next_difference: Vec<_> = difference_sequences.last().unwrap().windows(2).map(|v| v[1] - v[0]).collect();
            difference_sequences.push(next_difference);
        }
        difference_sequences.iter().rev().fold(0, |acc, e| acc + e.last().unwrap())
    })
    .sum()
}

#[aoc(day9, part2)]
pub fn p2(input: &str) -> i64 {
    let sequence_data: Vec<Vec<i64>> = input
        .lines()
        .map(|line| line.split(' ').map(|n| n.parse::<i64>().unwrap()).collect())
        .collect();

    sequence_data.iter().map(|sequence| {
        let mut difference_sequences = vec![sequence.clone()];
        while  difference_sequences.last().unwrap().iter().any(|n| *n != 0) {
            let next_difference: Vec<_> = difference_sequences.last().unwrap().windows(2).map(|v| v[1] - v[0]).collect();
            difference_sequences.push(next_difference);
        }
        difference_sequences.iter().rev().fold(0, |acc, e| e.first().unwrap() - acc)
    })
    .sum()
}

#[cfg(test)]
mod test {
    use super::*;

const INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[test]
    fn test_part1() {
        let expected_output = 114;
        assert_eq!(expected_output, p1(&INPUT));
    }

    #[test]
    fn test_part2() {
        let expected_output = 2;
        assert_eq!(expected_output, p2(&INPUT));
    }
}
