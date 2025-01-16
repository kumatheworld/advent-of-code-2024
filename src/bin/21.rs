use advent_of_code::Matrix;
use itertools::{iproduct, Itertools};
use std::collections::HashMap;
use std::iter;

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u32> {
    const NUM_INTERMEDIATE_ROBOTS: usize = 2;

    let d2ks: HashMap<(i32, i32), [Vec<u8>; 2]> = iproduct!(-3..=3, -2..=2)
        .map(|(di, dj): (i32, i32)| {
            ((di, dj), {
                let is = vec![if di > 0 { b'v' } else { b'^' }; di.abs() as usize];
                let js = vec![if dj > 0 { b'>' } else { b'<' }; dj.abs() as usize];
                let a = vec![b'A'];
                [[&is[..], &js[..], &a[..]].concat(), [js, is, a].concat()]
            })
        })
        .collect();

    let get_directions = move |(a, b): (u8, u8), keymap: &Matrix<u8>| {
        let (ai, aj) = keymap.position(a).unwrap();
        let (bi, bj) = keymap.position(b).unwrap();
        let ds = d2ks.get(&(bi - ai, bj - aj)).unwrap();
        match (keymap.get((bi, aj)), keymap.get((ai, bj))) {
            (Some(b' '), Some(_)) => ds[1].clone(),
            (Some(_), Some(b' ')) => ds[0].clone(),
            (Some(_), Some(_)) => ds[(aj > bj) as usize].clone(), // This turns out working
            _ => unreachable!(),
        }
    };

    let dk = Matrix::from(" ^A\n<v>");
    let dks = "<v>^A";
    let kk2ks: HashMap<(u8, u8), Vec<u8>> = iproduct!(dks.bytes(), dks.bytes())
        .map(|ab| (ab, get_directions(ab, &dk)))
        .collect();

    let nk = Matrix::from("789\n456\n123\n 0A");
    Some(
        input
            .lines()
            .map(|line| {
                line[..line.len() - 1].parse::<u32>().unwrap() * {
                    let mut keys: Box<dyn Iterator<Item = u8>> = Box::new(
                        iter::once(b'A')
                            .chain(line.bytes())
                            .tuple_windows()
                            .flat_map(|ab| get_directions(ab, &nk)),
                    );
                    for _ in 0..NUM_INTERMEDIATE_ROBOTS {
                        keys = Box::new(
                            iter::once(b'A')
                                .chain(keys)
                                .tuple_windows()
                                .flat_map(|ab| kk2ks.get(&ab).unwrap().clone()),
                        );
                    }
                    keys.count() as u32
                }
            })
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
        assert_eq!(result, Some(126384));
    }
}
