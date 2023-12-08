use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(parse_game).map(check_game_valid).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().map(parse_game).map(get_power_of_cubes).sum())
}

#[derive(Debug)]
struct Game {
    id: u32,
    attempts: Vec<HashMap<String, u32>>,
}

fn parse_game(input: &str) -> Game {
    let re = Regex::new(r"^Game (?P<game_id>\d+): (?P<attempts>.*)$").unwrap();
    let caps = re.captures(input).unwrap();
    let attempts: Vec<HashMap<String, u32>> = caps["attempts"]
        .split(';')
        .map(|attempt| {
            attempt
                .split(',')
                .fold(HashMap::new(), |mut color_map, play| {
                    let res: Vec<&str> = play.trim().splitn(2, ' ').collect();
                    color_map.insert(res[1].to_string(), res[0].parse().unwrap());
                    color_map
                })
        })
        .collect();
    Game {
        id: caps["game_id"].parse::<u32>().unwrap(),
        attempts,
    }
}

fn attempt_is_possible(input: HashMap<String, u32>) -> bool {
    let max_colors: HashMap<&str, u32> = [("red", 12), ("green", 13), ("blue", 14)]
        .iter()
        .cloned()
        .collect();

    input
        .iter()
        .all(|(color, value)| *value <= *max_colors.get(color.as_str()).unwrap_or(&0))
}

fn check_game_valid(input: Game) -> u32 {
    match input.attempts.into_iter().all(attempt_is_possible) {
        true => input.id,
        false => 0,
    }
}

fn get_power_of_cubes(input: Game) -> u32 {
    let max_colors: HashMap<&str, u32> = input
        .attempts
        .iter()
        .flat_map(|attempt| attempt.iter())
        .fold(HashMap::new(), |mut acc, (color, value)| {
            let max_value = *acc.get(color.as_str()).unwrap_or(&0);
            acc.insert(color, max_value.max(*value));
            acc
        });

    max_colors.values().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
