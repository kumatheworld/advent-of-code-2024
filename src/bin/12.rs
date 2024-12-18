use advent_of_code::Matrix;

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
    None
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
