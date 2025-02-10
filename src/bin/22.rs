use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(22);

const REPS: usize = 2000;

fn next_secret(ref_n: &mut u64) -> u64 {
    let mut n = *ref_n;
    n = ((n << 6) ^ n) & 0xffffff;
    n = ((n >> 5) ^ n) & 0xffffff;
    n = ((n << 11) ^ n) & 0xffffff;
    *ref_n = n;
    n
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut n = line.parse::<u64>().unwrap();
                for _ in 0..REPS {
                    next_secret(&mut n);
                }
                n
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u16> {
    input
        .lines()
        .map(|line| {
            let mut n = line.parse::<u64>().unwrap();
            let prices = std::iter::once((n % 10) as i8)
                .chain((0..REPS).map(move |_| (next_secret(&mut n) % 10) as i8))
                .collect_vec();

            let mut changes_and_prices = HashMap::new();
            prices
                .iter()
                .tuple_windows()
                .map(|(&a, &b)| (a - b) as i8)
                .tuple_windows::<(_, _, _, _)>()
                .zip_eq(&prices[4..])
                .for_each(|(changes, &price)| {
                    changes_and_prices.entry(changes).or_insert(price as u16);
                });
            changes_and_prices
        })
        .reduce(|mut acc, caps| {
            for (changes, price) in caps {
                *acc.entry(changes).or_insert(0) += price;
            }
            acc
        })?
        .into_values()
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
