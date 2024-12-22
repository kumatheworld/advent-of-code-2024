use core::panic;

use advent_of_code::Matrix;

advent_of_code::solution!(15);

fn common(
    input: &str,
    load_txt: fn(&str) -> Matrix<u8>,
    push: fn(&mut Matrix<u8>, (i32, i32), (i32, i32), &Vec<u8>) -> (i32, i32),
    box_bytes: Vec<u8>,
) -> Option<u32> {
    const MULTIPLIER: i32 = 100;

    let (warehouse, directions) = input.split_once("\n\n").unwrap();

    let mut mat = load_txt(warehouse);
    let mut ij = mat.position(b'@').unwrap();
    mat[ij] = b'.';

    let directions = directions.chars().filter_map(|c| match c {
        '^' => Some((-1, 0)),
        '>' => Some((0, 1)),
        'v' => Some((1, 0)),
        '<' => Some((0, -1)),
        '\n' => None,
        _ => panic!(),
    });

    for dij in directions {
        ij = push(&mut mat, ij, dij, &box_bytes);
    }

    Some(
        mat.indices()
            .filter_map(|(i, j)| (mat[(i, j)] == box_bytes[0]).then(|| (MULTIPLIER * i + j) as u32))
            .sum(),
    )
}

fn push_line(
    mat: &mut Matrix<u8>,
    (i, j): (i32, i32),
    (di, dj): (i32, i32),
    box_bytes: &Vec<u8>,
) -> (i32, i32) {
    let mut k = 1;
    loop {
        match mat[(i + k * di, j + k * dj)] {
            b'.' => break,
            b if box_bytes.contains(&b) => (),
            b'#' => return (i, j),
            _ => panic!(),
        }
        k += 1;
    }

    mat.swap((i + di, j + dj), (i + k * di, j + k * dj));
    (i + di, j + dj)
}

pub fn part_one(input: &str) -> Option<u32> {
    common(input, Matrix::from, push_line, vec![b'O'])
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9201));
    }
}
