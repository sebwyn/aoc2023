use std::iter::repeat;

use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day1, part1)]
pub fn p1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits: Vec<char> = line.chars().filter(char::is_ascii_digit).collect();

            let first = digits.first().unwrap().to_digit(10).unwrap();
            let last = digits.last().unwrap().to_digit(10).unwrap();

            first * 10 + last
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn p2(input: &str) -> u32 {
    const NUM_RES: [&str; 9] = [
        r"(1|one)",
        r"(2|two)",
        r"(3|three)",
        r"(4|four)",
        r"(5|five)",
        r"(6|six)",
        r"(7|seven)",
        r"(8|eight)",
        r"(9|nine)",
    ];

    let res: Vec<Regex> = NUM_RES.iter().map(|s| Regex::new(s).unwrap()).collect();

    input
        .lines()
        .map(|line| {
            let unordered_digits: Vec<(usize, usize)> = res
                .iter()
                .enumerate()
                .map(|(i, re)| {
                    repeat(i+1).zip(re.captures_iter(line).map(|c| c.get(1).unwrap().start()))
                })
                .flatten()
                .collect();

            let first = unordered_digits.iter().min_by_key(|(_, s)| s).unwrap().0 as u32;
            let last = unordered_digits.iter().max_by_key(|(_, s)| s).unwrap().0 as u32;

            first * 10 + last
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let test_input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

        let expected_output = 142;
        assert_eq!(p1(test_input), expected_output);
    }

    #[test]
    fn test_p2() {
        let test_input: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

        let expected_output = 281;
        assert_eq!(p2(test_input), expected_output);
    }
}
