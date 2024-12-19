use advent_of_code::Matrix;
use itertools::Itertools;

advent_of_code::solution!(12);

fn common(input: &str, use_perimeter: bool) -> Option<u32> {
    const DIJ: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mat = Matrix::from(input);
    let mut groups = mat.new_uniform(None);

    let mut sum = 0;
    let mut gid = 0;
    for (i, j) in mat.indices() {
        if groups[(i, j)].is_some() {
            continue;
        }

        groups[(i, j)] = Some(gid);
        let mut stack = vec![(i, j)];
        let mut area = 1;
        let mut perimeter = 4;

        while !stack.is_empty() {
            let (ii, jj) = stack.pop().unwrap();
            let indices = DIJ.iter().filter_map(|&(di, dj)| {
                (mat.get(ii + di, jj + dj) == Some(mat[(i, j)])).then_some((ii + di, jj + dj))
            });
            for (iii, jjj) in indices {
                if groups[(iii, jjj)].is_some() {
                    perimeter -= 1;
                } else {
                    groups[(iii, jjj)] = Some(gid);
                    area += 1;
                    perimeter += 3;
                    stack.push((iii, jjj));
                }
            }
        }

        sum += area
            * if use_perimeter {
                perimeter
            } else {
                let g = &groups;
                (0..=mat.rows as i32)
                    .flat_map(|i| {
                        (0..mat.cols as i32)
                            .map(move |j| {
                                (g.get(i - 1, j) == Some(Some(gid)))
                                    .cmp(&(g.get(i, j) == Some(Some(gid))))
                            })
                            .dedup()
                    })
                    .filter(|&o| o.is_ne())
                    .count() as u32
                    + (0..=mat.cols as i32)
                        .flat_map(|j| {
                            (0..mat.rows as i32)
                                .map(move |i| {
                                    (g.get(i, j - 1) == Some(Some(gid)))
                                        .cmp(&(g.get(i, j) == Some(Some(gid))))
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
