use std::{collections::HashMap, ops::RangeInclusive};

use itertools::Itertools;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u64> {
    let (workflows, parts) = parse_input(input)?;
    let result = parts
        .iter()
        .map(|part| {
            let mut current_step = "in";

            while !["A", "R"].contains(&current_step) {
                current_step = apply_rules(&workflows[current_step].rules, part);
            }

            if current_step == "A" {
                part.x + part.m + part.a + part.s
            } else {
                0
            }
        })
        .sum();
    Some(result)
}

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

// .... it was at this moment he learned about Rust Limetime properties! :D
#[derive(Debug)]
struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
}

#[derive(Debug)]
struct Rule<'a> {
    part_name: char,
    comparator: char,
    value: u64,
    destination: &'a str,
}

fn parse_input(input: &str) -> Option<(HashMap<&str, Workflow>, Vec<Part>)> {
    let (workflows, parts) = input.split_once("\n\n")?;

    let parsed_workflows = workflows
        .lines()
        .try_fold(HashMap::new(), |mut acc, line| {
            let (name, steps) = line.split_once('{')?;
            let rules = steps
                .trim_end_matches('}')
                .split(',')
                .filter_map(|step| {
                    // possible passing rule
                    if let Some((condition, destination)) = step.split_once(':') {
                        let part_name = condition.chars().next()?;
                        let comparator = condition.chars().nth(1)?;
                        let value = condition[2..].parse().ok()?;

                        Some(Rule {
                            part_name,
                            comparator,
                            value,
                            destination,
                        })
                    } else {
                        // rule without condition, just destination
                        Some(Rule {
                            part_name: '_',
                            comparator: '_',
                            value: 0,
                            destination: step,
                        })
                    }
                })
                .collect_vec();
            acc.insert(name, Workflow { rules });
            Some(acc)
        })?;

    let parsed_parts = parts
        .lines()
        .map(|line| {
            let values = line
                .trim_start_matches('{')
                .trim_end_matches('}')
                .split(',')
                .filter_map(|part| {
                    part.split_once('=')
                        .and_then(|(_, value)| value.parse().ok())
                })
                .collect_vec();

            Part {
                x: values[0],
                m: values[1],
                a: values[2],
                s: values[3],
            }
        })
        .collect_vec();

    Some((parsed_workflows, parsed_parts))
}

fn rule_passes(rule: &Rule, value: u64) -> bool {
    match rule.comparator {
        '<' => value < rule.value,
        '>' => value > rule.value,
        _ => unreachable!(),
    }
}

fn apply_rules<'a>(rules: &[Rule<'a>], part: &Part) -> &'a str {
    let passing_rule = rules
        .iter()
        .find(|rule| match rule.part_name {
            'x' => rule_passes(rule, part.x),
            'm' => rule_passes(rule, part.m),
            'a' => rule_passes(rule, part.a),
            's' => rule_passes(rule, part.s),
            '_' => true,
            _ => unreachable!(),
        })
        .unwrap();
    passing_rule.destination
}

pub fn part_two(input: &str) -> Option<u64> {
    parse_input(input).map(|(workflows, _)| {
        resolve_state(&workflows, State::default())
            .into_iter()
            .flat_map(|state| {
                if state.path.last().unwrap() == &"A" {
                    Some(count_ranges(&state.range))
                } else {
                    None
                }
            })
            .sum()
    })
}

#[derive(Clone, Default)]
struct PartRange {
    x: Vec<RangeInclusive<u64>>,
    m: Vec<RangeInclusive<u64>>,
    a: Vec<RangeInclusive<u64>>,
    s: Vec<RangeInclusive<u64>>,
}

#[derive(Clone)]
struct State<'a> {
    range: PartRange,
    path: Vec<&'a str>,
}

impl<'a> Default for State<'a> {
    fn default() -> Self {
        Self {
            range: Default::default(),
            path: vec!["in"],
        }
    }
}

fn resolve_range(rule: &Rule, should_pass: bool) -> (char, RangeInclusive<u64>) {
    let floor = 1;
    let ceil = 4000;

    let range = match rule.comparator {
        '>' => {
            if should_pass {
                (rule.value + 1)..=ceil
            } else {
                floor..=rule.value
            }
        }
        '<' => {
            if should_pass {
                floor..=(rule.value - 1)
            } else {
                rule.value..=ceil
            }
        }
        _ => unreachable!(),
    };

    (rule.part_name, range)
}

fn update_ranges(state: &mut State, range: (char, RangeInclusive<u64>)) {
    match range.0 {
        'x' => state.range.x.push(range.1),
        'm' => state.range.m.push(range.1),
        'a' => state.range.a.push(range.1),
        's' => state.range.s.push(range.1),
        _ => unreachable!(),
    }
}

fn resolve_state<'a>(workflows: &'a HashMap<&str, Workflow>, state: State<'a>) -> Vec<State<'a>> {
    let mut next_states = vec![];

    let current = state.path.last().expect("empty paths are not allowed.");

    // we are at the end.
    if *current == "A" || *current == "R" {
        return vec![state];
    }

    let workflow = workflows.get(current).expect("undefined workflow");

    let mut current_state = state.clone();
    let mut reject_range;

    for rule in &workflow.rules {
        if rule.part_name == '_' {
            // reached the end so we haven't passed any rule
            current_state.path.push(rule.destination);
            next_states.extend(resolve_state(workflows, current_state.clone()));
        } else {
            let mut pass_state = current_state.clone();
            update_ranges(&mut pass_state, resolve_range(rule, true));

            pass_state.path.push(rule.destination);
            next_states.extend(resolve_state(workflows, pass_state));

            // also update the failing case
            reject_range = resolve_range(rule, false);
            update_ranges(&mut current_state, reject_range);
        }
    }

    next_states
}

fn collapse_range(range: &[RangeInclusive<u64>]) -> u64 {
    let range = range.iter().fold((1, 4000), |mut acc, curr| {
        if curr.start() > &acc.0 {
            acc.0 = *curr.start()
        }
        if curr.end() < &acc.1 {
            acc.1 = *curr.end()
        }
        acc
    });

    1 + range.1 - range.0
}

fn count_ranges(ranges: &PartRange) -> u64 {
    collapse_range(&ranges.x)
        * collapse_range(&ranges.m)
        * collapse_range(&ranges.a)
        * collapse_range(&ranges.s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
