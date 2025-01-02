use core::unreachable;

use advent_of_code::Matrix;
use itertools::Itertools;

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

    let directions = directions.chars().filter_map(|c| match c {
        '^' => Some((-1, 0)),
        '>' => Some((0, 1)),
        'v' => Some((1, 0)),
        '<' => Some((0, -1)),
        '\n' => None,
        _ => unreachable!(),
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
            _ => unreachable!(),
        }
        k += 1;
    }

    for (k0, k1) in (0..=k).rev().tuple_windows() {
        mat.swap((i + k0 * di, j + k0 * dj), (i + k1 * di, j + k1 * dj));
    }

    (i + di, j + dj)
}

pub fn part_one(input: &str) -> Option<u32> {
    common(input, Matrix::from, push_line, vec![b'O'])
}

pub fn part_two(input: &str) -> Option<u32> {
    common(
        input,
        |warehouse| {
            let array = warehouse
                .chars()
                .filter_map(|b| match b {
                    '#' => Some(vec![b'#', b'#']),
                    'O' => Some(vec![b'[', b']']),
                    '.' => Some(vec![b'.', b'.']),
                    '@' => Some(vec![b'@', b'.']),
                    '\n' => None,
                    _ => unreachable!(),
                })
                .flatten()
                .collect_vec()
                .into_boxed_slice();
            let rows = warehouse.lines().count();
            let cols = array.len() / rows;
            Matrix { array, rows, cols }
        },
        |mat, (i, j), (di, dj), box_bytes| {
            if di == 0 {
                push_line(mat, (i, j), (di, dj), box_bytes)
            } else {
                let mut jss = vec![];
                let mut k = 1;

                let mut js = vec![j];
                while !js.is_empty() {
                    jss.push(js.clone());
                    let mut js2 = vec![];
                    for jj in js.into_iter() {
                        match mat.get((i + k * di, jj)) {
                            Some(b'.') => (),
                            Some(b'[') => js2.extend([jj, jj + 1].iter()),
                            Some(b']') => {
                                if js2.last() != Some(&jj) {
                                    js2.extend([jj - 1, jj].iter())
                                }
                            }
                            Some(b'#') => return (i, j),
                            _ => unreachable!(),
                        }
                    }
                    js = js2;
                    k += 1;
                }

                let mut ii = i + k * di;
                for js in jss.into_iter().rev() {
                    ii -= di;
                    for jj in js {
                        mat.swap((ii - di, jj), (ii, jj));
                    }
                }

                (ii, j)
            }
        },
        vec![b'[', b']'],
    )
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
        assert_eq!(result, Some(9021));
    }
}
