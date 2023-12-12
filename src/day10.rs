use aoc_runner_derive::aoc;
use std::iter;

const PIPE_CONNECTIONS: [(char, [(i64, i64); 2]); 6] = [
        ('|', [(0, -1), (0, 1)]),
        ('-', [(1, 0), (-1, 0)]),
        ('L', [(0, -1), (1, 0)]),
        ('J', [(0, -1), (-1, 0)]),
        ('7', [(0, 1), (-1, 0)]),
        ('F', [(0, 1), (1, 0)]),
    ];

fn opposite_direction(dir: &(i64, i64)) -> (i64, i64) { (-dir.0, -dir.1) }

fn explore_direction(
    grid: &[Vec<char>],
    pos: (i64, i64),
    dir: (i64, i64),
) -> Option<((i64, i64), (i64, i64))> {
    let pipe_pos = (pos.0 + dir.0, pos.1 + dir.1);
    let coming_from = opposite_direction(&dir);

    if pipe_pos.0 < 0 || pipe_pos.1 < 0 {
        return None;
    }
    let p = grid.get(pipe_pos.1 as usize)?.get(pipe_pos.0 as usize)?;

    let pipe_connection = PIPE_CONNECTIONS.iter().find(|(c, _)| c == p)?.1;

    let i = pipe_connection.iter().position(|e| *e == coming_from)?;
    let other: usize = if i == 1 { 0 } else { 1 };

    Some((pipe_connection[other], pipe_pos))
}

#[aoc(day10, part1)]
pub fn p1(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let start_pos = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| iter::repeat(y).zip(row.iter().enumerate()))
        .find(|(_, (_, c))| **c == 'S')
        .map(|(y, (x, _))| (x as i64, y as i64))
        .unwrap();

    let (mut going_to, mut current_pos) = [(0, -1), (0, 1), (1, 0), (-1, 0)]
        .iter()
        .find_map(|dir| explore_direction(&grid, start_pos, *dir))
        .unwrap();
    let mut dist = 1;

    loop {
        dist += 1;
        let Some((next_going_to, next_pos)) = explore_direction(&grid, current_pos, going_to)
        else {
            break;
        };
        going_to = next_going_to;
        current_pos = next_pos;
    }

    dist / 2
}

#[aoc(day10, part2)]
pub fn p2(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut empty_grid = vec![vec![' '; grid[0].len()]; grid.len()];

    let start_pos = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| iter::repeat(y).zip(row.iter().enumerate()))
        .find(|(_, (_, c))| **c == 'S')
        .map(|(y, (x, _))| (x as i64, y as i64))
        .unwrap();

    let (starting_dir, (mut going_to, mut current_pos)) = [(0, -1), (0, 1), (1, 0), (-1, 0)]
        .into_iter()
        .find_map(|dir| explore_direction(&grid, start_pos, dir).map(|a| (dir, a)))
        .unwrap();

    loop {
        empty_grid[current_pos.1 as usize][current_pos.0 as usize] =
            grid[current_pos.1 as usize][current_pos.0 as usize];
        let Some((next_going_to, next_pos)) = explore_direction(&grid, current_pos, going_to)
        else {
            break;
        };
        going_to = next_going_to;
        current_pos = next_pos;
    }

    let ending_dir = opposite_direction(&going_to);

    //modify start position to be the proper pipe type
    let start_char = PIPE_CONNECTIONS.iter().find(|(_, v)| v.contains(&starting_dir) && v.contains(&ending_dir)).map(|(c, _)| *c).unwrap();
    empty_grid[start_pos.1 as usize][start_pos.0 as usize] = start_char;

    let border_crossing_combos = [['F', 'J'], ['L', '7']];

    let mut interior_locations = Vec::new();
    for (y, line) in empty_grid.iter().enumerate() {
        let mut border_crossings = 0;

        let mut cs = line.iter().enumerate();

        while let Some((x, c)) = cs.next() {
            match c {
                //slip into a new state where we continue to iterate until we find 'J' or '7'
                'F' | 'L' => {
                    let opening_match = c;
                    'matching: while let Some((_, c)) = cs.next() {
                        match c {
                            'J' | '7' => {
                                let closing_match = c;
                                if border_crossing_combos
                                    .contains(&[*opening_match, *closing_match])
                                {
                                    border_crossings += 1
                                }
                                break 'matching;
                            }
                            '-' => {}
                            _ => panic!("Unexpected closing character: {c}"),
                        }
                    }
                }
                '|' => border_crossings += 1,
                ' ' if border_crossings % 2 == 1 => interior_locations.push((x, y)),
                ' ' => {}
                _ => panic!("Unexpected Character: {c}"),
            }
        }
    }

    interior_locations.len() as i64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;

        let expected_output = 8;
        assert_eq!(expected_output, p1(&input));
    }

    #[test]
    fn test_part2() {
        let input: &str = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;

        let expected_output = 10;
        assert_eq!(expected_output, p2(&input));
    }
}
