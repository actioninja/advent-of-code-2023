use std::collections::{BTreeMap, HashSet};
use std::str::FromStr;
advent_of_code::solution!(4);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    pub id: u32,
    pub winning: HashSet<u32>,
    pub yours: Vec<u32>,
}

impl Card {
    pub fn is_winning(&self, number: u32) -> bool {
        self.winning.contains(&number)
    }

    pub fn score(&self) -> u32 {
        let only_winning: Vec<_> = self.yours.iter().filter(|n| self.is_winning(**n)).collect();

        if only_winning.is_empty() {
            0
        } else {
            let mut score = 1;
            for _ in 1..only_winning.len() {
                score *= 2;
            }
            score
        }
    }

    pub fn bonus_cards(&self) -> Vec<u32> {
        self.yours
            .iter()
            .filter(|n| self.is_winning(**n))
            .enumerate()
            .map(|(i, _)| i as u32)
            .map(|i| self.id + i + 1)
            .collect()
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');
        let id: u32 = split
            .next()
            .unwrap()
            .strip_prefix("Card")
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        let split_numbers = split.next().unwrap().split('|');
        let winning = split_numbers
            .clone()
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let yours = split_numbers
            .clone()
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        Ok(Self { id, winning, yours })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards: Vec<_> = input.lines().map(|l| l.parse::<Card>().unwrap()).collect();
    let scores: Vec<_> = cards.iter().map(|c| c.score()).collect();

    Some(scores.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let card_ref: Vec<_> = input.lines().map(|l| l.parse::<Card>().unwrap()).collect();
    let mut card_counts: Vec<u32> = vec![1; card_ref.len()];

    for card in card_ref.iter() {
        for bonus_card in card.bonus_cards() {
            card_counts[bonus_card as usize - 1] += card_counts[card.id as usize - 1];
        }
    }

    Some(card_counts.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
