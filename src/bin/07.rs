advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                let (test_str, nums_str) = line.split_once(": ").unwrap();
                let test: u64 = test_str.parse().unwrap();
                let (first, rest) = nums_str.split_once(' ').unwrap();

                let first_num = first.parse().unwrap();
                let mut it: Box<dyn Iterator<Item = u64>> = Box::new(std::iter::once(first_num));
                for num_str in rest.split(' ') {
                    let num = num_str.parse::<u64>().unwrap();
                    it = Box::new(it.flat_map(move |n| [num + n, num * n]));
                }

                it.find(|&n| n == test)
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
