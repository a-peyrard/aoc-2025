use itertools::enumerate;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let (numbers, operators) = parse_input(input);

    Some(
        numbers
            .into_iter()
            .zip(operators.into_iter())
            .map(|(nums, op)| match op {
                '+' => nums.iter().sum::<u64>(),
                '*' => nums.iter().product::<u64>(),
                _ => panic!("unexpected operator"),
            })
            .sum(),
    )
}

fn parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let mut numbers = Vec::<Vec<u64>>::new();
    let mut number_of_numbers_line = 0;
    for (index, line) in enumerate(input.lines()) {
        let first_char = line.chars().next().unwrap();
        if first_char == '+' || first_char == '*' {
            number_of_numbers_line = index;
            break;
        }
        let current_numbers: Vec<u64> = line
            .split_whitespace()
            .map(|token| token.parse::<u64>().unwrap())
            .collect();

        if numbers.is_empty() {
            for num in current_numbers {
                numbers.push(vec![num]);
            }
        } else {
            for (index, num) in enumerate(current_numbers) {
                numbers[index].push(num);
            }
        }
    }

    (
        numbers,
        input
            .lines()
            .skip(number_of_numbers_line)
            .next()
            .unwrap()
            .chars()
            .filter(|&c| c == '+' || c == '*')
            .collect(),
    )
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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two_example_1() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
