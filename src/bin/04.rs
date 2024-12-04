advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let rows: Vec<&str> = input.lines().collect();
    let m = rows.len() as i32;
    let n = rows[0].len() as i32;
    let s = input.as_bytes();
    let charat = |i: i32, j: i32, c: u8| s[((n + 1) * i + j) as usize] == c;

    let mut sum: u32 = 0;
    for i in 0..m {
        for j in 0..n {
            for di in -1..=1 {
                for dj in -1..=1 {
                    let ii = i + 3 * di;
                    let jj = j + 3 * dj;
                    sum += ((0..m).contains(&ii)
                        && (0..n).contains(&jj)
                        && charat(i, j, b'X')
                        && charat(i + di, j + dj, b'M')
                        && charat(i + 2 * di, j + 2 * dj, b'A')
                        && charat(ii, jj, b'S')) as u32;
                }
            }
        }
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
