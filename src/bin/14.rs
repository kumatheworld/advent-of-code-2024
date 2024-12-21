use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(14);

fn common(input: &str) -> ((i32, i32), impl Iterator<Item = (i32, i32, i32, i32)> + '_) {
    const BOUNDARY: usize = 1000;
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)$").unwrap();

    let size = if input.len() < BOUNDARY {
        (7, 11)
    } else {
        (103, 101)
    };

    let ppvvs = input.lines().map(move |line| {
        re.captures(line)
            .unwrap()
            .iter()
            .skip(1)
            .map(|x| x.unwrap().as_str().parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap()
    });

    (size, ppvvs)
}

pub fn part_one(input: &str) -> Option<u32> {
    const SECONDS: i32 = 100;
    let ((rows, cols), ppvvs) = common(input);

    Some(
        ppvvs
            .filter_map(|(px, py, vx, vy)| {
                let x = (px + SECONDS * vx).rem_euclid(cols).cmp(&(cols >> 1));
                let y = (py + SECONDS * vy).rem_euclid(rows).cmp(&(rows >> 1));
                (![x, y].contains(&std::cmp::Ordering::Equal)).then_some((x, y))
            })
            .counts()
            .values()
            .map(|&v| v as u32)
            .product(),
    )
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
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        part_two(&advent_of_code::template::read_file("examples", DAY));
    }
}
