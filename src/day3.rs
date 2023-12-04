use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
fn p1(input: &str) -> u32 {
    let line_length = input.lines().nth(0).unwrap().len() + 1;

    let mut sum = 0u32;

    let mut i = 0;
    while i < input.len() {
        if input.as_bytes()[i].is_ascii_digit() {
            let start = i;
            let num = input
                .get(start..)
                .unwrap()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>();
            i += num.len();

            let search_start = start.checked_sub(1).unwrap_or(0);
            let search_end = i + 1; //assume adding by one from the end of a number will always keep us on the same line (true because newline characters)

            let has_surrounding_symbol = [
                (search_start.checked_sub(line_length).unwrap_or(0)
                    ..search_end.checked_sub(line_length).unwrap_or(0)),
                (search_start..search_end),
                (search_start + line_length..search_end + line_length),
            ]
            .into_iter()
            .flatten()
            .filter_map(|i| input.as_bytes().get(i))
            .any(|&c| !(c == b'.' || c == b'\n' || c.is_ascii_digit()));

            if has_surrounding_symbol {
                sum += num.parse::<u32>().unwrap()
            }
        } else {
            i += 1;
        }
    }

    sum
}

#[aoc(day3, part2)]
fn p2(input: &str) -> u32 {
    let line_length = input.lines().nth(0).unwrap().len() + 1;

    let mut asterisks_to_surrounding_numbers: HashMap<usize, Vec<u32>> = HashMap::new();

    let mut i = 0;
    while i < input.len() {
        if input.as_bytes()[i].is_ascii_digit() {
            let start = i;
            let num = input
                .get(start..)
                .unwrap()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>();
            i += num.len();

            let search_start = start.checked_sub(1).unwrap_or(0);
            let search_end = i + 1; //assume adding by one from the end of a number will always keep us on the same line (true because newline characters)

            let Some((surrounding_asterisk_position, _)) = [
                (search_start.checked_sub(line_length).unwrap_or(0)
                    ..search_end.checked_sub(line_length).unwrap_or(0)),
                (search_start..search_end),
                (search_start + line_length..search_end + line_length),
            ]
            .into_iter()
            .flatten()
            .filter_map(|i| input.as_bytes().get(i).map(|c| (i, c)))
            .find(|(_, &c)| c == '*' as u8) else {
                continue;
            };

            asterisks_to_surrounding_numbers
                .entry(surrounding_asterisk_position)
                .or_insert(Vec::new())
                .push(num.parse().unwrap());
        } else {
            i += 1;
        }
    }

    asterisks_to_surrounding_numbers
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+..58
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn test_p1() {
        let expected_ouput = 4361;
        assert_eq!(p1(TEST_INPUT), expected_ouput);
    }

    #[test]
    fn test_p2() {
        let expected_ouput = 467835;
        assert_eq!(p2(TEST_INPUT), expected_ouput);
    }
}
