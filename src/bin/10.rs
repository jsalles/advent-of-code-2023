use std::collections::HashSet;

use advent_of_code::helpers::matrix::{read_matrix, Matrix};
use itertools::Itertools;

advent_of_code::solution!(10);

type Point = (usize, usize);

pub fn part_one(input: &str) -> Option<usize> {
    let mut mapping = read_matrix(input);
    let starting_position = get_start_position(&mapping);

    let mut visited = HashSet::new();
    navigate_pipes(&mut mapping, starting_position as Point, &mut visited);

    Some(visited.len() / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut mapping = read_matrix(input);
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

    Some(total_count)
}

fn get_start_position(mapping: &Matrix) -> Point {
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

fn get_next_cell(pipe: char, (row, col): Point, mapping: &Matrix) -> Vec<Point> {
    match pipe {
        '|' => vec![(row - 1, col), (row + 1, col)],
        '-' => vec![(row, col - 1), (row, col + 1)],
        'L' => vec![(row, col + 1), (row - 1, col)],
        'J' => vec![(row, col - 1), (row - 1, col)],
        '7' => vec![(row, col - 1), (row + 1, col)],
        'F' => vec![(row, col + 1), (row + 1, col)],
        'S' => {
            let mut next_cells = Vec::new();
            if row > 0 && ['|', '7', 'F'].contains(&mapping[row - 1][col]) {
                next_cells.push((row - 1, col));
            }
            if row < mapping.len() - 1 && ['|', 'L', 'J'].contains(&mapping[row + 1][col]) {
                next_cells.push((row + 1, col));
            }
            if col > 0 && ['-', 'L', 'F'].contains(&mapping[row][col - 1]) {
                next_cells.push((row, col - 1));
            }
            if col < mapping[0].len() && ['-', 'J', '7'].contains(&mapping[row][col + 1]) {
                next_cells.push((row, col + 1));
            }
            next_cells
        }
        _ => unreachable!(),
    }
}

fn navigate_pipes(mapping: &mut Matrix, (row, col): Point, visited: &mut HashSet<Point>) -> bool {
    let current_cel = mapping[row][col];
    if current_cel == 'S' && !visited.is_empty() {
        // prevent shortcutting back to start
        return visited.len() > 2;
    }
    if current_cel == '.' || visited.contains(&(row, col)) {
        return false;
    }

    if ['J', 'L', '|'].contains(&current_cel) {
        mapping[row][col] = '!';
    } else {
        mapping[row][col] = '_';
    }

    visited.insert((row, col));
    for next in get_next_cell(current_cel, (row, col), mapping).iter() {
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
