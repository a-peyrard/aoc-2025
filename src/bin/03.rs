advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(to_bank)
            .map(compute_joltage)
            .sum()
    )
}

fn to_bank(line: &str) -> Vec<u32> {
    line.chars().filter_map(|c| c.to_digit(10)).collect()
}

fn compute_joltage(bank: Vec<u32>) -> u64 {
    let mut first = bank[0];
    let mut second = 0;
    for i in 1..bank.len() - 1 {
        let cur = bank[i];
        if cur > first {
            first = cur;
            second = 0;
        } else if cur > second {
            second = cur;
        }
    }

    if bank[bank.len() - 1] > second {
        second = bank[bank.len() - 1]
    }

    ((first * 10) + second) as u64
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
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_compute_joltage_should_work_with_same_digit() {
        // GIVEN
        let bank = to_bank("111111111");

        // WHEN
        let joltage = compute_joltage(bank);

        // THEN
        assert_eq!(joltage, 11);
    }

    #[test]
    fn test_compute_joltage_should_work_with_best_at_beginning() {
        // GIVEN
        let bank = to_bank("987654321111111");

        // WHEN
        let joltage = compute_joltage(bank);

        // THEN
        assert_eq!(joltage, 98);
    }

    #[test]
    fn test_compute_joltage_should_work_with_best_at_end() {
        // GIVEN
        let bank = to_bank("117654321111198");

        // WHEN
        let joltage = compute_joltage(bank);

        // THEN
        assert_eq!(joltage, 98);
    }

    #[test]
    fn test_compute_joltage_should_work_with_best_at_end_2() {
        // GIVEN
        let bank = to_bank("117654321111189");

        // WHEN
        let joltage = compute_joltage(bank);

        // THEN
        assert_eq!(joltage, 89);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
