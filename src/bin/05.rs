use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

fn common(input: &str, ok_or_err: fn(Result<u32, u32>) -> Option<u32>) -> Option<u32> {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let mut one2many: HashMap<u32, HashSet<u32>> = HashMap::new();
    for line in rules.lines() {
        let (p, q) = line.split_once('|').unwrap();
        let p = p.parse().unwrap();
        let q = q.parse().unwrap();
        one2many.entry(q).or_default().insert(p);
    }

    Some(
        updates
            .lines()
            .map(move |line| {
                let pages = line
                    .split(',')
                    .map(|elem| elem.parse::<u32>().unwrap())
                    .collect_vec();

                // Count the number of previous pages for every page
                // It is a permutation of 0..pages.len() thanks to irreflexiveness and anti-symmetry
                let num_prev_pages = pages
                    .iter()
                    .map(|p| {
                        pages
                            .iter()
                            .copied()
                            .collect::<HashSet<_>>()
                            .intersection(one2many.get(p).unwrap_or(&HashSet::new()))
                            .count()
                    })
                    .collect_vec();

                let mid = pages[num_prev_pages
                    .iter()
                    .position(|&i| i == pages.len() >> 1)
                    .unwrap()];

                (if num_prev_pages == (0..pages.len()).collect_vec() {
                    Ok
                } else {
                    Err
                })(mid)
            })
            .filter_map(ok_or_err)
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    common(input, |r| r.ok())
}

pub fn part_two(input: &str) -> Option<u32> {
    common(input, |r| r.err())
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
        assert_eq!(result, Some(123));
    }
}
