advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut first_column = Vec::new();
    let mut second_column = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let first = parts
            .next()
            .ok_or("Missing first column")
            .ok()?
            .trim()
            .parse::<i32>()
            .ok()?;
        let second = parts
            .next()
            .ok_or("Missing second column")
            .ok()?
            .trim()
            .parse::<i32>()
            .ok()?;

        first_column.push(first);
        second_column.push(second);
    }

    first_column.sort();
    second_column.sort();

    let mut sum: u32 = 0;
    for (first, second) in first_column.iter().zip(second_column.iter()) {
        sum += (first - second).abs() as u32;
    }

    Some(sum)
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
