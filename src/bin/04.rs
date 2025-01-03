use advent_of_code::Matrix;
use itertools::iproduct;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mat = Matrix::from(input);
    Some(
        iproduct!(
            mat.indices().filter(|&(i, j)| mat[(i, j)] == b'X'),
            iproduct!(-1..=1, -1..=1).filter(|&(di, dj)| di != 0 || dj != 0)
        )
        .filter(|&((i, j), (di, dj))| {
            // Check 'S' first to avoid out-of-bounds errors
            mat.get((i + 3 * di, j + 3 * dj)) == Some(b'S')
                && mat[(i + 2 * di, j + 2 * dj)] == b'A'
                && mat[(i + di, j + dj)] == b'M'
        })
        .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mat = Matrix::from(input);
    const MMSS: [[u8; 4]; 4] = [
        [b'M', b'M', b'S', b'S'],
        [b'M', b'S', b'M', b'S'],
        [b'S', b'M', b'S', b'M'],
        [b'S', b'S', b'M', b'M'],
    ];
    Some(
        mat.inner_indices()
            .filter(|&(i, j)| {
                mat[(i, j)] == b'A'
                    && MMSS.contains(&[
                        mat[(i - 1, j - 1)],
                        mat[(i - 1, j + 1)],
                        mat[(i + 1, j - 1)],
                        mat[(i + 1, j + 1)],
                    ])
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
