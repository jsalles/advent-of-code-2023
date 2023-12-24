use std::collections::{HashMap, VecDeque};

use advent_of_code::helpers::math::lowest_common_multiple;
use itertools::Itertools;

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u64> {
    let mut nodes = parse_input(input);

    let mut pulse_count = (0, 0);
    (0..1_000).for_each(|i| {
        let pulses = press_button(&mut nodes, i);
        pulse_count.0 += pulses.0;
        pulse_count.1 += pulses.1;
    });

    Some(pulse_count.0 * pulse_count.1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut nodes = parse_input(input);

    // Nice idea from reddit! The parent and grandparents of the 'rx' node are all
    // conjunction modules. I just need to find the cycle when all grandparents received
    // Low pulses at the same time
    let parent = nodes.values().find(|n| n.destinations.contains(&"rx"))?;

    let grandparents = nodes
        .values()
        .filter(|n| n.destinations.contains(&parent.name))
        .map(|n| n.name)
        .collect_vec();

    for i in 0..10_000 {
        press_button(&mut nodes, i);
    }

    let grandparent_cycles = nodes
        .iter()
        .filter(|(name, _)| grandparents.contains(*name))
        .flat_map(|(_, node)| {
            node.cycles
                .iter()
                .filter(|(_, val)| val == &Pulse::Low)
                .tuple_windows()
                .map(|((first, _), (second, _))| second - first)
                .collect_vec()
        })
        .collect_vec();

    Some(lowest_common_multiple(&grandparent_cycles))
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module<'a> {
    Broadcast,
    FlipFlop(Pulse),
    Conjunction(HashMap<&'a str, Pulse>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node<'a> {
    name: &'a str,
    destinations: Vec<&'a str>,
    module: Module<'a>,
    cycles: Vec<(usize, Pulse)>,
}

impl<'a> Node<'a> {
    fn process_pulse(&mut self, source: &'a str, pulse: Pulse, cycle: usize) -> Option<Pulse> {
        self.cycles.push((cycle, pulse));
        match self.module {
            Module::Broadcast => Some(pulse),
            Module::FlipFlop(state) => {
                if pulse == Pulse::High {
                    None
                } else {
                    let new_state = if state == Pulse::High {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };
                    self.module = Module::FlipFlop(new_state);
                    Some(new_state)
                }
            }
            Module::Conjunction(ref mut map) => {
                map.insert(source, pulse);
                if map.values().contains(&Pulse::Low) {
                    Some(Pulse::High)
                } else {
                    Some(Pulse::Low)
                }
            }
        }
    }
}

fn press_button(nodes: &mut HashMap<&str, Node>, cycle: usize) -> (u64, u64) {
    let mut low_pulses_count: u64 = 0;
    let mut high_pulses_count: u64 = 0;

    let mut pulses = VecDeque::new();
    pulses.push_back(("button", "broadcaster", Pulse::Low));

    while !pulses.is_empty() {
        let pulse_count = pulses.len();
        (0..pulse_count).for_each(|_| {
            let (from, to, pulse) = pulses.pop_front().unwrap();
            match pulse {
                Pulse::Low => {
                    low_pulses_count += 1;
                }
                Pulse::High => {
                    high_pulses_count += 1;
                }
            }
            if let Some(node) = nodes.get_mut(to) {
                if let Some(new_pulse) = node.process_pulse(from, pulse, cycle) {
                    node.destinations.iter().for_each(|dest| {
                        // println!("{:?} -{:?}-> {:?}", to, new_pulse, dest);
                        pulses.push_back((to, dest, new_pulse))
                    });
                }
            }
        })
    }

    (low_pulses_count, high_pulses_count)
}

fn parse_input(input: &str) -> HashMap<&str, Node> {
    let mut nodes = input.lines().fold(HashMap::new(), |mut acc, line| {
        let (source, destinations) = line.split_once(" -> ").unwrap();
        let destinations = destinations.split(", ").collect_vec();

        if source == "broadcaster" {
            acc.insert(
                source,
                Node {
                    name: source,
                    destinations,
                    module: Module::Broadcast,
                    cycles: Vec::new(),
                },
            );
        } else if let Some(stripped) = source.strip_prefix('%') {
            acc.insert(
                stripped,
                Node {
                    name: stripped,
                    destinations,
                    module: Module::FlipFlop(Pulse::Low),
                    cycles: Vec::new(),
                },
            );
        } else if let Some(stripped) = source.strip_prefix('&') {
            acc.insert(
                stripped,
                Node {
                    name: stripped,
                    destinations,
                    module: Module::Conjunction(HashMap::new()),
                    cycles: Vec::new(),
                },
            );
        }

        acc
    });

    // now that we know every node, update the conjunction maps
    let cloned_nodes = nodes.clone();
    nodes.iter_mut().for_each(|(_, node)| {
        if let Module::Conjunction(ref mut state) = node.module {
            cloned_nodes
                .values()
                .filter(|n| n.destinations.contains(&node.name))
                .for_each(|n| {
                    state.insert(n.name, Pulse::Low);
                });
        }
    });

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
