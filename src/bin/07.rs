use core::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    const CARD_ORDER: [char; 13] = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];
    let hands: Vec<(&str, u32)> = input
        .lines()
        .map(|line| {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            (tokens[0], tokens[1].parse::<u32>().unwrap())
        })
        .collect();

    let mut hands_with_score: Vec<(&str, u32, Score)> = hands
        .clone()
        .iter()
        .map(|hand| {
            let score = calculate_hand_score(hand.0, false);
            (hand.0, hand.1, score)
        })
        .collect();

    hands_with_score.sort_by(|a, b| compare_hands(a, b, CARD_ORDER.to_vec()));
    let total_winnings: u32 = hands_with_score
        .iter()
        .enumerate()
        .map(|(i, (_, bid, _))| bid * (i as u32 + 1))
        .sum();
    Some(total_winnings)
}

pub fn part_two(input: &str) -> Option<u32> {
    const CARD_ORDER: [char; 13] = [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];
    let hands: Vec<(&str, u32)> = input
        .lines()
        .map(|line| {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            (tokens[0], tokens[1].parse::<u32>().unwrap())
        })
        .collect();

    let mut hands_with_score: Vec<(&str, u32, Score)> = hands
        .clone()
        .iter()
        .map(|hand| {
            let jokerized_hand = convert_jokers(hand.0);
            let score = calculate_hand_score(&jokerized_hand, true);
            (hand.0, hand.1, score)
        })
        .collect();

    hands_with_score.sort_by(|a, b| compare_hands(a, b, CARD_ORDER.to_vec()));
    let total_winnings: u32 = hands_with_score
        .iter()
        .enumerate()
        .map(|(i, (_, bid, _))| bid * (i as u32 + 1))
        .sum();
    Some(total_winnings)
}

#[derive(Debug, Clone, Copy)]
enum Score {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn highest_card_frequency(hand: &str, filter_jokers: bool) -> (char, i32) {
    if hand == "JJJJJ" {
        return ('J', 5);
    }
    hand.chars()
        .fold(HashMap::new(), |mut acc, cur_card| {
            let count = match acc.get(&cur_card) {
                Some(count) => *count,
                _ => 0,
            };
            acc.insert(cur_card, count + 1);
            acc
        })
        .iter()
        .filter(|entry| !filter_jokers || entry.0 != &'J')
        .max_by_key(|entry| entry.1)
        .map(|(k, v)| (*k, *v))
        .unwrap()
}

fn calculate_hand_score(hand: &str, filter_jokers: bool) -> Score {
    let unique_cards: HashSet<char> = hand.chars().collect();
    match unique_cards.len() {
        1 => Score::FiveOfAKind,
        2 => {
            // could be Score::FourOfAKind or Score::FullHouse
            // Check the Highest Frequency of a card.
            // if it's 1 or 4, then we have FourOfAKind
            // otherwise, it's FullHouse
            match highest_card_frequency(hand, filter_jokers) {
                (_, 4) => Score::FourOfAKind,
                _ => Score::FullHouse,
            }
        }
        3 => {
            // could be three of a kind or two pair
            // Check the Highest Frequency of a card
            // if it's 3, then we have ThreeOfAKind
            // otherwise, it's TwoPairs
            match highest_card_frequency(hand, filter_jokers) {
                (_, 3) => Score::ThreeOfAKind,
                _ => Score::TwoPair,
            }
        }
        4 => Score::OnePair,
        _ => Score::HighCard,
    }
}

fn compare_hands(
    a: &(&str, u32, Score),
    b: &(&str, u32, Score),
    card_order: Vec<char>,
) -> Ordering {
    // Ordering::Equal
    match (a.2 as u32).cmp(&(b.2 as u32)) {
        Ordering::Equal => {
            for (ai, bi) in a.0.chars().zip(b.0.chars()) {
                let left = card_order
                    .iter()
                    .position(|c| c.eq_ignore_ascii_case(&ai))
                    .unwrap();
                let right = card_order
                    .iter()
                    .position(|c| c.eq_ignore_ascii_case(&bi))
                    .unwrap();
                if left != right {
                    return left.cmp(&right);
                }
            }
            Ordering::Equal
        }
        ordering => ordering,
    }
}

fn convert_jokers(hand: &str) -> String {
    if !hand.chars().any(|c| c == 'J') {
        // println!("No jokers found: {}", hand);
        return hand.to_string();
    }

    let (most_frequent, _) = highest_card_frequency(hand, true);
    hand.replace('J', &most_frequent.to_string())
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
