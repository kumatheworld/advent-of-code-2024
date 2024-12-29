use advent_of_code::Matrix;
use itertools::{iproduct, Itertools};
use std::cmp::Ordering;
use std::collections::HashMap;

advent_of_code::solution!(16);

fn between(a: i32, b: i32) -> std::ops::Range<i32> {
    std::cmp::min(a, b) + 1..std::cmp::max(a, b)
}

fn common(
    input: &str,
) -> (
    (i32, i32),
    Matrix<u8>,
    HashMap<(i32, i32), (bool, u32, Vec<(i32, i32)>)>,
) {
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
                    ((i != ii || j != jj)
                        && (i == ii && between(j, jj).all(|jjj| mat_ref[(i, jjj)] == b'.')
                            || j == jj && between(i, ii).all(|iii| mat_ref[(iii, j)] == b'.')))
                    .then(|| {
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
    // (visited, distance, possible previous nodes that give the shortest path)
    let mut dp: HashMap<(i32, i32), (bool, u32, Vec<(i32, i32)>)> = nodes
        .iter()
        .map(|&ij| (ij, (false, if ij == start { 0 } else { u32::MAX }, vec![])))
        .collect();
    loop {
        let (&u, &(_, d, _)) = dp
            .iter()
            .filter(|(_, (b, _, _))| !b)
            .min_by_key(|(_, (_, dd, _))| dd)
            .unwrap();
        if u == end {
            return (end, mat, dp);
        }
        dp.get_mut(&u).unwrap().0 = true;
        for &(v, weight) in edges[&u].iter() {
            let (_, dd, ps) = dp.get_mut(&v).unwrap();
            match (d + weight).cmp(dd) {
                Ordering::Less => {
                    *dd = d + weight;
                    *ps = vec![u];
                }
                Ordering::Equal => {
                    ps.push(u);
                }
                Ordering::Greater => {}
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (end, _, dp) = common(input);
    dp.get(&end).map(|&(_, d, _)| d)
}

pub fn part_two(input: &str) -> Option<u32> {
    fn traverse(
        (i, j): (i32, i32),
        seen: &mut Matrix<bool>,
        dp: &HashMap<(i32, i32), (bool, u32, Vec<(i32, i32)>)>,
    ) {
        seen[(i, j)] = true;
        for &(ii, jj) in &dp[&(i, j)].2 {
            if i == ii {
                for jjj in between(j, jj) {
                    seen[(i, jjj)] = true;
                }
            } else if j == jj {
                for iii in between(i, ii) {
                    seen[(iii, j)] = true;
                }
            } else {
                unreachable!();
            }
            traverse((ii, jj), seen, dp);
        }
    }

    let (end, mat, dp) = common(input);
    let mut seen = mat.new_uniform(false);
    traverse(end, &mut seen, &dp);

    Some(seen.indices().filter(|&ij| seen[ij]).count() as u32)
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
