use std::ops::RangeInclusive;

use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    const LE100: RangeInclusive<i32> = 0..=100;

    let re = Regex::new(
        r"^Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)$",
    )
    .unwrap();

    Some(
        input
            .trim()
            .split("\n\n")
            .filter_map(|machine| {
                let (ax, ay, bx, by, px, py) = re
                    .captures(machine)
                    .unwrap()
                    .iter()
                    .skip(1)
                    .map(|x| x.unwrap().as_str().parse::<i32>().unwrap())
                    .collect_tuple()
                    .unwrap();

                let det = ax * by - bx * ay;
                if det == 0 {
                    None
                } else {
                    let adet = by * px - bx * py;
                    let bdet = ax * py - ay * px;
                    let a = adet / det;
                    let b = bdet / det;
                    (LE100.contains(&a) && LE100.contains(&b) && adet % det == 0 && bdet % det == 0)
                        .then(|| 3 * a as u32 + b as u32)
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
