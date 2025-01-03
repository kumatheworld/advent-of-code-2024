advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let (stripes, designs) = input.split_once("\n\n").unwrap();
    let re = regex::Regex::new(format!("^({})*$", stripes.replace(", ", "|")).as_str()).unwrap();
    Some(designs.lines().filter(|line| re.is_match(line)).count() as u32)
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
