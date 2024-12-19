use advent_of_code::Matrix;
use itertools::Itertools;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    const DIJ: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mat = Matrix::from(input);
    let mut seen = mat.new_uniform(false);

    let mut sum = 0;
    for (i, j) in mat.indices() {
        if seen[(i, j)] {
            continue;
        }

        let c = mat[(i, j)];
        seen[(i, j)] = true;
        let mut stack = vec![(i, j)];
        let mut area = 1;
        let mut perimeter = 4;
        while !stack.is_empty() {
            let (ii, jj) = stack.pop().unwrap();
            let indices = DIJ.iter().filter_map(|&(di, dj)| {
                (mat.get(ii + di, jj + dj) == Some(c)).then_some((ii + di, jj + dj))
            });
            for (iii, jjj) in indices {
                if seen[(iii, jjj)] {
                    perimeter -= 1;
                } else {
                    seen[(iii, jjj)] = true;
                    area += 1;
                    perimeter += 3;
                    stack.push((iii, jjj));
                }
            }
        }
        sum += area * perimeter
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    const DIJ: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mat = Matrix::from(input);
    let mut seen = mat.new_uniform(false);

    let mut sum = 0;
    for (i, j) in mat.indices() {
        if seen[(i, j)] {
            continue;
        }

        let c = mat[(i, j)];
        seen[(i, j)] = true;
        let mut stack = vec![(i, j)];
        let mut area = 1;
        let mut cs = seen.new_uniform(false);
        cs[(i, j)] = true;
        while !stack.is_empty() {
            let (ii, jj) = stack.pop().unwrap();
            let indices = DIJ.iter().filter_map(|&(di, dj)| {
                (mat.get(ii + di, jj + dj) == Some(c)).then_some((ii + di, jj + dj))
            });
            for (iii, jjj) in indices {
                if !seen[(iii, jjj)] {
                    seen[(iii, jjj)] = true;
                    cs[(iii, jjj)] = true;
                    area += 1;
                    stack.push((iii, jjj));
                }
            }
        }

        let cs = &cs;
        let sides = (0..=mat.rows as i32)
            .flat_map(|i| {
                (0..mat.cols as i32)
                    .map(move |j| {
                        cs.get(i - 1, j)
                            .or(Some(false))
                            .cmp(&cs.get(i, j).or(Some(false)))
                    })
                    .dedup()
            })
            .filter(|&o| o.is_ne())
            .count() as u32
            + (0..=mat.cols as i32)
                .flat_map(|j| {
                    (0..mat.rows as i32)
                        .map(move |i| {
                            cs.get(i, j - 1)
                                .or(Some(false))
                                .cmp(&cs.get(i, j).or(Some(false)))
                        })
                        .dedup()
                })
                .filter(|&o| o.is_ne())
                .count() as u32;

        sum += area * sides
    }

    Some(sum)
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
