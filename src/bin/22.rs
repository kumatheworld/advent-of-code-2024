advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u64> {
    const REPS: usize = 2000;
    Some(
        input
            .lines()
            .map(|line| {
                let mut n = line.parse::<u64>().unwrap();
                for _ in 0..REPS {
                    n = ((n << 6) ^ n) & 16777215;
                    n = ((n >> 5) ^ n) & 16777215;
                    n = ((n << 11) ^ n) & 16777215;
                }
                n
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
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
