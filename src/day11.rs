use aoc_runner_derive::aoc;

#[aoc(day11, part1)]
pub fn p1(input: &str) -> i64 {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let empty_row_indices: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter(|(_, r)| r.iter().all(|c| *c == '.'))
        .map(|(i, _)| i)
        .collect();

    let empty_column_indices: Vec<usize> = (0..grid[0].len())
        .filter(|c| grid.iter().all(|r| r[*c] == '.'))
        .collect();

    

    0
}

#[aoc(day11, part2)]
pub fn p2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input: &str = r#""#;

        let expected_output = 8;
        assert_eq!(expected_output, p1(&input));
    }

    #[test]
    fn test_part2() {
        let input: &str = r#""#;

        let expected_output = 10;
        assert_eq!(expected_output, p2(&input));
    }
}
