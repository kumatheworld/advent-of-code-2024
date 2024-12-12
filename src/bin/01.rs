use itertools::Itertools;

advent_of_code::solution!(1);

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let (m, n) = line.split_once("   ").unwrap();
            (m.parse::<u32>().unwrap(), n.parse::<u32>().unwrap())
        })
        .unzip()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (left, right) = parse(input);
    Some(
        left.iter()
            .sorted()
            .zip(right.iter().sorted())
            .map(|(m, n)| m.abs_diff(*n))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse(input);
    Some(
        left.iter()
            .map(|m| m * right.iter().filter(|n| *n == m).count() as u32)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
