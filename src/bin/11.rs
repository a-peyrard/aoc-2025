use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let devices: HashMap<&str, Vec<&str>> = input.lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(device, raw_output)| (device, raw_output.trim().split_whitespace().collect::<Vec<_>>()))
        .collect();

    let mut queue: Vec<&str> = vec!["you"];
    let mut paths = 0;
    while let Some(device) = queue.pop() {
        if device == "out" {
            paths += 1;
            continue;
        }
        if let Some(values) = devices.get(device) {
            for value in values {
                queue.push(value);
            }
        }
    }

    Some(paths)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_1() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
