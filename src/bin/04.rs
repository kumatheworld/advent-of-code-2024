use advent_of_code::Matrix;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mat = Matrix::from(input);

    let mut sum: u32 = 0;
    for i in 0..mat.rows as i32 {
        for j in 0..mat.cols as i32 {
            for di in -1..=1 {
                for dj in -1..=1 {
                    // Check 'S' first to avoid out-of-bounds errors
                    sum += (mat.get(i + 3 * di, j + 3 * dj) == Some(b'S')
                        && mat[(i + 2 * di, j + 2 * dj)] == b'A'
                        && mat[(i + di, j + dj)] == b'M'
                        && mat[(i, j)] == b'X') as u32
                }
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rows: Vec<&str> = input.lines().collect();
    let m = rows.len();
    let n = rows[0].len();
    let s = input.as_bytes();
    let mmss = vec![b'M', b'M', b'S', b'S'];
    let charat = |i: usize, j: usize| s[(n + 1) * i + j];

    let mut sum: u32 = 0;
    for i in 1..(m - 1) {
        for j in 1..(n - 1) {
            if charat(i, j) != b'A' {
                continue;
            }

            let mut edges = vec![
                charat(i - 1, j - 1),
                charat(i - 1, j + 1),
                charat(i + 1, j - 1),
                charat(i + 1, j + 1),
            ];
            if edges[0] == edges[3] {
                continue;
            }

            edges.sort();
            if edges == mmss {
                sum += 1;
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
