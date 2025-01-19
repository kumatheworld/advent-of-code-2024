use advent_of_code::Matrix;
use itertools::{iproduct, Itertools};
use std::collections::HashMap;

advent_of_code::solution!(21);

pub fn common(input: &str, num_intermediate_robots: usize) -> Option<u64> {
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
                line[..line.len() - 1].parse::<u64>().unwrap() * {
                    let mut counter = HashMap::<(u8, u8), u64>::new();
                    std::iter::once(b'A')
                        .chain(
                            std::iter::once(b'A')
                                .chain(line.bytes())
                                .tuple_windows()
                                .flat_map(|ab| get_directions(ab, &nk)),
                        )
                        .tuple_windows()
                        .for_each(|ab| *counter.entry(ab).or_insert(0) += 1);

                    for _ in 0..num_intermediate_robots {
                        let mut counter_new = HashMap::<(u8, u8), u64>::new();
                        counter.into_iter().for_each(|(k, v)| {
                            std::iter::once(b'A')
                                .chain(kk2ks.get(&k).unwrap().into_iter().map(|&a| a))
                                .tuple_windows()
                                .for_each(|ab| *counter_new.entry(ab).or_insert(0) += v)
                        });
                        counter = counter_new;
                    }
                    counter.values().sum::<u64>()
                }
            })
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    common(input, 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    common(input, 25)
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
