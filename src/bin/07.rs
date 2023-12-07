extern crate core;

use std::str::FromStr;
advent_of_code::solution!(7);

mod part_one {
    use std::collections::BTreeMap;
    use std::str::FromStr;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Card {
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        J,
        Q,
        K,
        A,
    }

    impl From<char> for Card {
        fn from(c: char) -> Self {
            match c {
                'A' => Card::A,
                'K' => Card::K,
                'Q' => Card::Q,
                'J' => Card::J,
                'T' => Card::Ten,
                '9' => Card::Nine,
                '8' => Card::Eight,
                '7' => Card::Seven,
                '6' => Card::Six,
                '5' => Card::Five,
                '4' => Card::Four,
                '3' => Card::Three,
                '2' => Card::Two,
                _ => panic!("Invalid card: {}", c),
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum HandType {
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Hand {
        pub cards: [Card; 5],
        pub bid: u32,
        pub hand_type: HandType,
    }

    impl Hand {
        fn resolve_hand_type(cards: &[Card; 5]) -> HandType {
            let mut cards = cards.clone();
            cards.sort();
            let mut counts = BTreeMap::new();
            for card in cards.iter() {
                *counts.entry(card).or_insert(0) += 1;
            }
            let mut counts = counts.values().copied().collect::<Vec<_>>();
            counts.sort();
            counts.reverse();
            match &counts[..] {
                [1, 1, 1, 1, 1] => HandType::HighCard,
                [2, 1, 1, 1] => HandType::OnePair,
                [2, 2, 1] => HandType::TwoPair,
                [3, 1, 1] => HandType::ThreeOfAKind,
                [3, 2] => HandType::FullHouse,
                [4, 1] => HandType::FourOfAKind,
                [5] => HandType::FiveOfAKind,
                _ => panic!("Invalid hand: {:?}", cards),
            }
        }
    }

    impl FromStr for Hand {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut split = s.split_whitespace();
            let cards: [Card; 5] = split
                .next()
                .unwrap()
                .chars()
                .map(Card::from)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let hand_type = Hand::resolve_hand_type(&cards);
            let bid = split.next().unwrap().parse().unwrap();

            Ok(Hand {
                cards,
                bid,
                hand_type,
            })
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            let self_cards = self.cards;
            let other_cards = other.cards;

            if self.hand_type != other.hand_type {
                return self.hand_type.cmp(&other.hand_type);
            }

            self_cards
                .iter()
                .zip(other_cards.iter())
                .map(|(self_card, other_card)| self_card.cmp(other_card))
                .find(|&x| x != std::cmp::Ordering::Equal)
                .unwrap_or(std::cmp::Ordering::Equal)
        }
    }

    pub fn execute(input: &str) -> Option<u32> {
        let mut hands = input
            .lines()
            .filter(|x| !x.is_empty())
            .map(Hand::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        hands.sort();

        let scores_sum: u32 = hands
            .iter()
            .enumerate()
            .map(|(usize, hand)| hand.bid * (usize as u32 + 1))
            .sum();

        Some(scores_sum)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    part_one::execute(input)
}

mod part_two {
    use std::collections::{BTreeMap, BTreeSet};
    use std::str::FromStr;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Card {
        J,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Q,
        K,
        A,
    }

    impl From<char> for Card {
        fn from(c: char) -> Self {
            match c {
                'A' => Card::A,
                'K' => Card::K,
                'Q' => Card::Q,
                'J' => Card::J,
                'T' => Card::Ten,
                '9' => Card::Nine,
                '8' => Card::Eight,
                '7' => Card::Seven,
                '6' => Card::Six,
                '5' => Card::Five,
                '4' => Card::Four,
                '3' => Card::Three,
                '2' => Card::Two,
                _ => panic!("Invalid card: {}", c),
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum HandType {
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Hand {
        pub cards: [Card; 5],
        pub bid: u32,
        pub hand_type: HandType,
    }

    impl Hand {
        fn resolve_hand_type(cards: &[Card; 5]) -> HandType {
            // my hacky garbage solution for if there are 5 jokers
            if *cards == [Card::J, Card::J, Card::J, Card::J, Card::J] {
                return HandType::FiveOfAKind;
            }
            let unique_cards = cards
                .iter()
                .filter(|&&x| x != Card::J)
                .collect::<BTreeSet<_>>();
            let mut possible_hands = Vec::new();
            for joker_pretending_to_be in unique_cards {
                let mut cards = *cards;
                for card in cards.iter_mut() {
                    if *card == Card::J {
                        *card = *joker_pretending_to_be;
                    }
                }
                cards.sort();
                let mut counts = BTreeMap::new();
                for card in cards.iter() {
                    *counts.entry(card).or_insert(0) += 1;
                }
                let mut counts = counts.values().copied().collect::<Vec<_>>();
                counts.sort();
                counts.reverse();
                let result = match &counts[..] {
                    [1, 1, 1, 1, 1] => HandType::HighCard,
                    [2, 1, 1, 1] => HandType::OnePair,
                    [2, 2, 1] => HandType::TwoPair,
                    [3, 1, 1] => HandType::ThreeOfAKind,
                    [3, 2] => HandType::FullHouse,
                    [4, 1] => HandType::FourOfAKind,
                    [5] => HandType::FiveOfAKind,
                    _ => panic!("Invalid hand: {:?}", cards),
                };
                possible_hands.push(result);
            }

            *possible_hands.iter().max().unwrap()
        }
    }

    impl FromStr for Hand {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut split = s.split_whitespace();
            let cards: [Card; 5] = split
                .next()
                .unwrap()
                .chars()
                .map(Card::from)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let hand_type = Hand::resolve_hand_type(&cards);
            let bid = split.next().unwrap().parse().unwrap();

            Ok(Hand {
                cards,
                bid,
                hand_type,
            })
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            let self_cards = self.cards;
            let other_cards = other.cards;

            if self.hand_type != other.hand_type {
                return self.hand_type.cmp(&other.hand_type);
            }

            self_cards
                .iter()
                .zip(other_cards.iter())
                .map(|(self_card, other_card)| self_card.cmp(other_card))
                .find(|&x| x != std::cmp::Ordering::Equal)
                .unwrap_or(std::cmp::Ordering::Equal)
        }
    }
    pub fn execute(input: &str) -> Option<u32> {
        let mut hands = input
            .lines()
            .filter(|x| !x.is_empty())
            .map(Hand::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        hands.sort();

        let scores_sum: u32 = hands
            .iter()
            .enumerate()
            .map(|(usize, hand)| hand.bid * (usize as u32 + 1))
            .sum();

        Some(scores_sum)
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    part_two::execute(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
