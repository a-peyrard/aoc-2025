advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let amounts = extract_amounts(input);

    let mut current = 50;
    let mut zeros = 0;
    for amount in amounts {
        current += amount;
        if current < 0 {
            current += 100;
        }

        current %= 100;
        if current == 0 {
            zeros += 1;
        }
    }

    Some(zeros)
}

pub fn part_two(input: &str) -> Option<u64> {
    let amounts = extract_amounts(input);

    let mut current = 50;
    let mut zeros = 0;
    for mut amount in amounts {
        zeros += (amount / 100).abs();
        amount %= 100;

        let prev = current;
        current += amount;
        if (prev != 0 && current <= 0) || current >= 100 {
            zeros += 1;
        }
        if current < 0 {
            current += 100;
        }

        current %= 100;
    }

    Some(zeros.try_into().unwrap())
}

fn extract_amounts(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            let direction = line.chars().next().unwrap();
            let amount: i32 = line[1..].parse().unwrap();

            if direction == 'L' { -amount } else { amount }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_1() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_one_custom_input_1() {
        let result = part_one(
            r#"R50
L100"#,
        );
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_custom_input_2() {
        let result = part_one(
            r#"L350
R100"#,
        );
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_solution() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1141));
    }

    #[test]
    fn test_part_two_example_1() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two_solution() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6634));
    }
}
