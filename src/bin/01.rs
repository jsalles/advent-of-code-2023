advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(read_digits).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().map(read_words).sum())
}

fn read_digits(input: &str) -> u32 {
    let (first, last) = input.chars().fold((None, None), |acc, c| {
        if let Some(digit) = c.to_digit(10) {
            (acc.0.or(Some(digit)), Some(digit))
        } else {
            acc
        }
    });

    match (first, last) {
        (Some(first), Some(last)) => first * 10 + last,
        _ => 0,
    }
}

fn read_words(input: &str) -> u32 {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut res: (u32, u32) = (0, 0);

    let mut left = input.len();
    let mut right = 0;
    for (i, number) in numbers.iter().enumerate() {
        if let Some(idx) = input.find(number) {
            if idx < left {
                left = idx;
                res.0 = (i + 1) as u32;
            }
        }
        if let Some(idx) = input.rfind(number) {
            if idx >= right {
                right = idx;
                res.1 = (i + 1) as u32;
            }
        }
    }

    if let Some(idx) = input.find(|c: char| c.is_ascii_digit()) {
        if idx < left {
            res.0 = input.chars().nth(idx).unwrap().to_digit(10).unwrap();
        }
    }
    if let Some(idx) = input.rfind(|c: char| c.is_ascii_digit()) {
        if idx >= right {
            res.1 = input.chars().nth(idx).unwrap().to_digit(10).unwrap();
        }
    }

    res.0 * 10 + res.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
