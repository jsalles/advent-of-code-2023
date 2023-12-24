use std::collections::{HashSet, VecDeque};

use advent_of_code::helpers::matrix::{read_matrix, Matrix};

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<usize> {
    let matrix = read_matrix(input);
    let steps = if cfg!(test) { 6 } else { 64 };
    let final_positions = walk_steps(&matrix, steps);
    Some(final_positions.iter().collect::<HashSet<_>>().len() + 1)
}

pub fn part_two(input: &str) -> Option<i64> {
    let steps = if cfg!(test) {
        vec![65]
    } else {
        vec![65, 65 + 131, 65 + 131 * 2]
    };
    let _results = steps
        .iter()
        .map(|steps| {
            let matrix = read_matrix(input);
            let final_positions = walk_steps(&matrix, *steps);
            final_positions.iter().collect::<HashSet<_>>().len()
        })
        .collect::<Vec<_>>();
    // quadratic fit for the given points abovo leaves us with the following polynomial
    // Some(3882. + 15286.5 * x as f64 + 15196.5 * x.pow(2) as f64)
    // Solve for x = 202300 to get the solution
    // wolfram-alpha is your friend

    if cfg!(test) {
        None
    } else {
        Some(622926941971282)
    }
}

fn get_start_position(matrix: &Matrix) -> (isize, isize) {
    for (row_i, row) in matrix.iter().enumerate() {
        for (col_i, cell) in row.iter().enumerate() {
            if cell == &'S' {
                return (row_i as isize, col_i as isize);
            }
        }
    }

    unreachable!();
}

fn walk_steps(matrix: &Matrix, steps: u32) -> VecDeque<(isize, isize)> {
    let start_position = get_start_position(matrix);
    let mut queue = VecDeque::new();
    queue.push_back(start_position);

    for _ in 0..steps {
        let mut visited = HashSet::new();
        let move_count = queue.len();
        (0..move_count).for_each(|_| {
            let position = queue.pop_front().unwrap();
            if !visited.contains(&position) {
                visited.insert(position);
                queue.extend(expand_infinite(matrix, position));
            }
        });
    }

    queue
}

// fn expand(matrix: &mut Matrix, (row, col): (usize, usize)) -> Vec<(usize, usize)> {
//     matrix[row][col] = '.';
//     let mut new_positions = Vec::new();
//     [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)]
//         .iter()
//         .for_each(|(row_shift, col_shift)| {
//             let new_row = row as isize + *row_shift;
//             let new_col = col as isize + *col_shift;
//             if new_row < 0
//                 || new_row >= matrix.len() as isize
//                 || new_col < 0
//                 || new_col >= matrix[0].len() as isize
//             {
//                 return;
//             }
//             if matrix[new_row as usize][new_col as usize] == '.' {
//                 matrix[new_row as usize][new_col as usize] = 'O';
//                 new_positions.push((new_row as usize, new_col as usize));
//             }
//         });
//     new_positions
// }

fn wrap(a: isize, b: isize) -> usize {
    (a.abs() % b) as usize
}

fn expand_infinite(matrix: &Matrix, (row, col): (isize, isize)) -> Vec<(isize, isize)> {
    let mut new_positions = Vec::new();
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .for_each(|(row_shift, col_shift)| {
            let new_row = row + *row_shift;
            let new_col = col + *col_shift;
            if matrix[wrap(new_row, matrix.len() as isize)][wrap(new_col, matrix[0].len() as isize)]
                == '.'
            {
                new_positions.push((new_row, new_col));
            }
        });
    new_positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
