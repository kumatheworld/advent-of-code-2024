advent_of_code::solution!(7);

fn add(m: u64, n: u64) -> u64 {
    m + n
}

fn mul(m: u64, n: u64) -> u64 {
    m * n
}

fn cat(m: u64, n: u64) -> u64 {
    (m.to_string() + &n.to_string()).parse().unwrap()
}

fn common(input: &str, ops: &[&dyn Fn(u64, u64) -> u64]) -> Option<u64> {
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
                    let n = num_str.parse::<u64>().unwrap();
                    it = Box::new(it.flat_map(move |m| ops.iter().map(move |&f| f(m, n))));
                }

                it.find(|&n| n == test)
            })
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    common(input, &[&add, &mul])
}

pub fn part_two(input: &str) -> Option<u64> {
    common(input, &[&add, &mul, &cat])
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
        assert_eq!(result, Some(11387));
    }
}
