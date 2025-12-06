use std::io::BufRead;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Eq, Hash, PartialEq, Copy, Clone, Debug, Ord, PartialOrd)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn rotate_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::NorthEast => Direction::SouthEast,
            Direction::East => Direction::South,
            Direction::SouthEast => Direction::SouthWest,
            Direction::South => Direction::West,
            Direction::SouthWest => Direction::NorthWest,
            Direction::West => Direction::North,
            Direction::NorthWest => Direction::NorthEast,
        }
    }

    pub fn rotate_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::NorthEast => Direction::NorthWest,
            Direction::East => Direction::North,
            Direction::SouthEast => Direction::NorthEast,
            Direction::South => Direction::East,
            Direction::SouthWest => Direction::SouthEast,
            Direction::West => Direction::South,
            Direction::NorthWest => Direction::SouthWest,
        }
    }
}

pub struct ElementIterator<'a> {
    grid: &'a Grid,
    x: usize,
    y: usize,
    element: u8,
}

impl<'a> Iterator for ElementIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        for j in self.y..self.grid.height {
            for i in self.x..self.grid.width {
                if self.grid.elems[j][i] == self.element {
                    self.x = i + 1;
                    self.y = j;

                    return Some((i, j));
                }
            }
            self.x = 0;
        }

        None
    }
}

pub struct AdjacentIterator<'a> {
    grid: &'a Grid,
    x: usize,
    y: usize,
    directions: std::vec::IntoIter<Direction>,
}

impl<'a> Iterator for AdjacentIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        for direction in &mut self.directions {
            if let Some(coords) = self.grid.get_coords(direction, self.x, self.y) {
                return Some(coords);
            }
        }

        None
    }
}

pub struct AdjacentElementIterator<'a> {
    grid: &'a Grid,
    element: u8,
    adjacent: AdjacentIterator<'a>,
}

impl<'a> Iterator for AdjacentElementIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        for coords in &mut self.adjacent {
            if self.grid.get(coords) == self.element {
                return Some(coords);
            }
        }

        None
    }
}

#[derive(Clone, Debug)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub elems: Vec<Vec<u8>>,
}

impl Grid {
    pub fn parse_input(input: &str) -> Self {
        Self::new(input.as_bytes().lines().flatten().collect::<Vec<_>>())
    }

    pub fn new(raw: Vec<String>) -> Self {
        Self {
            width: raw[0].len(),
            height: raw.len(),
            elems: raw.iter().map(|s| s.bytes().collect()).collect(),
        }
    }

    pub fn get_coords2(
        &self,
        direction: Direction,
        (x, y): (usize, usize),
    ) -> Option<(usize, usize)> {
        self.get_coords(direction, x, y)
    }

    pub fn get_coords(&self, direction: Direction, x: usize, y: usize) -> Option<(usize, usize)> {
        match direction {
            Direction::North => {
                if y == 0 {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
            Direction::NorthEast => {
                if y == 0 || x >= self.width - 1 {
                    None
                } else {
                    Some((x + 1, y - 1))
                }
            }
            Direction::East => {
                if x >= self.width - 1 {
                    None
                } else {
                    Some((x + 1, y))
                }
            }
            Direction::SouthEast => {
                if y >= self.height - 1 || x >= self.width - 1 {
                    None
                } else {
                    Some((x + 1, y + 1))
                }
            }
            Direction::South => {
                if y >= self.height - 1 {
                    None
                } else {
                    Some((x, y + 1))
                }
            }
            Direction::SouthWest => {
                if y >= self.height - 1 || x == 0 {
                    None
                } else {
                    Some((x - 1, y + 1))
                }
            }
            Direction::West => {
                if x == 0 {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
            Direction::NorthWest => {
                if y == 0 || x == 0 {
                    None
                } else {
                    Some((x - 1, y - 1))
                }
            }
        }
    }

    pub fn find(&self, element: u8) -> ElementIterator<'_> {
        ElementIterator {
            grid: self,
            x: 0,
            y: 0,
            element,
        }
    }

    pub fn adjacent(&self, x: usize, y: usize) -> AdjacentIterator<'_> {
        AdjacentIterator {
            grid: self,
            x,
            y,
            directions: Direction::iter().collect::<Vec<_>>().into_iter(),
        }
    }

    pub fn adjacent_element(&self, x: usize, y: usize, element: u8) -> AdjacentElementIterator<'_> {
        AdjacentElementIterator {
            adjacent: self.adjacent(x, y),
            grid: self,
            element,
        }
    }

    pub fn get(&self, (x, y): (usize, usize)) -> u8 {
        self.elems[y][x]
    }

    pub fn set(&mut self, (x, y): (usize, usize), value: u8) {
        self.elems[y][x] = value;
    }

    pub fn print(&self) {
        for j in 0..self.height {
            println!("{}", String::from_utf8(self.elems[j].clone()).unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_find_should_return_iterator_over_elements() {
        // GIVEN
        let grid = Grid::parse_input(
            r#"000
0a0
0aa
"#,
        );

        // WHEN
        let set = grid.find(b'a').collect::<HashSet<(usize, usize)>>();

        // THEN
        assert_eq!(set.len(), 3);
        assert_eq!(set.contains(&(1, 1)), true);
        assert_eq!(set.contains(&(1, 2)), true);
        assert_eq!(set.contains(&(2, 2)), true);
    }

    #[test]
    fn test_adjacent_should_return_all_valid_neighbors() {
        // GIVEN
        let grid = Grid::parse_input(
            r#"abc
def
ghi
"#,
        );

        // WHEN
        let set = grid.adjacent(1, 1).collect::<HashSet<(usize, usize)>>();

        // THEN
        assert_eq!(set.len(), 8);
        assert_eq!(set.contains(&(0, 0)), true); // NW
        assert_eq!(set.contains(&(1, 0)), true); // N
        assert_eq!(set.contains(&(2, 0)), true); // NE
        assert_eq!(set.contains(&(0, 1)), true); // W
        assert_eq!(set.contains(&(2, 1)), true); // E
        assert_eq!(set.contains(&(0, 2)), true); // SW
        assert_eq!(set.contains(&(1, 2)), true); // S
        assert_eq!(set.contains(&(2, 2)), true); // SE
    }

    #[test]
    fn test_adjacent_should_handle_corner_cells() {
        // GIVEN
        let grid = Grid::parse_input(
            r#"abc
def
ghi
"#,
        );

        // WHEN
        let set = grid.adjacent(0, 0).collect::<HashSet<(usize, usize)>>();

        // THEN
        assert_eq!(set.len(), 3);
        assert_eq!(set.contains(&(1, 0)), true); // E
        assert_eq!(set.contains(&(0, 1)), true); // S
        assert_eq!(set.contains(&(1, 1)), true); // SE
    }

    #[test]
    fn test_adjacent_should_handle_edge_cells() {
        // GIVEN
        let grid = Grid::parse_input(
            r#"abc
def
ghi
"#,
        );

        // WHEN
        let set = grid.adjacent(1, 0).collect::<HashSet<(usize, usize)>>();

        // THEN
        assert_eq!(set.len(), 5);
        assert_eq!(set.contains(&(0, 0)), true); // W
        assert_eq!(set.contains(&(2, 0)), true); // E
        assert_eq!(set.contains(&(0, 1)), true); // SW
        assert_eq!(set.contains(&(1, 1)), true); // S
        assert_eq!(set.contains(&(2, 1)), true); // SE
    }

    #[test]
    fn test_adjacent_element_should_return_matching_neighbors() {
        // GIVEN
        let grid = Grid::parse_input(
            r#"aaa
aba
aaa
"#,
        );

        // WHEN
        let set = grid.adjacent_element(1, 1, b'a').collect::<HashSet<(usize, usize)>>();

        // THEN
        assert_eq!(set.len(), 8);
        assert_eq!(set.contains(&(0, 0)), true);
        assert_eq!(set.contains(&(1, 0)), true);
        assert_eq!(set.contains(&(2, 0)), true);
        assert_eq!(set.contains(&(0, 1)), true);
        assert_eq!(set.contains(&(2, 1)), true);
        assert_eq!(set.contains(&(0, 2)), true);
        assert_eq!(set.contains(&(1, 2)), true);
        assert_eq!(set.contains(&(2, 2)), true);
    }

    #[test]
    fn test_adjacent_element_should_filter_non_matching_neighbors() {
        // GIVEN
        let grid = Grid::parse_input(
            r#"xyz
abc
def
"#,
        );

        // WHEN
        let set = grid.adjacent_element(1, 1, b'a').collect::<HashSet<(usize, usize)>>();

        // THEN
        assert_eq!(set.len(), 1);
        assert_eq!(set.contains(&(0, 1)), true);
    }

    #[test]
    fn test_adjacent_element_should_return_empty_when_no_matches() {
        // GIVEN
        let grid = Grid::parse_input(
            r#"000
010
000
"#,
        );

        // WHEN
        let set = grid.adjacent_element(1, 1, b'2').collect::<HashSet<(usize, usize)>>();

        // THEN
        assert_eq!(set.len(), 0);
    }
}
