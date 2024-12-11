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

    let mut i = 0;
    while i < blocks.len() {
        if blocks[i].is_none() {
            let end = blocks.len() - 1;
            blocks[i] = (i..end).filter_map(|_| blocks.pop().unwrap()).next()
        }
        i += 1;
    }

    Some(
        blocks
            .iter()
            .enumerate()
            .map(|(i, j)| i as u64 * j.unwrap_or_default() as u64)
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
