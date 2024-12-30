use itertools::Itertools;

advent_of_code::solution!(2);

fn safe1(levels: &Vec<i32>) -> bool {
    let diff = levels
        .iter()
        .tuple_windows()
        .map(|(m, n)| m - n)
        .collect_vec();
    let inc = diff.iter().all(|d| 1 <= *d && *d <= 3);
    let dec = diff.iter().all(|d| -3 <= *d && *d <= -1);
    inc || dec
}

fn common(input: &str, safe: fn(&Vec<i32>) -> bool) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.split(" ")
                    .map(|elem| elem.parse::<i32>().unwrap())
                    .collect()
            })
            .filter(safe)
            .count() as u32,
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    common(input, safe1)
}

pub fn part_two(input: &str) -> Option<u32> {
    common(input, |levels| {
        (0..levels.len()).any(|i| {
            let mut levels_removed = levels.clone();
            levels_removed.remove(i);
            safe1(&levels_removed)
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
