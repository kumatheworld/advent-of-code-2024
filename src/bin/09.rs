use itertools::Itertools;

advent_of_code::solution!(9);

fn common(input: &str, move_blocks: fn(&str, &mut Vec<Option<usize>>)) -> Option<u64> {
    let mut blocks = input
        .trim()
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            std::iter::repeat((i & 1 == 0).then_some(i >> 1)).take(c.to_digit(10).unwrap() as usize)
        })
        .collect_vec();

    move_blocks(input, &mut blocks);

    Some(
        blocks
            .iter()
            .enumerate()
            .map(|(i, j)| i as u64 * j.unwrap_or_default() as u64)
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    common(input, |_, blocks| {
        let mut i = 0;
        while i < blocks.len() {
            if blocks[i].is_none() {
                let end = blocks.len() - 1;
                blocks[i] = (i..end).filter_map(|_| blocks.pop().unwrap()).next()
            }
            i += 1;
        }
    })
}

pub fn part_two(input: &str) -> Option<u64> {
    common(input, |input_, blocks| {
        let (somes, mut nones): (Vec<_>, Vec<_>) = input_
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .enumerate()
            .partition(|(i, _)| i & 1 == 0);

        let mut cumsum = somes
            .iter()
            .interleave(nones.iter())
            .map(|&(_, a)| a)
            .scan(0, |a, b| {
                *a += b;
                Some(*a as usize)
            })
            .collect_vec();

        for &(i, some) in somes.iter().rev() {
            if let Some(&(j, _)) = nones[..i >> 1].iter().find(|&&(_, cap)| cap >= some) {
                let left = cumsum[j - 1];
                let right = cumsum[i - 1];
                for k in 0..some as usize {
                    blocks.swap(left + k, right + k);
                }
                cumsum[j - 1] += some as usize;
                nones[j >> 1].1 -= some;
            }
        }
    })
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
