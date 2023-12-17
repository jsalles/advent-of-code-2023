use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use itertools::Itertools;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<i32> {
    let map = parse_input(input);
    let result = modified_dijkstra(&map, 1, 3);

    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let map = parse_input(input);
    let result = modified_dijkstra(&map, 4, 10);

    Some(result)
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c as u8).collect_vec())
        .collect_vec()
}

fn modified_dijkstra(map: &[Vec<u8>], min_range: isize, max_range: isize) -> i32 {
    let mut distance_map = HashMap::new();
    let mut heap = BinaryHeap::new();
    let starting_cell = (0, 0);
    let starting_direction = (0, 0); // this is invalid but it won't matter for the first looo
    heap.push((Reverse(0), (starting_cell, starting_direction)));

    while let Some((Reverse(cur_distance), ((row, col), direction))) = heap.pop() {
        let (rows, cols) = (map.len(), map[0].len());

        if (row, col) == (rows - 1, cols - 1) {
            return cur_distance;
        }

        for (shift_row, shift_col) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            // stop now if going in the same direction or attempting a reverse
            if direction == (shift_row, shift_col) || direction == (-shift_row, -shift_col) {
                continue;
            }

            let mut next_distance = cur_distance;

            for range in 1..=max_range {
                let new_row = (row as isize + shift_row * range) as usize;
                let new_col = (col as isize + shift_col * range) as usize;

                // prevent out of bounds
                if new_row >= rows || new_col >= cols {
                    continue;
                }

                next_distance += (map[new_row][new_col] - b'0') as i32;

                if range < min_range {
                    continue;
                }

                let next_cell_with_direction = ((new_row, new_col), (shift_row, shift_col));
                let known_dist = distance_map
                    .get(&next_cell_with_direction)
                    .copied()
                    .unwrap_or(i32::MAX);

                if next_distance < known_dist {
                    distance_map.insert(next_cell_with_direction, next_distance);
                    heap.push((Reverse(next_distance), next_cell_with_direction));
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
