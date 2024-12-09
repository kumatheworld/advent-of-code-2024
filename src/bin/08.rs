use advent_of_code::Matrix;
use itertools::iproduct;
use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut mat = Matrix::from(input);

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

    Some(
        antennas
            .iter()
            .map(|(_, v)| {
                iproduct!(v, v)
                    .filter(|((i0, j0), (i1, j1))| {
                        (i0, j0) != (i1, j1) && {
                            let i2 = 2 * i1 - i0;
                            let j2 = 2 * j1 - j0;
                            ![None, Some(b'#')].contains(&mat.get(2 * i1 - i0, 2 * j1 - j0)) && {
                                mat[(i2, j2)] = b'#';
                                true
                            }
                        }
                    })
                    .count() as u32
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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
