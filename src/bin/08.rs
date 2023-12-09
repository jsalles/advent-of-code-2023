use advent_of_code::helpers::math::lowest_common_multiple;
use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let (instructions, maps) = parse_input(input)?;

    let mut current_step = "AAA";
    for counter in 0..usize::MAX {
        let current_instruction = instructions.chars().nth(counter % instructions.len());
        current_step = match current_instruction {
            Some('L') => &maps.get(current_step).unwrap().0,
            Some('R') => &maps.get(current_step).unwrap().1,
            _ => unreachable!(),
        };
        if current_step == "ZZZ" {
            return Some(counter + 1);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let (instructions, maps) = parse_input(input)?;

    let initial_steps = maps.keys().filter(|step| step.ends_with('A')).collect_vec();
    let mut totals: Vec<usize> = Vec::new();
    for step in initial_steps {
        let mut current_step = step;
        for counter in 0..usize::MAX {
            let current_instruction = instructions.chars().nth(counter % instructions.len());
            current_step = match current_instruction {
                Some('L') => &maps.get(current_step).unwrap().0,
                Some('R') => &maps.get(current_step).unwrap().1,
                _ => unreachable!(),
            };
            if current_step.ends_with('Z') {
                totals.push(counter + 1);
                break;
            }
        }
    }

    Some(lowest_common_multiple(&totals))
}

type Mappings = HashMap<String, (String, String)>;
fn parse_input(input: &str) -> Option<(&str, Mappings)> {
    let (instructions, maps) = input.split_once("\n\n")?;
    let parsed_maps: Mappings = maps.split('\n').fold(HashMap::new(), |mut acc, line| {
        match line.split_once(" = ") {
            Some((source, options)) => {
                let parsed_options = options
                    .split(", ")
                    .map(|opt| opt.chars().filter(|c| c.is_alphanumeric()).collect())
                    .collect_tuple()
                    .unwrap();

                acc.insert(source.to_string(), parsed_options);
                acc
            }
            _ => acc,
        }
    });

    Some((instructions, parsed_maps))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
