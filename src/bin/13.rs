use std::ops::Range;

use itertools::Itertools;

advent_of_code::solution!(13);

fn common(input: &str, range: Range<i64>, offset: i64) -> Option<u64> {
    let re = regex::Regex::new(
        r"^Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)$",
    )
    .unwrap();

    Some(
        input
            .trim()
            .split("\n\n")
            .filter_map(|machine| {
                let (ax, ay, bx, by, px_, py_) = re
                    .captures(machine)
                    .unwrap()
                    .iter()
                    .skip(1)
                    .map(|x| x.unwrap().as_str().parse::<i64>().unwrap())
                    .collect_tuple()
                    .unwrap();

                let px = px_ + offset;
                let py = py_ + offset;

                let det = ax * by - bx * ay;
                if det == 0 {
                    None
                } else {
                    let adet = by * px - bx * py;
                    let bdet = ax * py - ay * px;
                    let a = adet / det;
                    let b = bdet / det;
                    (range.contains(&a) && range.contains(&b) && adet % det == 0 && bdet % det == 0)
                        .then(|| 3 * a as u64 + b as u64)
                }
            })
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    common(input, 0..100, 0)
}

pub fn part_two(input: &str) -> Option<u64> {
    common(input, 0..i64::MAX, 10000000000000)
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
        assert_eq!(result, Some(875318608908)); // Test written after the submission was accepted
    }
}
