use std::collections::{HashMap, HashSet};

use regex::Regex;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let cards = parse_input(input);

    let part1: u32 = cards.iter().map(calculate_card_points).sum();
    Some(part1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = parse_input(input);

    let mut counter: u32 = 0;
    let mut memo: HashMap<usize, u32> = HashMap::new();
    (0..cards.len()).for_each(|i| win_scratch_cards(&cards, i, &mut counter, &mut memo));
    Some(counter)
}

fn parse_input(input: &str) -> Vec<Card> {
    let re = Regex::new(
        r"Card\s+(?P<card_id>\d+): (?P<winning_numbers>[\d ]+) \| (?P<numbers_you_have>[\d ]+)$",
    )
    .unwrap();
    input
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            let winning_numbers: HashSet<u32> = caps["winning_numbers"]
                .split_whitespace()
                .map(|ele| ele.parse().unwrap())
                .collect();
            let numbers_you_have: Vec<u32> = caps["numbers_you_have"]
                .split_whitespace()
                .map(|ele| ele.parse().unwrap())
                .collect();

            Card {
                _id: caps["card_id"].parse().unwrap(),
                winning_numbers,
                numbers_you_have,
            }
        })
        .collect()
}

fn count_winning_numbers(input: &Card) -> u32 {
    input
        .numbers_you_have
        .iter()
        .filter(|n| input.winning_numbers.contains(n))
        .count() as u32
}

fn calculate_card_points(input: &Card) -> u32 {
    match count_winning_numbers(input) {
        0 => 0,
        count => 2_u32.pow(count - 1),
    }
}

// TODO: I desperately need to keep a context of this recursion
fn _old_win_scratch_cards(cards: &Vec<Card>, current_card_index: usize, counter: &mut u32) {
    *counter += 1;
    let winning_numbers = count_winning_numbers(&cards[current_card_index]);
    (current_card_index + 1..current_card_index + 1 + winning_numbers as usize)
        .for_each(|i| _old_win_scratch_cards(cards, i, counter));
}

// DONE!
// benchmarks show an improvement from 30.16s to 1.378ms :D
// almost 22thousand times faster
fn win_scratch_cards(
    cards: &Vec<Card>,
    current_card_index: usize,
    counter: &mut u32,
    memo: &mut HashMap<usize, u32>,
) {
    *counter += 1;

    if let Some(&cached_result) = memo.get(&current_card_index) {
        // If result is already memoized, use the cached value and return
        *counter += cached_result;
        return;
    }

    let winning_numbers = count_winning_numbers(&cards[current_card_index]);
    let mut subcounter = 0;

    (current_card_index + 1..current_card_index + 1 + winning_numbers as usize).for_each(|i| {
        win_scratch_cards(cards, i, &mut subcounter, memo);
    });

    // Save the result in the memoization map
    memo.insert(current_card_index, subcounter);
    *counter += subcounter;
}

#[derive(Debug)]
struct Card {
    _id: u32,
    winning_numbers: HashSet<u32>,
    numbers_you_have: Vec<u32>,
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
