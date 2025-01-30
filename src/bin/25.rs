use advent_of_code::Matrix;
use itertools::{iproduct, Itertools};

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u32> {
    let mut locks = Vec::<Vec<i32>>::new();
    let mut keys = Vec::<Vec<i32>>::new();
    for block in input.trim().split("\n\n") {
        let mat = Matrix::from(block);
        let mat00 = mat[(0, 0)];
        match mat00 {
            b'#' => &mut locks,
            b'.' => &mut keys,
            _ => unreachable!(),
        }
        .push(
            (0..mat.cols as i32)
                .map(move |j| {
                    (1..mat.rows as i32)
                        .find(|&i| mat[(i, j)] != mat[(0, 0)])
                        .unwrap()
                })
                .collect_vec(),
        );
    }
    Some(
        iproduct!(locks, keys)
            .filter(|(l, k)| l.iter().zip_eq(k).all(|(a, b)| a <= b))
            .count() as u32,
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
        assert_eq!(result, Some(3));
    }
}
