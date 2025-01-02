use advent_of_code::Matrix;
use itertools::{iproduct, iterate, Itertools};
use std::collections::HashMap;

advent_of_code::solution!(8);

fn common<F>(input: &str, yield_antennas: F) -> Option<u32>
where
    F: Fn((i32, i32), (i32, i32), &Matrix<u8>) -> Box<dyn Iterator<Item = (i32, i32)> + '_>,
{
    let mat = Matrix::from(input);
    let mut antennas: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
    for (i, j) in mat.indices() {
        if mat[(i, j)] != b'.' {
            antennas
                .entry(mat[(i, j)])
                .or_insert_with(Vec::new)
                .push((i, j));
        }
    }

    Some(
        antennas
            .iter()
            .flat_map(|(_, v)| {
                iproduct!(v, v)
                    .filter(|(p, q)| p != q)
                    .flat_map(|(&(i0, j0), &(i1, j1))| yield_antennas((i0, j0), (i1, j1), &mat))
            })
            .unique()
            .count() as u32,
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    common(input, |(i0, j0), (i1, j1), mat| {
        let ij2 = (2 * i1 - i0, 2 * j1 - j0);
        Box::new(mat.get(ij2).is_some().then_some(ij2).into_iter())
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    common(input, |(i0, j0), (i1, j1), mat| {
        Box::new(
            iterate((i1, j1), move |(i, j)| (i + i1 - i0, j + j1 - j0))
                .take_while(|&ij| mat.get(ij).is_some()),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
