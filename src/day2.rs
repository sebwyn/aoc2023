use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Eq, Debug)]
pub struct Game {
    id: u32,
    sets: Vec<[u32; 3]>,
}

#[aoc_generator(day2)]
pub fn game_generator(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let mut sets = Vec::new();

            let (identifier, data) = line.split_once(':').unwrap();

            let id: u32 = identifier.split_once(' ').unwrap().1.parse().unwrap();

            for set in data.split(';') {
                let mut set_array = [0u32; 3];

                for rgb_value in set.split(',') {
                    let (count, rgb) = rgb_value.trim().split_once(' ').unwrap();
                    
                    let i = ["red", "green", "blue"].into_iter()
                        .position(|c| c == rgb)
                        .unwrap();

                    set_array[i] = count.parse().unwrap();
                }
                sets.push(set_array);
            }

            Game { id, sets }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn p1(input: &[Game]) -> u32 {
    let max_set: [u32; 3] = [12, 13, 14];
    let validate_set = |s: &[u32; 3]| !s.iter().zip(max_set).any(|(&a, b)| a > b);

    input
        .iter()
        .filter(|game| game.sets.iter().all(validate_set))
        .map(|game| game.id)
        .sum()
}

#[aoc(day2, part2)]
pub fn p2(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|game| {
            (0..3)
                .map(|i| game.sets.iter().map(|set| set[i]).max().unwrap())
                .fold(1, |a, b| a * b)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_game_generator() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        let expected_output = vec![
            Game {
                id: 1,
                sets: vec![[4, 0, 3], [1, 2, 6], [0, 2, 0]],
            },
            Game {
                id: 2,
                sets: vec![[0, 2, 1], [1, 3, 4], [0, 1, 1]],
            },
            Game {
                id: 3,
                sets: vec![[20, 8, 6], [4, 13, 5], [1, 5, 0]],
            },
            Game {
                id: 4,
                sets: vec![[3, 1, 6], [6, 3, 0], [14, 3, 15]],
            },
            Game {
                id: 5,
                sets: vec![[6, 3, 1], [1, 2, 2]],
            },
        ];

        assert_eq!(game_generator(input), expected_output);
    }
}
