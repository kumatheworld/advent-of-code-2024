use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut blocks = input
        .trim()
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            std::iter::repeat((i & 1 == 0).then_some(i >> 1)).take(c.to_digit(10).unwrap() as usize)
        })
        .collect_vec();

    let mut sum = 0;
    let mut i = 0;
    while i < blocks.len() {
        sum += i * blocks[i].unwrap_or_else(|| {
            let end = blocks.len() - 1;
            (i..end)
                .filter_map(|_| blocks.pop().unwrap())
                .next()
                .unwrap_or_default()
        });
        i += 1;
    }
    Some(sum as u64)
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
