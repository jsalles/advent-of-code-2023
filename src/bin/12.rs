use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let problems = parse_input(input);
    let mut memo = HashMap::new();
    let result = problems
        .iter()
        .map(|(code, groups)| count_arrangements(code, groups, &mut memo))
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let problems = parse_input(input);
    let mut memo = HashMap::new();
    let result = problems
        .iter()
        .map(|(code, groups)| {
            count_arrangements(&[*code; 5].join("?"), &groups.repeat(5), &mut memo)
        })
        .sum();
    Some(result)
}

fn parse_input(input: &str) -> Vec<(&str, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (code, groups) = line.split_once(' ').unwrap();
            let groups = groups
                .split(',')
                .filter_map(|c| c.parse::<usize>().ok())
                .collect_vec();
            (code, groups)
        })
        .collect_vec()
}

fn count_arrangements(code: &str, groups: &[usize], memo: &mut HashMap<String, u64>) -> u64 {
    let memo_key = code.to_owned() + &groups.iter().map(|el| el.to_string()).join(",");
    if let Some(memoized) = memo.get(&memo_key) {
        return *memoized;
    }
    if code.is_empty() {
        return if groups.is_empty() { 1 } else { 0 };
    }

    let arrangements = match code.chars().next() {
        Some('.') => count_arrangements(&code[1..], groups, memo),
        Some('?') => {
            count_arrangements(&code[1..], groups, memo)
                + count_arrangements(&("#".to_owned() + &code[1..]), groups, memo)
        }
        Some('#') => {
            if groups.is_empty() {
                return 0;
            }

            let expected = groups.first().unwrap();

            // if we expected more characters than we have
            if *expected > code.len() {
                return 0;
            }
            // if I can't make a chunk of selected length
            if code[0..*expected].chars().any(|c| c == '.') {
                return 0;
            }
            // if the code has exactly the expected length
            if code.len() == *expected {
                // validate if this is the last group
                return if groups.len() == 1 { 1 } else { 0 };
            }
            // if the char right after the chunk is # we will not have the expected length
            if code.chars().nth(*expected) == Some('#') {
                return 0;
            }
            // skip expected + 1 and remove one group to continue
            return count_arrangements(&code[*expected + 1..], &groups[1..], memo);
        }
        _ => unreachable!(),
    };
    memo.insert(memo_key, arrangements);
    arrangements
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
