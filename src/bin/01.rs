use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut first_column = Vec::new();
    let mut second_column = Vec::new();

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let first = parts[0].parse::<i32>().unwrap();
        let second = parts[1].parse::<i32>().unwrap();

        first_column.push(first);
        second_column.push(second);
    }

    (first_column, second_column)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut first_column, mut second_column) = parse(input);

    first_column.sort();
    second_column.sort();

    let mut sum: u32 = 0;
    for (first, second) in first_column.iter().zip(second_column.iter()) {
        sum += (first - second).abs() as u32;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (first_column, second_column) = parse(input);
    let mut counter: HashMap<i32, u32> = HashMap::new();

    for key in second_column {
        let value = counter.entry(key).or_default();
        *value += 1;
    }

    let mut sum: u32 = 0;
    for key in first_column {
        let value = counter.entry(key).or_default();
        sum += key as u32 * *value;
    }

    Some(sum)
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
