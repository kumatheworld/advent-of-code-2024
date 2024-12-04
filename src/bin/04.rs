advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let rows: Vec<&str> = input.lines().collect();
    let m = rows.len() as i32;
    let n = rows[0].len() as i32;
    let s = input.as_bytes();
    let charatis = |i: i32, j: i32, c: u8| s[((n + 1) * i + j) as usize] == c;

    let mut sum: u32 = 0;
    for i in 0..m {
        for j in 0..n {
            for di in -1..=1 {
                for dj in -1..=1 {
                    let ii = i + 3 * di;
                    let jj = j + 3 * dj;
                    sum += ((0..m).contains(&ii)
                        && (0..n).contains(&jj)
                        && charatis(i, j, b'X')
                        && charatis(i + di, j + dj, b'M')
                        && charatis(i + 2 * di, j + 2 * dj, b'A')
                        && charatis(ii, jj, b'S')) as u32;
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
