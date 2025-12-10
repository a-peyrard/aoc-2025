use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}};

use advent_of_code::util::union_find::UnionFind;

advent_of_code::solution!(8);

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Box(u64, u64, u64);

impl Box {
    fn parse(line: &str) -> Self {
        let mut parts = line.split(',');
        Box::new(
            parts.next().unwrap().parse::<u64>().unwrap(),
            parts.next().unwrap().parse::<u64>().unwrap(),
            parts.next().unwrap().parse::<u64>().unwrap(),
        )
    }

    fn new(x: u64, y: u64, z: u64) -> Self {
        Box(x, y, z)
    }

    fn distance(&self, o: &Box) -> u64 {
        ((o.0 as i64 - self.0 as i64).pow(2)
            + (o.1 as i64 - self.1 as i64).pow(2)
            + (o.2 as i64 - self.2 as i64).pow(2)) as u64
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    part_one_with_sample(input, 1000)
}

pub fn part_one_with_sample(input: &str, samples: usize) -> Option<u64> {
    let boxes: Vec<Box> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Box::parse)
        .collect();

    let mut heap: BinaryHeap<(u64, Box, Box)> = BinaryHeap::new();

    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            let box1 = &boxes[i];
            let box2 = &boxes[j];
            let distance = box1.distance(box2);
            heap.push((distance, box1.clone(), box2.clone()));
            if heap.len() > samples {
                heap.pop();
            }
        }
    }

    let mut union_find = UnionFind::new(boxes.len());
    let mut box_mapping: HashMap<Box, usize> = HashMap::new();
    while let Some((_, box1, box2)) = heap.pop() {
        let len = box_mapping.len();
        let box1_idx = *box_mapping.entry(box1).or_insert(len);
        let len = box_mapping.len();
        let box2_idx = *box_mapping.entry(box2).or_insert(len);

        union_find.union(box1_idx, box2_idx);
    }

    let mut component_sizes: BinaryHeap<u64> = (0..box_mapping.len())
        .map(|i| union_find.find(i))
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|root| union_find.component_size(root) as u64)
        .collect();

    Some(
        std::iter::from_fn(|| component_sizes.pop())
            .take(3)
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let boxes: Vec<Box> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Box::parse)
        .collect();

    let mut heap: BinaryHeap<Reverse<(u64, Box, Box)>> = BinaryHeap::new();

    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            let box1 = &boxes[i];
            let box2 = &boxes[j];
            let distance = box1.distance(box2);
            heap.push(Reverse((distance, box1.clone(), box2.clone())));
        }
    }

    let mut union_find = UnionFind::new(boxes.len());
    let mut box_mapping: HashMap<Box, usize> = HashMap::new();
    while let Some(Reverse((_, box1, box2))) = heap.pop() {
        let x1 = box1.0;
        let x2 = box2.0;

        let len = box_mapping.len();
        let box1_idx = *box_mapping.entry(box1).or_insert(len);
        let len = box_mapping.len();
        let box2_idx = *box_mapping.entry(box2).or_insert(len);

        union_find.union(box1_idx, box2_idx);

        if union_find.num_components() == 1 {
            return Some(x1 * x2);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one_with_sample(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
