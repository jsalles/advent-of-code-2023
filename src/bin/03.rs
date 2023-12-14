use advent_of_code::helpers::matrix::{read_matrix, Matrix};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid_part_1: Matrix = read_matrix(input);
    let part1 = count_all_part_numbers(&mut grid_part_1);
    Some(part1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid_part_2: Matrix = read_matrix(input);
    let part2 = count_gear_ratios(&mut grid_part_2);
    Some(part2)
}

fn extract_number(row: &mut Vec<char>, col: usize) -> u32 {
    let mut left: i32 = col as i32;
    while left >= 0 && row[left as usize].is_ascii_digit() {
        left -= 1;
    }
    let mut right = col;
    while right < row.len() && row[right].is_ascii_digit() {
        right += 1;
    }

    let number = row[((left + 1) as usize)..right]
        .iter()
        .collect::<String>()
        .parse::<u32>()
        .unwrap();

    row[((left + 1) as usize)..right]
        .iter_mut()
        .for_each(|c| *c = '.');

    number
}

fn check_adjascent_and_diagonals(row: &mut Vec<char>, col: usize) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();
    if row[col].is_ascii_digit() {
        numbers.push(extract_number(row, col));
    }
    if col > 0 && row[col - 1].is_ascii_digit() {
        numbers.push(extract_number(row, col - 1));
    }
    if col < row.len() - 1 && row[col + 1].is_ascii_digit() {
        numbers.push(extract_number(row, col + 1));
    }

    numbers
}

fn count_all_part_numbers(input: &mut Matrix) -> u32 {
    let mut result = 0;
    for row in 0..input.len() {
        let mut row_numbers: Vec<u32> = Vec::new();
        for col in 0..input[row].len() {
            if !input[row][col].is_ascii_digit() && input[row][col] != '.' {
                if row > 0 {
                    row_numbers
                        .append(&mut check_adjascent_and_diagonals(&mut input[row - 1], col));
                }
                if row < input[row].len() - 1 {
                    row_numbers
                        .append(&mut check_adjascent_and_diagonals(&mut input[row + 1], col));
                }

                row_numbers.append(&mut check_adjascent_and_diagonals(&mut input[row], col));
            }
        }
        result += row_numbers.iter().sum::<u32>();
    }

    result
}

fn count_adjacent(input: &Matrix, row: usize, col: usize) -> u32 {
    let mut result = 0;

    if row > 0 && input[row - 1][col].is_ascii_digit() {
        result += 1;
    } else {
        if col > 0 && input[row - 1][col - 1].is_ascii_digit() {
            result += 1
        }
        if col < input[row - 1].len() - 1 && input[row - 1][col + 1].is_ascii_digit() {
            result += 1
        }
    }
    if col > 0 && input[row][col - 1].is_ascii_digit() {
        result += 1;
    }
    if col < input[row].len() - 1 && input[row][col + 1].is_ascii_digit() {
        result += 1;
    }
    if row < input.len() - 1 && input[row + 1][col].is_ascii_digit() {
        result += 1;
    } else {
        if col > 0 && input[row + 1][col - 1].is_ascii_digit() {
            result += 1
        }
        if col < input[row].len() - 1 && input[row + 1][col + 1].is_ascii_digit() {
            result += 1
        }
    }

    result
}

fn count_gear_ratios(input: &mut Matrix) -> u32 {
    let mut result = 0;
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            if input[row][col] == '*' && count_adjacent(input, row, col) == 2 {
                let mut gear_parts: Vec<u32> = Vec::new();
                gear_parts.append(&mut check_adjascent_and_diagonals(&mut input[row - 1], col));
                gear_parts.append(&mut check_adjascent_and_diagonals(&mut input[row], col));
                gear_parts.append(&mut check_adjascent_and_diagonals(&mut input[row + 1], col));
                if gear_parts.len() != 2 {
                    panic!("Gear Parts should be 2");
                }
                result += gear_parts.iter().product::<u32>();
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
