use itertools::enumerate;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, end_of_ranges) = parse_ranges(input);

    Some(
        input
            .lines()
            .skip(end_of_ranges + 1)
            .map(|id_str| id_str.parse::<u64>().unwrap())
            .filter(|id| is_fresh(*id, &ranges))
            .count() as u64,
    )
}

fn is_fresh(id: u64, ranges: &[(u64, u64)]) -> bool {
    match ranges.binary_search_by_key(&id, |(start, _)| *start) {
        Ok(_) => true,
        Err(index) => index > 0 && id <= ranges[index - 1].1,
    }
}

fn parse_ranges(input: &str) -> (Vec<(u64, u64)>, usize) {
    let mut end_of_ranges = 0;
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for (index, line) in enumerate(input.lines()) {
        if line.is_empty() {
            end_of_ranges = index;
            break;
        }
        ranges.push(parse_range(line));
    }

    ranges.sort_by_key(|(start, _)| *start);

    let mut merged_ranges = Vec::<(u64, u64)>::new();
    let mut merged: (u64, u64) = ranges[0];
    for cur in ranges.iter().skip(1) {
        if merged.1 < cur.0 {
            merged_ranges.push(merged);
            merged = *cur;
        } else if merged.1 < cur.1 {
            merged.1 = cur.1;
        }
    }
    merged_ranges.push(merged);

    (merged_ranges, end_of_ranges)
}

fn parse_range(line: &str) -> (u64, u64) {
    let mut tokens = line.split('-');

    (
        tokens.next().unwrap().parse::<u64>().unwrap(),
        tokens.next().unwrap().parse::<u64>().unwrap(),
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_parse_ranges_should_sort_and_merge_ranges() {
        // GIVEN
        let input = r#"1-3
1-4
2-8
10-12
10-11
10-15
22-27
"#;

        // WHEN
        let (ranges, _) = parse_ranges(input);

        // THEN
        assert_eq!(ranges, vec![
            (1, 8),
            (10, 15),
            (22, 27)
        ]);
    }

    #[test]
    fn test_part_two_example_1() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
