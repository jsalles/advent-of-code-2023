use rayon::prelude::*;
use std::collections::HashMap;

use regex::{Captures, Regex};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(
        r"seeds: (?P<seeds>[\d ]+)\s+seed-to-soil map: (?P<seed_to_soil>[\d ]+)\s+soil-to-fertilizer map: (?P<soil_to_fertilizer>[\d ]+)\s+fertilizer-to-water map: (?P<fertilizer_to_water>[\d ]+)\s+water-to-light map: (?P<water_to_light>[\d ]+)\s+light-to-temperature map: (?P<light_to_temperature>[\d ]+)\s+temperature-to-humidity map: (?P<temperature_to_humidity>[\d ]+)\s+humidity-to-location map: (?P<humidity_to_location>[\d ]+)$"
    )
    .unwrap();
    // let input: &str = &INPUT.lines().collect::<Vec<&str>>().join(" ");
    let input = input.lines().collect::<Vec<&str>>().join(" ");

    let caps = re.captures(&input).unwrap();
    let seeds: Vec<u32> = caps["seeds"]
        .split_whitespace()
        .map(|e| e.parse().unwrap())
        .collect();
    let mappings = parse_all_mappings(&caps);
    seeds
        .par_iter()
        .map(|seed| from_seed_to_location(*seed, &mappings))
        .min()
}

// I loved this solution so I'll keep it here
pub fn part_two(input: &str) -> Option<usize> {
    let (seeds, maps) = parse(input)?;

    let ranges: Vec<(usize, usize)> = seeds
        .chunks(2)
        .map(|vals| {
            let x = vals[0] - 1;
            let y = x + vals[1];
            (x, y)
        })
        .collect();

    // Go backwards, from location to seed, and check if each number if a possible the solution.
    // The first one we find is the lowest possible solution. Luckily for our inputs the solution
    // will be in the millions, so it finishes quite fast
    for i in 0..usize::MAX {
        let start_value = maps.iter().rev().fold(i, |acc, map| {
            let range = map.iter().find(|range| {
                acc >= range.destination_start && acc < (range.destination_start + range.length)
            });

            match range {
                Some(range) => acc + range.source_start - range.destination_start,
                None => acc,
            }
        });

        if ranges
            .iter()
            .any(|r| start_value >= r.0 && start_value <= r.1)
        {
            return Some(i);
        }
    }

    None
}

struct Range {
    length: usize,
    source_start: usize,
    destination_start: usize,
}

fn parse_map(chunk: &str) -> Option<Vec<Range>> {
    let ranges: Vec<Range> = chunk
        .lines()
        .filter(|l| l.chars().next().is_some_and(|c| c.is_ascii_digit()))
        .map(|l| {
            let mut splits = l.split_whitespace().filter_map(|x| x.parse::<usize>().ok());
            Range {
                destination_start: splits.next().unwrap(),
                source_start: splits.next().unwrap(),
                length: splits.next().unwrap(),
            }
        })
        .collect();

    if ranges.is_empty() {
        None
    } else {
        Some(ranges)
    }
}

fn parse(input: &str) -> Option<(Vec<usize>, Vec<Vec<Range>>)> {
    let seeds = input
        .lines()
        .next()?
        .split_once(':')?
        .1
        .split_ascii_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    let maps = input.split("\n\n").skip(1).filter_map(parse_map).collect();

    Some((seeds, maps))
}

const CAPTURES_NAMES: [&str; 7] = [
    "seed_to_soil",
    "soil_to_fertilizer",
    "fertilizer_to_water",
    "water_to_light",
    "light_to_temperature",
    "temperature_to_humidity",
    "humidity_to_location",
];

#[derive(Debug)]
struct Mapping(u32, u32, u32);

fn captures_to_mappings(caps: &Captures, capture_name: &str) -> Vec<Mapping> {
    caps[capture_name]
        .split_whitespace()
        .map(|e| e.parse().unwrap())
        .collect::<Vec<u32>>()
        .chunks(3)
        .map(|chunk| Mapping(chunk[0], chunk[1], chunk[2]))
        .collect()
}

fn parse_all_mappings(caps: &Captures) -> HashMap<String, Vec<Mapping>> {
    CAPTURES_NAMES
        .iter()
        .fold(HashMap::new(), |mut mappings, capture_name| {
            let current_mapping = captures_to_mappings(caps, capture_name);
            mappings.insert(capture_name.to_string(), current_mapping);
            mappings
        })
}

fn from_seed_to_location(seed: u32, mappings: &HashMap<String, Vec<Mapping>>) -> u32 {
    CAPTURES_NAMES.iter().fold(seed, |current, capture_name| {
        match mappings
            .get(*capture_name)
            .unwrap()
            .iter()
            .find(|Mapping(_, source, range)| current >= *source && current < source + range)
        {
            Some(Mapping(destination, source, _)) => current - source + destination,
            _ => current,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
