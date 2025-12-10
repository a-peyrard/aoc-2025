advent_of_code::solution!(9);

type Coords = (i64, i64);

pub fn part_one(input: &str) -> Option<u64> {
    let coords: Vec<Coords> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_once(',')
                .map(|(left, right)| (left.parse::<i64>().unwrap(), right.parse::<i64>().unwrap()))
                .unwrap()
        })
        .collect();

    let mut area = 0;
    for i in 0..coords.len() {
        for j in i+1..coords.len() {
            area = area.max(compute_area(coords[i], coords[j]));
        }
    }

    Some(area)
}

fn compute_area(c1: Coords, c2: Coords) -> u64 {
    (((c1.0 - c2.0).abs() + 1) * ((c1.1 - c2.1).abs() + 1)) as u64
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
