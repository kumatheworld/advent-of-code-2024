use advent_of_code::Matrix;
use itertools::Itertools;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u32> {
    const BOUNDARY: usize = 100;
    const DIJ: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let coordinates = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .collect_vec();

    let (size, bytes) = if coordinates.len() < BOUNDARY {
        (7, 12)
    } else {
        (71, 1024)
    };

    let mut seen = Matrix::uniform(size, size, false);
    for &(x, y) in &coordinates[..bytes] {
        seen[(y, x)] = true;
    }
    print!("{}", seen);

    let start = (0, 0);
    let end = (size as i32 - 1, size as i32 - 1);
    let mut queue = vec![start];
    seen[start] = true;
    for k in 1.. {
        let mut q = vec![];
        for (i, j) in queue {
            for (di, dj) in DIJ {
                let iijj = (i + di, j + dj);
                if iijj == end {
                    return Some(k);
                }
                if seen.get(iijj) == Some(false) {
                    seen[iijj] = true;
                    q.push(iijj);
                }
            }
        }
        queue = q;
    }
    None
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
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
