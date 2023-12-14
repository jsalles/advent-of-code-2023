use advent_of_code::helpers::matrix::{read_matrices, transpose, Matrix};

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    let allowed_differences = 0;
    let patterns = read_matrices(input);

    let result = patterns
        .iter()
        .map(|pattern| {
            find_mirror_row(pattern, allowed_differences) * 100
                + find_mirror_row(&transpose(pattern), allowed_differences)
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let expected_differences = 1;
    let patterns = read_matrices(input);

    let result = patterns
        .iter()
        .map(|pattern| {
            find_mirror_row(pattern, expected_differences) * 100
                + find_mirror_row(&transpose(pattern), expected_differences)
        })
        .sum();

    Some(result)
}

fn find_mirror_row(pattern: &Matrix, expected_differences: usize) -> usize {
    let mut total_differences;
    for row in 1..pattern.len() {
        let first_half = &pattern[0..row];
        let second_half = &pattern[row..];
        total_differences = 0;
        for i in 0..first_half.len().min(second_half.len()) {
            let first_line = &first_half[first_half.len() - i - 1];
            let second_line = &second_half[i];
            if second_line != first_line {
                total_differences += count_differences(first_line, second_line);

                if total_differences > expected_differences {
                    break;
                }
            }
        }
        if total_differences == expected_differences {
            return row;
        }
    }

    0
}

fn count_differences(first: &[char], second: &[char]) -> usize {
    (0..first.len()).filter(|i| first[*i] != second[*i]).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
