use advent_of_code::{Matrix, IJ};
use itertools::{iproduct, iterate, Itertools};
use std::collections::HashMap;

advent_of_code::solution!(8);

fn common<F>(input: &str, yield_antennas: F) -> Option<u32>
where
    F: Fn(IJ, IJ, &Matrix<u8>) -> Box<dyn Iterator<Item = IJ> + '_>,
{
    let mat = Matrix::from(input);
    let mut antennas: HashMap<u8, Vec<IJ>> = HashMap::new();
    for ij in mat.indices() {
        if mat[ij] != b'.' {
            antennas.entry(mat[ij]).or_insert_with(Vec::new).push(ij);
        }
    }

    Some(
        antennas
            .iter()
            .flat_map(|(_, v)| {
                iproduct!(v, v)
                    .filter(|(p, q)| p != q)
                    .flat_map(|(&ij0, &ij1)| yield_antennas(ij0, ij1, &mat))
            })
            .unique()
            .count() as u32,
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    common(input, |ij0, ij1, mat| {
        let ij2 = 2 * ij1 - ij0;
        Box::new(mat.get(ij2).is_some().then_some(ij2).into_iter())
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    common(input, |ij0, ij1, mat| {
        Box::new(iterate(ij1, move |&ij| ij + ij1 - ij0).take_while(|&ij| mat.get(ij).is_some()))
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
