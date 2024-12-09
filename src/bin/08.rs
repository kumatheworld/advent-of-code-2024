use advent_of_code::Matrix;
use itertools::{iproduct, iterate};
use std::collections::HashMap;
advent_of_code::solution!(8);

fn prepare(input: &str) -> (Matrix, HashMap<u8, Vec<(i32, i32)>>) {
    let mat = Matrix::from(input);

    let mut antennas: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
    for i in 0..mat.rows as i32 {
        for j in 0..mat.cols as i32 {
            if mat[(i, j)] != b'.' {
                antennas
                    .entry(mat[(i, j)])
                    .or_insert_with(Vec::new)
                    .push((i, j));
            }
        }
    }

    (mat, antennas)
}

fn build_antenna(mat: &mut Matrix, i: i32, j: i32) -> bool {
    ![None, Some(b'#')].contains(&mat.get(i, j)) && {
        mat[(i, j)] = b'#';
        true
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut mat, antennas) = prepare(input);
    Some(
        antennas
            .iter()
            .map(|(_, v)| {
                iproduct!(v, v)
                    .filter(|((i0, j0), (i1, j1))| {
                        (i0, j0) != (i1, j1) && build_antenna(&mut mat, 2 * i1 - i0, 2 * j1 - j0)
                    })
                    .count() as u32
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut mat, antennas) = prepare(input);
    let m = mat.rows as i32;
    let n = mat.cols as i32;
    Some(
        antennas
            .iter()
            .map(|(_, v)| {
                iproduct!(v, v)
                    .filter(|(p, q)| p != q)
                    .flat_map(|(&(i0, j0), &(i1, j1))| {
                        iterate((i1, j1), move |&(i, j)| (i + i1 - i0, j + j1 - j0))
                            .take_while(|&(i, j)| (0..m).contains(&i) && (0..n).contains(&j))
                    })
                    .filter(|&(i, j)| build_antenna(&mut mat, i, j))
                    .count() as u32
            })
            .sum(),
    )
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
