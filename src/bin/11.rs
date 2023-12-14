use advent_of_code::helpers::matrix::{read_matrix, transpose, Matrix};
use itertools::Itertools;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    static EXPANSION_RATE: usize = 2;
    let map = read_matrix(input);

    Some(calculate_distances(&map, EXPANSION_RATE))
}

pub fn part_two(input: &str) -> Option<usize> {
    static EXPANSION_RATE: usize = if cfg!(test) { 100 } else { 1_000_000 };
    let map = read_matrix(input);

    Some(calculate_distances(&map, EXPANSION_RATE))
}

fn find_rows_without_galaxies(map: &Matrix) -> Vec<usize> {
    (0..map.len())
        .filter(|row| map[*row].iter().all(|c| *c != '#'))
        .collect_vec()
}

fn find_columns_without_galaxies(map: &Matrix) -> Vec<usize> {
    let transposed = transpose(map);
    find_rows_without_galaxies(&transposed)
}

fn find_galaxies(map: &Matrix) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();
    (0..map.len()).for_each(|row| {
        (0..map[row].len()).for_each(|col| {
            if map[row][col] == '#' {
                galaxies.push((row, col));
            }
        });
    });
    galaxies
}

fn calculate_distances(map: &Matrix, expansion_rate: usize) -> usize {
    let rows_without_galaxies = find_rows_without_galaxies(map);
    let columns_without_galaxies = find_columns_without_galaxies(map);

    let galaxies = find_galaxies(map);
    galaxies
        .iter()
        .combinations(2)
        .map(|el| {
            let first = el[0];
            let second = el[1];
            let count_rows_without_galaxies = rows_without_galaxies
                .iter()
                .filter(|row| (first.0.min(second.0)..second.0.max(first.0)).contains(row))
                .count();
            let count_columns_without_galaxies = columns_without_galaxies
                .iter()
                .filter(|col| (first.1.min(second.1)..second.1.max(first.1)).contains(col))
                .count();
            let diff_row =
                first.0.abs_diff(second.0) + count_rows_without_galaxies * (expansion_rate - 1);
            let diff_col =
                first.1.abs_diff(second.1) + count_columns_without_galaxies * (expansion_rate - 1);

            diff_row + diff_col
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8410));
    }
}
