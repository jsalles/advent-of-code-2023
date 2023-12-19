use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(18);

const MARGIN: usize = 200;

pub fn part_one(input: &str) -> Option<i64> {
    let instructions = input
        .lines()
        .map(|line| {
            let (direction, length, _) = line.split_whitespace().collect_tuple().unwrap();
            (direction, length.parse::<i64>().unwrap())
        })
        .collect_vec();
    if cfg!(flood_fill) {
        solve_with_flood_fill(instructions)
    } else {
        solve_with_picks_theorem(instructions)
    }
}

pub fn part_two(input: &str) -> Option<i64> {
    let instructions_iter = input
        .lines()
        .map(|line| {
            let (_, _, color) = line.split_whitespace().collect_tuple().unwrap();
            // (direction, length.parse::<isize>().unwrap())
            let direction = match color.chars().nth(7).unwrap() {
                '0' => "R",
                '1' => "D",
                '2' => "L",
                _ => "U",
            };
            let dist = i64::from_str_radix(&color.chars().skip(2).take(5).collect::<String>(), 16)
                .unwrap();
            (direction, dist)
        })
        .collect_vec();
    solve_with_picks_theorem(instructions_iter)
}

// much better with  Shoelace and Pick's theorem
// https://en.wikipedia.org/wiki/Shoelace_formula
// https://en.wikipedia.org/wiki/Pick%27s_theorem
fn solve_with_picks_theorem(instructions: Vec<(&str, i64)>) -> Option<i64> {
    let mut row = 0;
    let mut col = 0;
    let mut area = 0;
    let mut perimeter = 0;
    instructions.iter().for_each(|(direction, dist)| {
        let old_row = row;
        let old_col = col;
        match *direction {
            "R" => {
                col += *dist;
            }
            "D" => {
                row += *dist;
            }
            "L" => {
                col -= *dist;
            }
            "U" => {
                row -= *dist;
            }
            _ => unreachable!(),
        };
        area += old_col * row - col * old_row;
        perimeter += *dist;
    });

    Some((area / 2).abs() + perimeter / 2 + 1)
}

fn solve_with_flood_fill(instructions: Vec<(&str, i64)>) -> Option<i64> {
    let (height, width) = calculate_dimensions(&instructions);
    // give it a margin so we can flood the outside
    let mut map = vec![vec!['.'; width as usize + 2 * MARGIN]; height as usize + 2 * MARGIN];

    draw_boundaries(&instructions, &mut map);
    let result = flood_fill(&mut map);

    Some(result)
}

fn flood_fill(map: &mut Vec<Vec<char>>) -> i64 {
    let mut visited = HashSet::new();
    let mut queue: VecDeque<(i64, i64)> = VecDeque::new();
    queue.push_back((0, 0));

    while !queue.is_empty() {
        let (row, col) = queue.pop_front().unwrap();

        if row < 0 || row as usize >= map.len() || col < 0 || col as usize >= map[0].len() {
            continue;
        }

        if map[row as usize][col as usize] != '.' {
            continue;
        }

        if visited.contains(&(row, col)) {
            continue;
        }

        visited.insert((row, col));
        map[row as usize][col as usize] = '@';
        queue.push_back((row - 1, col));
        queue.push_back((row + 1, col));
        queue.push_back((row, col - 1));
        queue.push_back((row, col + 1));
    }

    (map.len() * map[0].len()) as i64 - visited.len() as i64
}

fn navigate((cur_row, cur_col): (i64, i64), (instr, shift): (&str, i64)) -> (i64, i64) {
    match instr {
        "L" => (cur_row, cur_col - shift),
        "R" => (cur_row, cur_col + shift),
        "U" => (cur_row - shift, cur_col),
        "D" => (cur_row + shift, cur_col),
        _ => unreachable!(),
    }
}

fn calculate_dimensions(instructions: &[(&str, i64)]) -> (i64, i64) {
    let mut dims = (0, 0);
    instructions.iter().fold((0, 0), |acc, (instr, shift)| {
        let next = navigate(acc, (instr, *shift));
        dims = (dims.0.max(next.0.abs()), dims.1.max(next.1.abs()));

        next
    });
    dims
}

fn draw_boundaries(instructions: &[(&str, i64)], map: &mut [Vec<char>]) {
    let mut row = MARGIN;
    let mut col = MARGIN;
    instructions.iter().for_each(|(instr, shift)| {
        (0..*shift).for_each(|_| {
            match *instr {
                "L" => {
                    col -= 1;
                }
                "R" => {
                    col += 1;
                }
                "U" => {
                    row -= 1;
                }
                "D" => {
                    row += 1;
                }
                _ => unreachable!(),
            };
            map[row][col] = '#';
        });
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
