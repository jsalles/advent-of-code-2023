advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<f64> {
    Some(
        parse(input, false)
            .iter()
            .map(count_winning_conditions)
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<f64> {
    Some(
        parse(input, true)
            .iter()
            .map(count_winning_conditions)
            .product(),
    )
}

fn parse(input: &str, squash_numbers: bool) -> Vec<(i64, i64)> {
    let results: Vec<Vec<i64>> = input
        .lines()
        .map(|l| {
            let line = l.split_once(": ").unwrap().1;
            let numbers = if squash_numbers {
                [line.replace(' ', "")].to_vec()
            } else {
                line.split_whitespace()
                    .map(String::from)
                    .collect::<Vec<String>>()
            };

            numbers
                .iter()
                .filter_map(|n| n.parse::<i64>().ok())
                .collect::<Vec<i64>>()
        })
        .collect();

    results[0]
        .iter()
        .zip(results[1].iter())
        .map(|(t, d)| (*t, *d))
        .collect::<Vec<(i64, i64)>>()
}

fn count_winning_conditions((t, d): &(i64, i64)) -> f64 {
    // I want the smallest number X that satisfies
    // X * (T - X) > D -> -x2 + xt - d > 0
    let delta = (t.pow(2) - 4 * d) as f64;
    let x1 = ((-1 * t) as f64 + f64::sqrt(delta)) / -2f64;
    let x2 = ((-1 * t) as f64 - f64::sqrt(delta)) / -2f64;
    (x2 - 1.0).ceil() - (x1 + 1.0).floor() + 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288.0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503.0));
    }
}
