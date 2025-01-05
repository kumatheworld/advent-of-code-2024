use advent_of_code::{Matrix, DIJ};
use itertools::Itertools;

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u32> {
    const SAVE: u32 = 100;

    let mut mat = Matrix::from(input);
    let mut dist = mat.new_uniform(None);
    let start = mat.position(b'S').unwrap();
    let (mut i, mut j) = start;
    for k in 0.. {
        mat[(i, j)] = b'#';
        dist[(i, j)] = Some(k);
        let ijs = DIJ
            .iter()
            .filter_map(|&(di, dj)| (mat[(i + di, j + dj)] != b'#').then_some((i + di, j + dj)))
            .collect_vec();
        match ijs.len() {
            0 => break,
            1 => (i, j) = ijs[0],
            _ => unreachable!(),
        };
    }

    Some(
        dist.inner_indices()
            .filter(|(i, j)| {
                let ds = DIJ
                    .iter()
                    .filter_map(|(di, dj)| dist[(i + di, j + dj)])
                    .sorted()
                    .collect_vec();
                ds.len() >= 2 && ds[ds.len() - 1] >= ds[0] + SAVE + 2
            })
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
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
