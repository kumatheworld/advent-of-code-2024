use std::collections::HashMap;

use advent_of_code::Matrix;
use itertools::{iproduct, Itertools};

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u32> {
    const TURN_PENALTY: u32 = 1000;
    const DIJ: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let linear = [vec![DIJ[0], DIJ[2]], vec![DIJ[1], DIJ[3]]];

    let mat = Matrix::from(input);
    let bottom = mat.rows as i32 - 2;
    let start = (bottom, 1);
    let end = (1, mat.cols as i32 - 2);

    let nodes = iproduct!(0..mat.rows as i32 - 1, 0..mat.cols as i32 - 1)
        .filter(|&(i, j)| {
            mat[(i, j)] != b'#' && {
                let ds = DIJ
                    .into_iter()
                    .filter(|(di, dj)| mat[(i + di, j + dj)] == b'.')
                    .collect_vec();
                !linear.contains(&ds)
            }
        })
        .collect_vec();
    let edges: HashMap<(i32, i32), Vec<((i32, i32), u32)>> = nodes
        .iter()
        .map(|&(i, j)| {
            let mat_ref = &mat;
            let v = nodes
                .iter()
                .filter_map(move |&(ii, jj)| {
                    let connected = ii == i && {
                        (std::cmp::min(j, jj) + 1..std::cmp::max(j, jj))
                            .all(|jjj| mat_ref[(i, jjj)] == b'.')
                    } || jj == j && {
                        (std::cmp::min(i, ii) + 1..std::cmp::max(i, ii))
                            .all(|iii| mat_ref[(iii, j)] == b'.')
                    };
                    connected.then(|| {
                        (
                            (ii, jj),
                            (TURN_PENALTY
                                * !(i == bottom && ii == bottom && [j, jj].contains(&1)) as u32
                                + i.abs_diff(ii)
                                + j.abs_diff(jj)),
                        )
                    })
                })
                .collect_vec();
            ((i, j), v)
        })
        .collect();

    // Dijkstra
    let mut dp: HashMap<(i32, i32), (bool, u32)> = nodes
        .iter()
        .map(|&ij| (ij, (false, if ij == start { 0 } else { u32::MAX })))
        .collect();
    loop {
        let (&u, &(_, d)) = dp
            .iter()
            .filter(|(_, (b, _))| !b)
            .min_by_key(|(_, (_, d))| d)
            .unwrap();
        if u == end {
            return Some(d as u32);
        }
        dp.insert(u, (true, d));
        for &(v, weight) in edges[&u].iter() {
            let (_, dd) = dp.get_mut(&v)?;
            *dd = std::cmp::min(*dd, d + weight);
        }
    }
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
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }
}
