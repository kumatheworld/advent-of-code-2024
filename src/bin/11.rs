use itertools::Itertools;

advent_of_code::solution!(11);

pub fn common(input: &str, reps: usize) -> Option<u64> {
    // dp[n][d] is the number of stones the stone d gets after n blinks
    let mut dp = vec![
        [1; 10],
        [1; 10],
        [1, 2, 2, 2, 2, 1, 1, 1, 1, 1],
        [2, 4, 4, 4, 4, 2, 2, 2, 2, 2],
        [4; 10],
    ];
    for n in 5..=reps {
        dp.push([
            dp[n - 1][1],                                              // 0 -> 1 in 1 blink
            dp[n - 3][2] + dp[n - 3][0] + dp[n - 3][2] + dp[n - 3][4], // 1 -> 1 * 2024 = 2024, split in 3 blinks
            dp[n - 3][4] + dp[n - 3][0] + dp[n - 3][4] + dp[n - 3][8], // 2 -> 2 * 2024 = 4048, split in 3 blinks
            dp[n - 3][6] + dp[n - 3][0] + dp[n - 3][7] + dp[n - 3][2], // 3 -> 3 * 2024 = 6072, split in 3 blinks
            dp[n - 3][8] + dp[n - 3][0] + dp[n - 3][9] + dp[n - 3][6], // 4 -> 4 * 2024 = 8096, split in 3 blinks
            dp[n - 5][2]
                + dp[n - 5][0]
                + dp[n - 5][4]
                + dp[n - 5][8]
                + dp[n - 5][2]
                + dp[n - 5][8]
                + dp[n - 5][8]
                + dp[n - 5][0], // 5 -> 5 * 2024^2 = 20482880, split in 5 blinks
            dp[n - 5][2]
                + dp[n - 5][4]
                + dp[n - 5][5]
                + dp[n - 5][7]
                + dp[n - 5][9]
                + dp[n - 5][4]
                + dp[n - 5][5]
                + dp[n - 5][6], // 6 -> 6 * 2024^2 = 24579456, split in 5 blinks
            dp[n - 5][2]
                + dp[n - 5][8]
                + dp[n - 5][6]
                + dp[n - 5][7]
                + dp[n - 5][6]
                + dp[n - 5][0]
                + dp[n - 5][3]
                + dp[n - 5][2], // 7 -> 7 * 2024^2 = 28676032, split in 5 blinks
            dp[n - 5][3]
                + dp[n - 5][2]
                + dp[n - 5][7]
                + dp[n - 5][7]
                + dp[n - 5][2]
                + dp[n - 5][6]
                + dp[n - 4][8], // 8 -> 8 * 2024^2 = 32772608, split in 5 blinks, but 08 is reduced to 8 in the 4th blink
            dp[n - 5][3]
                + dp[n - 5][6]
                + dp[n - 5][8]
                + dp[n - 5][6]
                + dp[n - 5][9]
                + dp[n - 5][1]
                + dp[n - 5][8]
                + dp[n - 5][4], // 9 -> 9 * 2024^2 = 36869184, split in 5 blinks
        ])
    }

    let mut buf = input
        .trim()
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();
    let mut sum: u64 = 0;

    for n in (1..=reps).rev() {
        buf = buf
            .into_iter()
            .flat_map(|d| {
                if d < 10 {
                    sum += dp[n][d];
                    vec![]
                } else {
                    let num_digits = d.to_string().len();
                    if num_digits & 1 == 0 {
                        let divisor = 10usize.pow(num_digits as u32 >> 1);
                        vec![d / divisor, d % divisor]
                    } else {
                        vec![d * 2024]
                    }
                }
            })
            .collect_vec()
    }

    Some(sum + buf.len() as u64)
}

pub fn part_one(input: &str) -> Option<u64> {
    common(input, 25)
}

pub fn part_two(input: &str) -> Option<u64> {
    common(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
