use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u32> {
    let mut graph = HashMap::<[u8; 2], HashSet<[u8; 2]>>::new();
    for line in input.lines() {
        let (u, v) = line.split_once('-').unwrap();
        let u = <[u8; 2]>::try_from(u.as_bytes()).unwrap();
        let v = <[u8; 2]>::try_from(v.as_bytes()).unwrap();
        graph.entry(u).or_default().insert(v);
        graph.entry(v).or_default().insert(u);
    }

    Some(
        graph
            .iter()
            .flat_map(|(u, vs)| {
                vs.iter().tuple_combinations().filter(|&(v, w)| {
                    [u[0], v[0], w[0]].contains(&b't') && graph.get(v).unwrap().contains(w)
                })
            })
            .count() as u32
            / 3,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
