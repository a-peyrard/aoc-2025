use std::collections::{HashMap, HashSet};

use advent_of_code::util::grid::Grid;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::parse_input(input);

    let start = grid.find(b'S').next().unwrap();
    let start_row = start.1;
    let mut beams = HashSet::from([start.0]);

    let mut splits = 0;
    for row in start_row + 1..grid.height-1 {
        let mut new_beams = HashSet::new();
        for beam in beams {
            match grid.get((beam, row)) {
                b'^' => {
                    new_beams.insert(beam - 1);
                    new_beams.insert(beam + 1);
                    splits += 1;
                },
                _ => {
                    new_beams.insert(beam);
                },
            }
        }
        beams = new_beams;
    }

    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = Grid::parse_input(input);

    let start = grid.find(b'S').next().unwrap();
    let start_row = start.1;
    let mut beams = HashMap::from([(start.0, 1)]);

    for row in start_row + 1..grid.height-1 {
        let mut new_beams = HashMap::<usize, u64>::new();
        for (beam, paths) in beams {
            match grid.get((beam, row)) {
                b'^' => {
                    *new_beams.entry(beam - 1).or_insert(0) += paths;
                    *new_beams.entry(beam + 1).or_insert(0) += paths;
                },
                _ => {
                    *new_beams.entry(beam).or_insert(0) += paths;
                },
            }
        }
        beams = new_beams;
    }

    Some(beams.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_1() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two_example_1() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
