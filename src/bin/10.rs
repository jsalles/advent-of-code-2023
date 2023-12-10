use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(10);

type Point = (usize, usize);

pub fn part_one(input: &str) -> Option<usize> {
    let mut mapping = parse_input(input);
    let starting_position = get_start_position(&mapping);

    let mut visited = HashSet::new();
    navigate_pipes(&mut mapping, starting_position as Point, &mut visited);

    Some(visited.len() / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut mapping = parse_input(input);
    let starting_position = get_start_position(&mapping);

    let mut visited = HashSet::new();
    navigate_pipes(&mut mapping, starting_position as Point, &mut visited);

    let mut total_count = 0;
    (0..mapping.len()).for_each(|row| {
        let mut inside = false;
        (0..mapping[row].len()).for_each(|col| {
            let current_cel = mapping[row][col];
            if current_cel == '!' {
                inside = !inside;
            } else if !visited.contains(&(row, col)) && inside {
                total_count += 1;
            }
        });
    });
    // print_mapping(&mapping);

    Some(total_count)
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn get_start_position(mapping: &[Vec<char>]) -> Point {
    mapping
        .iter()
        .enumerate()
        .filter_map(|(i, row)| {
            row.iter()
                .find_position(|c| **c == 'S')
                .map(|(j, _)| (i, j))
        })
        .last()
        .unwrap()
}

fn get_next_cell(pipe: char, (row, col): Point) -> Vec<Point> {
    match pipe {
        '|' => vec![(row - 1, col), (row + 1, col)],
        '-' => vec![(row, col - 1), (row, col + 1)],
        'L' => vec![(row, col + 1), (row - 1, col)],
        'J' => vec![(row, col - 1), (row - 1, col)],
        '7' => vec![(row, col - 1), (row + 1, col)],
        'F' => vec![(row, col + 1), (row + 1, col)],
        'S' => vec![
            // (row - 1, col),
            // (row + 1, col),
            (row, col - 1),
            // (row, col + 1),
        ],
        _ => unreachable!(),
    }
}

fn _print_mapping(mapping: &[Vec<char>]) {
    for r in mapping.iter() {
        for c in r.iter() {
            print!("{}", c);
        }
        println!();
    }
}

fn navigate_pipes(
    mapping: &mut Vec<Vec<char>>,
    (row, col): Point,
    visited: &mut HashSet<Point>,
) -> bool {
    let current_cel = mapping[row][col];
    if current_cel == 'S' && !visited.is_empty() {
        // prevent shortcutting back to start
        return visited.len() > 2;
    }
    if current_cel == '.' || visited.contains(&(row, col)) {
        return false;
    }

    if ['J', 'L', '|', 'S'].contains(&current_cel) {
        mapping[row][col] = '!';
    } else {
        mapping[row][col] = '_';
    }

    visited.insert((row, col));
    for next in get_next_cell(current_cel, (row, col)).iter() {
        if navigate_pipes(mapping, *next, visited) {
            return true;
        }
    }

    false
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8));
    }
}
