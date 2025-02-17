use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

fn common(input: &str, ok_or_err: fn(Result<u32, u32>) -> Option<u32>) -> Option<u32> {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let mut one2many: HashMap<u32, HashSet<u32>> = HashMap::new();
    rules
        .lines()
        .map(|line| {
            line.split_once('|')
                .map(|(m, n)| (m.parse::<u32>().unwrap(), n.parse::<u32>().unwrap()))
                .unwrap()
        })
        .for_each(|(m, n)| {
            one2many.entry(m).or_default().insert(n);
        });

    Some(
        updates
            .lines()
            .map(move |line| {
                let pages = line
                    .split(',')
                    .map(|elem| elem.parse::<u32>().unwrap())
                    .collect_vec();

                // Count the number of subsequent pages for every page
                // It is a permutation of 0..pages.len() thanks to the fact that p|p does not hold
                // and that either one of p|q or q|p holds for each distinct pair (p, q) of pages
                let num_next_pages = pages
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

                let mid = pages[num_next_pages
                    .iter()
                    .position(|&i| i == pages.len() >> 1)
                    .unwrap()];

                (if num_next_pages == (0..pages.len()).rev().collect_vec() {
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
