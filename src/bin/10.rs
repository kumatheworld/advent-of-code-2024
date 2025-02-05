use advent_of_code::{Matrix, DIJ, IJ};
use itertools::Itertools;

advent_of_code::solution!(10);

fn common(input: &str, apply_unique: bool) -> Option<u32> {
    let mat = Matrix::from(input);

    Some(
        mat.indices()
            .filter(|&ij| mat[ij] == b'0')
            .map(|ij0| {
                let mut buf: Box<dyn Iterator<Item = IJ>> = Box::new(std::iter::once(ij0));
                for level in b'1'..=b'9' {
                    let mat_ref = &mat;
                    buf = Box::new(
                        buf.flat_map(|ij| DIJ.map(|dij| ij + dij))
                            .filter(move |&ij| mat_ref.get(ij) == Some(level)),
                    );
                    if apply_unique {
                        buf = Box::new(buf.unique());
                    }
                }
                buf.count() as u32
            })
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    common(input, true)
}

pub fn part_two(input: &str) -> Option<u32> {
    common(input, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
