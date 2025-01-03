advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let (stripes, designs) = input.split_once("\n\n").unwrap();
    let re = regex::Regex::new(format!("^({})*$", stripes.replace(", ", "|")).as_str()).unwrap();
    Some(designs.lines().filter(|line| re.is_match(line)).count() as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (stripes, designs) = input.split_once("\n\n").unwrap();
    let stripes: Vec<&str> = stripes.split(", ").collect();
    Some(
        designs
            .lines()
            .map(move |line| {
                let mut dp = vec![0; line.len() + 1];
                dp[0] = 1;
                for i in 1..=line.len() {
                    for &s in &stripes {
                        if s.len() <= i {
                            let j = i - s.len();
                            if line[j..i] == *s {
                                dp[i] += dp[j];
                            }
                        }
                    }
                }
                dp[line.len()]
            })
            .sum(),
    )
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
        assert_eq!(result, Some(16));
    }
}
