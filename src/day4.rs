use aoc_runner_derive::{aoc, aoc_generator};

pub struct LotteryInfo {
    num_winning: u32,
}

#[aoc_generator(day4)]
pub fn construct_lottery_info(input: &str) -> Vec<LotteryInfo> {
    input
        .lines()
        .map(|line| {
            let (_, numbers) = line.split_once(':').unwrap();
            let (winning, held) = numbers.split_once('|').unwrap();

            let winning_numbers: Vec<u32> = winning
                .trim()
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            let held_numbers: Vec<u32> = held
                .trim()
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            let num_winning = winning_numbers
                .iter()
                .filter(|w| held_numbers.iter().find(|h| w == h).is_some())
                .count() as u32;

            LotteryInfo { num_winning }
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn p1(info: &[LotteryInfo]) -> u32 {
    info.iter()
        .filter(|info| info.num_winning > 0)
        .map(|lottery_info| 2u32.pow(lottery_info.num_winning - 1))
        .sum()
}

#[aoc(day4, part2)]
pub fn p2(info: &[LotteryInfo]) -> u32 {
    let mut card_counts = vec![1u32; info.len()];

    for (i, card_info) in info.iter().enumerate() {
        let num_copies = card_counts[i];
        let first_scratchcard_won = i + 1;
        let last_scratchcard_copy = (i + card_info.num_winning as usize).min(info.len());
        card_counts
            .get_mut(first_scratchcard_won..=last_scratchcard_copy)
            .unwrap()
            .iter_mut()
            .for_each(|c| *c += num_copies);
    }

    card_counts.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn test_part1() {
        let expected = 13;
        assert_eq!(p1(&construct_lottery_info(INPUT)), expected)
    }

    #[test]
    fn test_part2() {
        let expected = 30;
        assert_eq!(p2(&construct_lottery_info(INPUT)), expected)
    }
}
