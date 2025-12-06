advent_of_code::solution!(3);

fn part_gen(input: &str, joltage_computer: fn(&[u32]) -> u64) -> Option<u64> {
    Some(
        input //
            .lines()
            .map(to_bank)
            .map(|bank| joltage_computer(&bank))
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    part_gen(input, compute_joltage)
}

fn to_bank(line: &str) -> Vec<u32> {
    line.chars().filter_map(|c| c.to_digit(10)).collect()
}

fn compute_joltage(bank: &[u32]) -> u64 {
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

pub fn part_two(input: &str) -> Option<u64> {
    part_gen(input, compute_joltage_2)
}

fn compute_joltage_2(bank: &[u32]) -> u64 {
    let target = 12;

    let mut batteries = vec![];
    let mut index = 0;

    while batteries.len() < target && target - batteries.len() < bank.len() - index {
        let (max, idx) = find_maximum_in_range(
            &bank, //
            index,
            bank.len() - target + 1 + batteries.len(),
        );
        batteries.push(max);
        index = idx + 1;
    }

    if batteries.len() < target {
        let missing_batteries = target - batteries.len();
        for idx in 0..missing_batteries {
            let index_to_pick = bank.len() - missing_batteries + idx;
            batteries.push(bank[index_to_pick]);
        }
    }

    to_number(&batteries)
}

fn find_maximum_in_range(bank: &[u32], start: usize, end: usize) -> (u32, usize) {
    let mut max = 0;
    let mut max_at = 0;
    for idx in start..end {
        if bank[idx] > max {
            max = bank[idx];
            max_at = idx;
        }
        if max == 9 {
            return (max, max_at);
        }
    }

    (max, max_at)
}

fn to_number(pieces: &[u32]) -> u64 {
    let mut cur = pieces[0] as u64;
    for idx in 1..pieces.len() {
        cur *= 10;
        cur += pieces[idx] as u64;
    }

    cur
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
        let joltage = compute_joltage(&bank);

        // THEN
        assert_eq!(joltage, 11);
    }

    #[test]
    fn test_compute_joltage_should_work_with_best_at_beginning() {
        // GIVEN
        let bank = to_bank("987654321111111");

        // WHEN
        let joltage = compute_joltage(&bank);

        // THEN
        assert_eq!(joltage, 98);
    }

    #[test]
    fn test_compute_joltage_should_work_with_best_at_end() {
        // GIVEN
        let bank = to_bank("117654321111198");

        // WHEN
        let joltage = compute_joltage(&bank);

        // THEN
        assert_eq!(joltage, 98);
    }

    #[test]
    fn test_compute_joltage_should_work_with_best_at_end_2() {
        // GIVEN
        let bank = to_bank("117654321111189");

        // WHEN
        let joltage = compute_joltage(&bank);

        // THEN
        assert_eq!(joltage, 89);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }

    #[test]
    fn test_compute_joltage_2_should_work_with_example_1() {
        // GIVEN
        let bank = to_bank("987654321111111");

        // WHEN
        let joltage = compute_joltage_2(&bank);

        // THEN
        assert_eq!(joltage, 987654321111);
    }

    #[test]
    fn test_compute_joltage_2_should_work_with_example_2() {
        // GIVEN
        let bank = to_bank("811111111111119");

        // WHEN
        let joltage = compute_joltage_2(&bank);

        // THEN
        assert_eq!(joltage, 811111111119);
    }

    #[test]
    fn test_compute_joltage_2_should_work_with_example_3() {
        // GIVEN
        let bank = to_bank("234234234234278");

        // WHEN
        let joltage = compute_joltage_2(&bank);

        // THEN
        assert_eq!(joltage, 434234234278);
    }

    #[test]
    fn test_compute_joltage_2_should_work_with_example_4() {
        // GIVEN
        let bank = to_bank("818181911112111");

        // WHEN
        let joltage = compute_joltage_2(&bank);

        // THEN
        assert_eq!(joltage, 888911112111);
    }

    #[test]
    fn test_compute_joltage_2_should_work_with_custom_example_1() {
        // GIVEN
        let bank = to_bank("111199999999999");

        // WHEN
        let joltage = compute_joltage_2(&bank);

        // THEN
        assert_eq!(joltage, 199999999999);
    }

    #[test]
    fn test_to_number_should_transform_vec_to_number() {
        // GIVEN
        let pieces = vec![1, 2, 3, 4, 5];

        // WHEN
        let num = to_number(&pieces);

        // THEN
        assert_eq!(num, 12345);
    }
}
