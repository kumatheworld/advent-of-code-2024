use advent_of_code::{Matrix, IJ};
use itertools::iproduct;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mat = Matrix::from(input);
    Some(
        iproduct!(
            mat.indices().filter(|&ij| mat[ij] == b'X'),
            iproduct!(-1..=1, -1..=1)
                .filter(|&(di, dj)| di != 0 || dj != 0)
                .map(IJ)
        )
        .filter(|&(ij, dij)| {
            // Check 'S' first to avoid out-of-bounds errors
            mat.get(ij + 3 * dij) == Some(b'S')
                && mat[ij + 2 * dij] == b'A'
                && mat[ij + dij] == b'M'
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
            .filter(|&ij| {
                mat[ij] == b'A'
                    && MMSS.contains(&[
                        mat[ij + IJ((-1, -1))],
                        mat[ij + IJ((-1, 1))],
                        mat[ij + IJ((1, -1))],
                        mat[ij + IJ((1, 1))],
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
