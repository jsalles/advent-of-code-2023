use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let problems = parse_input(input);

    let result = problems.iter().map(|problem| predict(problem, true)).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let problems = parse_input(input);
    let result = problems.iter().map(|problem| predict(problem, false)).sum();
    Some(result)
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|el| el.parse::<i32>().ok())
                .collect()
        })
        .collect()
}

fn predict(line: &[i32], forwards: bool) -> i32 {
    if line.iter().all(|el| *el == 0) {
        return 0;
    }

    let new_line = line
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();

    if forwards {
        line.last().unwrap() + predict(&new_line, forwards)
    } else {
        line.first().unwrap() - predict(&new_line, forwards)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
