use std::iter;

use aoc_runner_derive::aoc;

#[aoc(day11, part1)]
pub fn p1(input: &str) -> i32 {
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

    for i in empty_column_indices.into_iter().rev() {
        for r in grid.iter_mut() {
            r.insert(i, '.');
        }
    }

    for i in empty_row_indices.into_iter().rev() {
        grid.insert(i, vec!['.'; grid[0].len()]);
    }

    let galaxies: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, r)| iter::repeat(y).zip(r.iter().enumerate()))
        .filter(|(_, (_, c))| **c == '#')
        .map(|(y, (x, _))| (x, y))
        .collect();

    galaxies.iter().enumerate().flat_map(|(i, gal)| {
        galaxies.get((i+1)..).unwrap().iter().map(|other| {
            (other.0 as i32 - gal.0 as i32).abs() + (other.1 as i32 - gal.1 as i32).abs()
        })
    })
    .sum()
}

#[aoc(day11, part2)]
pub fn p2(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let empty_row_indices: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter(|(_, r)| r.iter().all(|c| *c == '.'))
        .map(|(i, _)| i)
        .collect();

    let empty_column_indices: Vec<usize> = (0..grid[0].len())
        .filter(|c| grid.iter().all(|r| r[*c] == '.'))
        .collect();

    let galaxies: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, r)| iter::repeat(y).zip(r.iter().enumerate()))
        .filter(|(_, (_, c))| **c == '#')
        .map(|(y, (x, _))| (x, y))
        .collect();

        galaxies.iter().enumerate().flat_map(|(i, gal)| {
            galaxies.get((i+1)..).unwrap().iter().map(|other| {
                let row_range = 
                if gal.0 > other.0 {
                    other.0..gal.0
                } else {
                    gal.0..other.0
                };

                let void_rows = empty_row_indices.iter().filter(|i| (gal.1..other.1).contains(i)).count() as i64;
                let void_columns = empty_column_indices.iter().filter(|i| row_range.contains(i)).count() as i64;
        
                (other.0 as i64 - gal.0 as i64).abs() + (other.1 as i64 - gal.1 as i64).abs() + void_rows * 999999 + void_columns * 999999
            })
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

        let expected_output = 374;
        assert_eq!(expected_output, p1(&input));
    }

    #[test]
    fn test_part2() {
        let input: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

        let expected_output = 8410;
        assert_eq!(expected_output, p2(&input));
    }
}
