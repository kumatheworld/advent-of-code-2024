use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    Some(
        re.captures_iter(input)
            .map(|c| c.extract())
            .map(|(_, [m, n])| (m.parse::<u32>().unwrap(), n.parse::<u32>().unwrap()))
            .map(|(m, n)| m * n)
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    part_one(
        &input
            .split("do()")
            .map(|s| &s[0..s.find("don't()").unwrap_or(s.len())])
            .join(""),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
