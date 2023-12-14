use std::collections::HashMap;

use advent_of_code::helpers::matrix::{read_matrix, reverse_rows, transpose, Matrix};
use itertools::Itertools;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let mapping = read_matrix(input);
    let mapping = slide_rocks_north(&mapping, &mut HashMap::new());

    let result: usize = mapping
        .iter()
        .rev()
        .enumerate()
        .map(|(i, r)| r.iter().filter(|c| **c == 'O').count() * (i + 1))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut memo = HashMap::new();
    let mut mapping = read_matrix(input);
    // starts repeating after 118 cycles! :D
    for _ in 0..118 {
        mapping = slide_rocks_north(&mapping, &mut memo);
        mapping = slide_rocks_west(&mapping, &mut memo);
        mapping = slide_rocks_south(&mapping, &mut memo);
        mapping = slide_rocks_east(&mapping, &mut memo);
    }

    let result: usize = mapping
        .iter()
        .rev()
        .enumerate()
        .map(|(i, r)| r.iter().filter(|c| **c == 'O').count() * (i + 1))
        .sum();

    Some(result)
}

fn matrix_to_memo_key(matrix: &Matrix, direction: &str) -> String {
    matrix_to_memoized(matrix) + direction
}

fn matrix_from_memoized(memoized: &str) -> Matrix {
    memoized
        .split('-')
        .map(|row| row.chars().collect_vec())
        .collect_vec()
}

fn matrix_to_memoized(matrix: &Matrix) -> String {
    matrix.iter().map(|row| row.iter().join("")).join("-")
}

fn slide_rocks_north(mapping: &Matrix, memo: &mut HashMap<String, String>) -> Matrix {
    let memo_key = matrix_to_memo_key(mapping, "north");
    if let Some(memoized) = memo.get(&memo_key) {
        return matrix_from_memoized(memoized);
    }
    let transposed = transpose(mapping);
    let response = transpose(&slide_rocks_west(&transposed, memo));
    memo.insert(memo_key, matrix_to_memoized(&response));
    response
}

fn slide_rocks_east(mapping: &Matrix, memo: &mut HashMap<String, String>) -> Matrix {
    let memo_key = matrix_to_memo_key(mapping, "east");
    if let Some(memoized) = memo.get(&memo_key) {
        return matrix_from_memoized(memoized);
    }
    let reversed = reverse_rows(mapping);
    let response = reverse_rows(&slide_rocks_west(&reversed, memo));
    memo.insert(memo_key, matrix_to_memoized(&response));
    response
}

fn slide_rocks_south(mapping: &Matrix, memo: &mut HashMap<String, String>) -> Matrix {
    let memo_key = matrix_to_memo_key(mapping, "south");
    if let Some(memoized) = memo.get(&memo_key) {
        return matrix_from_memoized(memoized);
    }
    let transposed_reversed = reverse_rows(&transpose(mapping));
    let response = transpose(&reverse_rows(&slide_rocks_west(&transposed_reversed, memo)));
    memo.insert(memo_key, matrix_to_memoized(&response));
    response
}

fn slide_rocks_west(mapping: &Matrix, memo: &mut HashMap<String, String>) -> Matrix {
    let memo_key = matrix_to_memo_key(mapping, "west");
    if let Some(memoized) = memo.get(&memo_key) {
        return matrix_from_memoized(memoized);
    }

    let mut mapping_after_slide = Vec::new();

    for r in mapping {
        let mut last_position = 0;
        let mut row = Vec::new();
        while last_position < r.len() {
            if let Some((rock_position, _)) =
                r[last_position..].iter().find_position(|c| **c == '#')
            {
                let count_rocks = r[last_position..last_position + rock_position]
                    .iter()
                    .filter(|c| **c == 'O')
                    .count();
                row.append(&mut vec!['O'; count_rocks]);
                row.append(&mut vec!['.'; rock_position - count_rocks]);
                row.push('#');

                last_position += rock_position + 1;
            } else {
                let count_rocks = r[last_position..].iter().filter(|c| **c == 'O').count();
                row.append(&mut vec!['O'; count_rocks]);
                row.append(&mut vec!['.'; r[last_position..].len() - count_rocks]);

                break;
            }
        }
        mapping_after_slide.push(row);
    }
    memo.insert(memo_key, matrix_to_memoized(&mapping_after_slide));

    mapping_after_slide
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
