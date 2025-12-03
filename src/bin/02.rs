advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    part_gen(input, is_invalid)
}

fn is_invalid(id: u64) -> bool {
    let num_digit = count_digits(id);
    if num_digit % 2 != 0 {
        return false;
    }

    let split_point = 10_u64.pow(num_digit / 2);

    let left = id / split_point;
    let right = id % split_point;

    left == right
}

fn is_invalid_2(id: u64) -> bool {
    let num_digit = count_digits(id);
    if num_digit % 2 != 0 {
        // multiple cases, either same digit, or group of 3, because input numbers are 10 digits length max
        if num_digit > 1 && has_same_pattern(id, 1) {
            return true;
        }
        if num_digit > 3 && num_digit % 3 == 0 && has_same_pattern(id, 3) {
            return true;
        }
    } else {
        // we should also check group of 2 if length is 6 or 10, because 2121212121 can not be split in two parts
        if (num_digit == 6 || num_digit == 10) && has_same_pattern(id, 2) {
            return true;
        }

        // otherwise fallback on same as part1
        let split_point = 10_u64.pow(num_digit / 2);

        let left = id / split_point;
        let right = id % split_point;

        if left == right {
            return true;
        }
    }

    false
}

fn has_same_pattern(num: u64, pattern_length: u32) -> bool {
    let split_point = 10_u64.pow(pattern_length);
    let initial = num % split_point;
    let mut n = num;

    while n > 0 {
        if n % split_point != initial {
            return false;
        }
        n /= split_point;
    }

    true
}

fn count_digits(n: u64) -> u32 {
    match n {
        0 => 1,
        _ => n.ilog10() + 1,
    }
}

fn part_gen(input: &str, is_invalid_predicate: fn(u64) -> bool) -> Option<u64> {
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
            .filter(|id| is_invalid_predicate(*id))
            .sum::<u64>();
    }

    Some(invalid_ids)
}

pub fn part_two(input: &str) -> Option<u64> {
    part_gen(input, is_invalid_2)
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
    fn test_part_two_example_1() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
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

    #[test]
    fn test_is_invalid_2_should_return_true_if_invalid() {
        // GIVEN
        let test_cases = vec![
            ("two digits", 55),
            ("three digits", 111),
            ("four digits", 6464),
            ("six digits", 123123),
            ("nine digits", 123123123),
            ("ten digits", 1188511885),
            ("ten digits with groups of 2", 2121212121),
            ("six digits alt", 446446),
        ];

        // WHEN & THEN
        for (name, id) in test_cases {
            assert_eq!(
                is_invalid_2(id),
                true,
                "{}: expected {} to be invalid",
                name,
                id
            );
        }
    }

    #[test]
    fn test_is_invalid_2_should_return_false_if_valid() {
        // GIVEN
        let test_cases = vec![
            ("one digit", 5),
            ("three digits", 646),
            ("six digits", 123124),
        ];

        // WHEN & THEN
        for (name, id) in test_cases {
            assert_eq!(
                is_invalid_2(id),
                false,
                "{}: expected {} to be valid",
                name,
                id
            );
        }
    }
}
