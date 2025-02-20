use advent_of_code::{Index, Matrix, DIJ, IJ};
use itertools::Itertools;

advent_of_code::solution!(18);

fn parse(input: &str) -> (Vec<IJ>, usize, usize) {
    const BOUNDARY: usize = 100;
    let coordinates = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            IJ((x.parse().unwrap(), y.parse().unwrap()))
        })
        .collect_vec();

    let (size, bytes) = if coordinates.len() < BOUNDARY {
        (7, 12)
    } else {
        (71, 1024)
    };

    (coordinates, size, bytes)
}

fn distance(coordinates: &[IJ], size: usize) -> u32 {
    let mut seen = Matrix::uniform(size, size, false);
    for &IJ((x, y)) in coordinates {
        seen[IJ((y, x))] = true;
    }

    let start = IJ((0, 0));
    let end = IJ((size as Index - 1, size as Index - 1));
    let mut queue = vec![start];
    let mut k = 1;
    seen[start] = true;
    while !queue.is_empty() {
        let mut q = vec![];
        for ij in queue {
            for dij in DIJ {
                let iijj = ij + dij;
                if iijj == end {
                    return k;
                }
                if seen.get(iijj) == Some(false) {
                    seen[iijj] = true;
                    q.push(iijj);
                }
            }
        }
        queue = q;
        k += 1;
    }
    u32::MAX
}

pub fn part_one(input: &str) -> Option<u32> {
    let (coordinates, size, bytes) = parse(input);
    Some(distance(&coordinates[..bytes], size))
}

pub fn part_two(input: &str) -> Option<String> {
    let (coordinates, size, bytes) = parse(input);
    let k = bytes
        + (bytes + 1..coordinates.len())
            .collect_vec()
            .partition_point(|&b| distance(&coordinates[..b], size) < u32::MAX);
    let IJ((i, j)) = coordinates[k];
    Some(format!("{},{}", i, j))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
