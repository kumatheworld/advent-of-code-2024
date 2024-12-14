advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let mut buf: Box<dyn Iterator<Item = String>> =
        Box::new(input.trim().split(' ').map(|s| s.to_string()));

    for _ in 0..25 {
        buf = Box::new(buf.flat_map(|mut s| {
            s.drain(..s.find(|c| c != '0').unwrap_or(s.len()));
            if s.is_empty() {
                vec!["1".to_string()]
            } else if s.len() & 1 == 0 {
                let latter = s.split_off(s.len() >> 1);
                vec![s, latter]
            } else {
                vec![(s.parse::<u64>().unwrap() * 2024).to_string()]
            }
        }))
    }

    Some(buf.count() as u64)
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
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
