use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let response = input
        .trim_end_matches('\n')
        .split(',')
        .map(|hash| {
            let mut total: u32 = 0;
            for c in hash.chars() {
                total = ((total + c as u32) * 17) % 256;
            }
            total
        })
        .sum::<u32>();

    Some(response)
}

struct BoxInfo {
    label: String,
    focal_length: u32,
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut box_map: HashMap<u32, Vec<BoxInfo>> = HashMap::new();

    input.trim_end_matches('\n').split(',').for_each(|hash| {
        let separator_pos = hash.find(|c| ['-', '='].contains(&c)).unwrap();
        let (label, rest) = hash.split_at(separator_pos);
        let (separator, focal_length_str) = rest.split_at(1);

        let box_number = label
            .chars()
            .fold(0, |acc, cur| ((acc + cur as u32) * 17) % 256);

        if separator == "=" {
            let focal_length = focal_length_str.parse::<u32>().unwrap();
            let box_info = BoxInfo {
                label: label.to_string(),
                focal_length,
            };

            let matching_box = box_map.entry(box_number).or_default();
            if let Some((matching_pos, _)) = matching_box
                .iter()
                .find_position(|item| item.label.starts_with(label))
            {
                matching_box[matching_pos] = box_info;
            } else {
                matching_box.push(box_info);
            }
        } else if let Some(matching_box) = box_map.get_mut(&box_number) {
            matching_box.retain(|item| !item.label.starts_with(label));
        }
    });

    let result = box_map
        .iter()
        .flat_map(|(box_number, box_infos)| {
            box_infos
                .iter()
                .enumerate()
                .map(|(idx, info)| (*box_number + 1) * (idx as u32 + 1) * info.focal_length)
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
