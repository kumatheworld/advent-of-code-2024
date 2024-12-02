advent_of_code::solution!(2);

fn safe(levels: &Vec<i32>) -> bool {
    let mut prev = levels[0];
    let mut diff_prev = 0;
    for &level in levels.iter().skip(1) {
        let diff = level - prev;
        if diff == 0 || diff.abs() > 3 || diff * diff_prev < 0 {
            return false;
        }
        prev = level;
        diff_prev = diff;
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let levels: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        if safe(&levels) {
            sum += 1
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let levels: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        for i in 0..levels.len() {
            let mut levels_removed = levels.clone();
            levels_removed.remove(i);
            if safe(&levels_removed) {
                sum += 1;
                break;
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
