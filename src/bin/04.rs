use advent_of_code::util::grid::Grid;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::parse_input(input);

    Some(
        grid.find(b'@')
            .map(|(x, y)| grid.adjacent_element(x, y, b'@').count())
            .filter(|number_of_adjacent| *number_of_adjacent < 4)
            .count() as u64,
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two_example_1() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
