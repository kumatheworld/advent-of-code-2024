use core::panic;

use advent_of_code::Matrix;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    const MULTIPLIER: i32 = 100;

    let (warehouse, directions) = input.split_once("\n\n").unwrap();

    let mut mat = Matrix::from(warehouse);
    let (mut i, mut j) = mat.position(b'@').unwrap();
    mat[(i, j)] = b'.';

    let directions = directions.chars().filter_map(|c| match c {
        '^' => Some((-1, 0)),
        '>' => Some((0, 1)),
        'v' => Some((1, 0)),
        '<' => Some((0, -1)),
        '\n' => None,
        _ => panic!(),
    });

    'outer: for (di, dj) in directions {
        let mut k = 1;
        loop {
            match mat.get(i + k * di, j + k * dj) {
                Some(b'.') => break,
                Some(b'O') => (),
                Some(b'#') => continue 'outer,
                _ => panic!(),
            }
            k += 1;
        }

        mat.swap((i + di, j + dj), (i + k * di, j + k * dj));
        i = i + di;
        j = j + dj;
    }

    Some(
        mat.indices()
            .filter_map(|(i, j)| (mat[(i, j)] == b'O').then(|| (MULTIPLIER * i + j) as u32))
            .sum(),
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
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
