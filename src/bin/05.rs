use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let rules_and_seqs = input.split("\n\n").collect_vec();

    let rules = rules_and_seqs[0]
        .lines()
        .filter_map(|line| line.split('|').next_tuple())
        .map(|(m, n)| (m.parse::<u32>().unwrap(), n.parse::<u32>().unwrap()));
    let mut one2many: HashMap<u32, HashSet<u32>> = HashMap::new();
    for (m, n) in rules {
        one2many.entry(m).or_insert_with(HashSet::new).insert(n);
    }

    Some(
        rules_and_seqs[1]
            .lines()
            .map(|line| {
                let pages = line
                    .split(',')
                    .map(|elem| elem.parse::<u32>().unwrap())
                    .collect_vec();
                if pages.iter().enumerate().all(|(i, p)| {
                    pages[i + 1..]
                        .iter()
                        .copied()
                        .collect::<HashSet<_>>()
                        .is_subset(one2many.entry(*p).or_default())
                }) {
                    pages[pages.len() / 2]
                } else {
                    0
                }
            })
            .sum(),
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
