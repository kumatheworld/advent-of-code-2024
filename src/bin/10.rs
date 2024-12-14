use advent_of_code::Matrix;
use itertools::{iproduct, Itertools};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    static D: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mat = Matrix::from(input);
    let zeros =
        iproduct!(0..mat.rows as i32, 0..mat.cols as i32).filter(|&(i, j)| mat[(i, j)] == b'0');

    Some(
        zeros
            .map(|(i0, j0)| {
                let mut buf: Box<dyn Iterator<Item = (i32, i32)>> =
                    Box::new(std::iter::once((i0, j0)));
                for level in b'1'..=b'9' {
                    let mat_ref = &mat;
                    buf = Box::new(
                        buf.flat_map(|(i, j)| D.map(|(di, dj)| (i + di, j + dj)))
                            .filter(move |&(i, j)| mat_ref.get(i, j) == Some(level))
                            .unique(),
                    );
                }
                buf.count() as u32
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
