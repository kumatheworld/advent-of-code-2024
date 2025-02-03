use advent_of_code::{Index, Matrix, DIJ};
use itertools::Itertools;

advent_of_code::solution!(20);

pub fn common(input: &str, picoseconds: u32) -> Option<u32> {
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
            .tuple_combinations()
            .filter(|&((i0, j0), (i1, j1))| {
                if let (Some(d0), Some(d1)) = (dist[(i0, j0)], dist[(i1, j1)]) {
                    let dij = i0.abs_diff(i1) + j0.abs_diff(j1);
                    dij <= picoseconds && (d0 as Index).abs_diff(d1) >= SAVE + dij
                } else {
                    false
                }
            })
            .count() as u32,
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    common(input, 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    common(input, 20)
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
