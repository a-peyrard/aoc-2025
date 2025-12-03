advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges: Vec<(u64, u64)> = input
        .trim()
        .split(",")
        .map(|range| {
            let mut tokens = range.split("-");

            (
                tokens.next().unwrap().parse().unwrap(),
                tokens.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut invalid_ids = 0;
    for range in ranges {
        invalid_ids += (range.0..=range.1)
            .filter(|id| is_invalid(*id))
            .sum::<u64>();
    }

    Some(invalid_ids)
}

fn is_invalid(id: u64) -> bool {
    let num_digit = count_digits(id);
    if num_digit % 2 != 0 {
        return false;
    }

    let magnitude = 10_u64.pow(num_digit / 2);

    let left = id / magnitude;
    let right = id % magnitude;

    left == right
}

fn count_digits(n: u64) -> u32 {
    match n {
        0 => 1,
        _ => n.ilog10() + 1,
    }
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_count_digits_should_count_number_of_digits_from_987() {
        // GIVEN
        let n = 987;

        // WHEN
        let d = count_digits(n);

        // THEN
        assert_eq!(d, 3);
    }

    #[test]
    fn test_is_invalid_should_return_true_if_invalid() {
        // GIVEN
        let test_cases = vec![
            ("two digits", 55),
            ("four digits", 6464),
            ("six digits", 123123),
            ("ten digits", 1188511885),
            ("six digits alt", 446446),
        ];

        // WHEN & THEN
        for (name, id) in test_cases {
            assert_eq!(
                is_invalid(id),
                true,
                "{}: expected {} to be invalid",
                name,
                id
            );
        }
    }

    #[test]
    fn test_is_invalid_should_return_false_if_valid() {
        // GIVEN
        let test_cases = vec![
            ("one digit", 5),
            ("three digits", 646),
            ("six digits", 123124),
        ];

        // WHEN & THEN
        for (name, id) in test_cases {
            assert_eq!(
                is_invalid(id),
                false,
                "{}: expected {} to be valid",
                name,
                id
            );
        }
    }
}
