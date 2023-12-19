use std::collections::HashSet;

use itertools::Itertools;
use rayon::{
    iter::{ParallelBridge, ParallelIterator},
    *,
};

advent_of_code::solution!(16);

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
    pub value: char,
}

pub fn part_one(input: &str) -> Option<usize> {
    let layout = parse_input(input);

    let mut collisions = HashSet::new();
    let mut trail = HashSet::new();
    navigate(
        Cell {
            row: 0,
            col: 0,
            value: '.',
        },
        if cfg!(test) {
            Direction::East
        } else {
            Direction::South
        },
        &layout,
        &mut collisions,
        &mut trail,
    );

    // energize_cells(&layout, &trail);

    Some(trail.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let layout = parse_input(input);
    let length = if cfg!(test) { 10 } else { 110 };

    let mut starting_points = Vec::new();
    (0..length).for_each(|el| {
        // top row
        starting_points.push((0, el, Direction::South));

        // west column
        starting_points.push((el, 0, Direction::East));

        // east column
        starting_points.push((el, length - 1, Direction::West));

        // bottom row
        starting_points.push((length - 1, el, Direction::North));
    });

    let result = starting_points
        .iter()
        .par_bridge()
        .map(|el| {
            let mut collisions = HashSet::new();
            let mut trail = HashSet::new();
            navigate(
                Cell {
                    row: el.0,
                    col: el.1,
                    value: '.',
                },
                el.2,
                &layout,
                &mut collisions,
                &mut trail,
            );

            trail.len()
        })
        .max();

    // energize_cells(&layout, &trail);

    result
}

fn _energize_cells(map: &[Cell], trail: &HashSet<Cell>) {
    let length: usize = if cfg!(test) { 10 } else { 110 };
    (0..length).for_each(|row| {
        (0..length).for_each(|col| {
            let cell = map
                .iter()
                .find(|item| item.col == col && item.row == row)
                .unwrap_or_else(|| panic!("expected to find item in map {} {}", row, col));
            let value = if trail.contains(cell) {
                if cell.value == '.' {
                    '#'
                } else {
                    cell.value
                }
            } else {
                cell.value
            };
            print!("{}", value);
        });
        println!();
    });
}

fn navigate(
    current_cell: Cell,
    direction: Direction,
    map: &[Cell],
    collisions: &mut HashSet<String>,
    trail: &mut HashSet<Cell>,
) {
    let hash_key = format!("{}-{}-{:?}", current_cell.row, current_cell.col, direction);
    if collisions.contains(&hash_key) {
        return;
    }
    collisions.insert(hash_key);

    let next_collision = get_next_collision(current_cell, &direction, map);
    get_trail(current_cell, direction, next_collision, map)
        .iter()
        .for_each(|item| {
            trail.insert(*item);
        });
    let Some(collision) = next_collision else {
        return;
    };
    let next_directions = treat_collision(collision.value, &direction);
    next_directions
        .iter()
        .for_each(|next_direction| navigate(collision, *next_direction, map, collisions, trail));
}

fn parse_input(input: &str) -> Vec<Cell> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, value)| Cell { row, col, value })
                .collect_vec()
        })
        .collect_vec()
}

fn get_trail(from: Cell, direction: Direction, to: Option<Cell>, map: &[Cell]) -> Vec<Cell> {
    let to = match to {
        Some(val) => val,
        _ => match direction {
            Direction::North => Cell {
                row: 0,
                col: from.col,
                value: '.',
            },
            Direction::East => Cell {
                row: from.row,
                col: usize::MAX,
                value: '.',
            },
            Direction::South => Cell {
                row: usize::MAX,
                col: from.col,
                value: '.',
            },
            Direction::West => Cell {
                row: from.row,
                col: 0,
                value: '.',
            },
        },
    };
    map.iter()
        .filter(|item| {
            (from.row.min(to.row)..=from.row.max(to.row)).contains(&item.row)
                && (from.col.min(to.col)..=from.col.max(to.col)).contains(&item.col)
        })
        .copied()
        .collect_vec()
}

fn treat_collision(value: char, direction: &Direction) -> Vec<Direction> {
    match direction {
        Direction::North => match value {
            '\\' => vec![Direction::West],
            '/' => vec![Direction::East],
            '-' => vec![Direction::West, Direction::East],
            _ => unreachable!(),
        },
        Direction::East => match value {
            '\\' => vec![Direction::South],
            '/' => vec![Direction::North],
            '|' => vec![Direction::South, Direction::North],
            _ => unreachable!(),
        },
        Direction::South => match value {
            '\\' => vec![Direction::East],
            '/' => vec![Direction::West],
            '-' => vec![Direction::West, Direction::East],
            _ => unreachable!(),
        },
        Direction::West => match value {
            '\\' => vec![Direction::North],
            '/' => vec![Direction::South],
            '|' => vec![Direction::North, Direction::South],
            _ => unreachable!(),
        },
    }
}

fn get_next_collision(cell: Cell, direction: &Direction, map: &[Cell]) -> Option<Cell> {
    match direction {
        Direction::North => map
            .iter()
            .filter(|item| item.row < cell.row && item.col == cell.col)
            .sorted_by_key(|item| item.row)
            .rev()
            .find(|item| item.value != '.' && item.value != '|')
            .copied(),
        Direction::East => map
            .iter()
            .filter(|item| item.row == cell.row && item.col > cell.col)
            .sorted_by_key(|item| item.col)
            .find(|item| item.value != '.' && item.value != '-')
            .copied(),
        Direction::South => map
            .iter()
            .filter(|item| item.row > cell.row && item.col == cell.col)
            .sorted_by_key(|item| item.row)
            .find(|item| item.value != '.' && item.value != '|')
            .copied(),
        Direction::West => map
            .iter()
            .filter(|item| item.row == cell.row && item.col < cell.col)
            .sorted_by_key(|item| item.col)
            .rev()
            .find(|item| item.value != '.' && item.value != '-')
            .copied(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
