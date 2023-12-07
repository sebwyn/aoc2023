
use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Class {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    class: Class,
    cards: String,
    ordering: [char; 13]
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let card_value = |a| self.ordering.iter().position(|c| a == *c).unwrap();
        
        match self.class.partial_cmp(&other.class) {
            Some(core::cmp::Ordering::Equal) => {
                for i in 0..5 {
                    let my_card = card_value(self.cards.chars().nth(i).unwrap());
                    let other_card = card_value(other.cards.chars().nth(i).unwrap());

                    if my_card != other_card {
                        return my_card.partial_cmp(&other_card);
                    }
                }
            }
            ord => return ord,
        }
        Some(core::cmp::Ordering::Equal)
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[aoc(day7, part1)]
pub fn p1(input: &str) -> u32 {
    let mut hands: Vec<(Hand, u32)> = input.lines().map(|line| {
        let (hand, bid) = line.split_once(' ').unwrap();

        let mut freqs = HashMap::new();
        for c in hand.chars() {
            *freqs.entry(c).or_insert(0) += 1;
        }
        let freq_vec: Vec<(char, u32)> = freqs.into_iter().collect();
    
        let class = match freq_vec.len() {
            5 => { Class::HighCard },
            4 => { Class::OnePair },
            3 => { if freq_vec.iter().find(|(_, count)| *count == 3).is_some() { Class::ThreeOfAKind } else { Class::TwoPair } },
            2 => { if freq_vec.iter().find(|(_, count)| *count == 4).is_some() { Class::FourOfAKind } else { Class::FullHouse } },
            1 => { Class::FiveOfAKind },
            _ => panic!("Unsupported hand!")
        };

        let hand = Hand {
            class,
            cards: hand.to_string(),
            ordering: ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A']
        };

        (hand, bid.parse().unwrap())
    })
    .collect();
    hands.sort_by(|(h1, _), (h2, _)| h1.cmp(h2));
    hands.iter().enumerate().map(|(rank, (_, bid))| (rank as u32 + 1) * bid).fold(0, |acc, e| acc + e)
}

#[aoc(day7, part2)]
pub fn p2(input: &str) -> u32 {
    let mut hands: Vec<(Hand, u32)> = input.lines().map(|line| {
        let (hand, bid) = line.split_once(' ').unwrap();

        let mut freqs = HashMap::new();
        for c in hand.chars() {
            *freqs.entry(c).or_insert(0) += 1;
        }
        let mut freq_vec: Vec<(char, u32)> = freqs.into_iter().collect();

        //added for part 2
        freq_vec.sort_by_key(|(_, count)| *count);
        freq_vec.reverse();
        
        if let Some(j_pos) = freq_vec.iter().position(|(c, count)| *c == 'J' && *count != 5) {    
            let (_,  j_count) = freq_vec.remove(j_pos);    
            freq_vec.first_mut().unwrap().1 += j_count;
        }

        let class = match freq_vec.len() {
            5 => { Class::HighCard },
            4 => { Class::OnePair },
            3 => { if freq_vec.iter().find(|(_, count)| *count == 3).is_some() { Class::ThreeOfAKind } else { Class::TwoPair } },
            2 => { if freq_vec.iter().find(|(_, count)| *count == 4).is_some() { Class::FourOfAKind } else { Class::FullHouse } },
            1 => { Class::FiveOfAKind },
            _ => panic!("Unsupported hand!")
        };

        let hand = Hand {
            class,
            cards: hand.to_string(),
            ordering: ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'] //modified for part 2
        };

        (hand, bid.parse().unwrap())
    })
    .collect();
    hands.sort_by(|(h1, _), (h2, _)| h1.cmp(h2));
    hands.iter().enumerate().map(|(rank, (_, bid))| (rank as u32 + 1) * bid).fold(0, |acc, e| acc + e)
}

#[cfg(test)]
mod test {
    use super::*;

const INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn test_part1() {
        let expected_output = 6440;
        assert_eq!(expected_output, p1(INPUT));
    }

    #[test]
    fn test_part2() {
        let expected_output = 5905;
        assert_eq!(expected_output, p2(INPUT));
    }
}
