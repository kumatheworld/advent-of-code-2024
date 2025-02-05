use advent_of_code::{Index, Matrix, DIJ, IJ};
use itertools::Itertools;

advent_of_code::solution!(12);

fn common(input: &str, use_perimeter: bool) -> Option<u32> {
    let mat = Matrix::from(input);
    let mut groups = mat.new_uniform(None);

    let mut sum = 0;
    let mut gid = 0;
    for ij in mat.indices() {
        if groups[ij].is_some() {
            continue;
        }

        groups[ij] = Some(gid);
        let mut stack = vec![ij];
        let mut area = 1;
        let mut perimeter = 4;

        while !stack.is_empty() {
            let iijj = stack.pop().unwrap();
            DIJ.iter()
                .filter_map(|&dij| (mat.get(iijj + dij) == Some(mat[ij])).then_some(iijj + dij))
                .for_each(|iiijjj| {
                    if groups[iiijjj].is_some() {
                        perimeter -= 1;
                    } else {
                        groups[iiijjj] = Some(gid);
                        area += 1;
                        perimeter += 3;
                        stack.push(iiijjj);
                    }
                });
        }

        sum += area
            * if use_perimeter {
                perimeter
            } else {
                let g = &groups;
                (0..=mat.rows as Index)
                    .flat_map(|i| {
                        (0..mat.cols as Index)
                            .map(move |j| {
                                (g.get(IJ((i - 1, j))) == Some(Some(gid)))
                                    .cmp(&(g.get(IJ((i, j))) == Some(Some(gid))))
                            })
                            .dedup()
                    })
                    .filter(|&o| o.is_ne())
                    .count() as u32
                    + (0..=mat.cols as Index)
                        .flat_map(|j| {
                            (0..mat.rows as Index)
                                .map(move |i| {
                                    (g.get(IJ((i, j - 1))) == Some(Some(gid)))
                                        .cmp(&(g.get(IJ((i, j))) == Some(Some(gid))))
                                })
                                .dedup()
                        })
                        .filter(|&o| o.is_ne())
                        .count() as u32
            };

        gid += 1;
    }

    Some(sum)
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
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_part_one_3() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(436));
    }

    #[test]
    fn test_part_two_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_part_two_4() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(236));
    }

    #[test]
    fn test_part_two_5() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(368));
    }
}
