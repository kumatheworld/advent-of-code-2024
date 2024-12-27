use advent_of_code::Matrix;
use itertools::{iproduct, Itertools};
use petgraph::algo::dijkstra;
use petgraph::graph::UnGraph;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u32> {
    const TURN_PENALTY: u32 = 1000;
    const DIJ: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let linear = [vec![DIJ[0], DIJ[2]], vec![DIJ[1], DIJ[3]]];

    let mat = Matrix::from(input);
    let bottom = mat.rows as i32 - 2;
    let start = mat.serialize((bottom, 1)) as u32;
    let end = mat.serialize((1, mat.cols as i32 - 2)) as u32;

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
    let edges = nodes
        .iter()
        .flat_map(|&(i, j)| {
            let mat_ref = &mat;
            nodes.iter().filter_map(move |&(ii, jj)| {
                let connected = ii == i && {
                    (std::cmp::min(j, jj) + 1..std::cmp::max(j, jj))
                        .all(|jjj| mat_ref[(i, jjj)] == b'.')
                } || jj == j && {
                    (std::cmp::min(i, ii) + 1..std::cmp::max(i, ii))
                        .all(|iii| mat_ref[(iii, j)] == b'.')
                };
                connected.then(|| {
                    (
                        (i, j),
                        (ii, jj),
                        (TURN_PENALTY
                            * !(i == bottom && ii == bottom && [j, jj].contains(&1)) as u32
                            + i.abs_diff(ii)
                            + j.abs_diff(jj)),
                    )
                })
            })
        })
        .map(|(ij0, ij1, w)| (mat.serialize(ij0) as u32, mat.serialize(ij1) as u32, w));
    let g = UnGraph::<u32, u32>::from_edges(edges);

    dijkstra(&g, start.into(), Some(end.into()), |e| *e.weight())
        .get(&end.into())
        .map(|&s| s)
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
        assert_eq!(result, None);
    }
}
