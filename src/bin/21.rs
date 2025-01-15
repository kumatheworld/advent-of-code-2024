use advent_of_code::Matrix;
use itertools::{iproduct, Itertools};
use std::collections::HashMap;
use std::iter;

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u32> {
    const NUM_INTERMEDIATE_ROBOTS: usize = 2;

    let d2ks: HashMap<(i32, i32), Vec<Vec<u8>>> = iproduct!(-3..=3, -2..=2)
        .map(|(di, dj): (i32, i32)| {
            ((di, dj), {
                let is = vec![if di > 0 { b'v' } else { b'^' }; di.abs() as usize];
                let js = vec![if dj > 0 { b'>' } else { b'<' }; dj.abs() as usize];
                let a = vec![b'A'];
                if di == 0 || dj == 0 {
                    vec![[is, js, a].concat()]
                } else {
                    vec![
                        [is.clone(), js.clone(), a.clone()].concat(),
                        [js, is, a].concat(),
                    ]
                }
            })
        })
        .collect();

    let get_directions = move |(a, b): (u8, u8), keymap: &Matrix<u8>| {
        let (ai, aj) = keymap.position(a).unwrap();
        let (bi, bj) = keymap.position(b).unwrap();
        let ds = d2ks.get(&(bi - ai, bj - aj)).unwrap();
        match (keymap.get((bi, aj)), keymap.get((ai, bj))) {
            (Some(b' '), Some(_)) => ds[1..].to_vec(),
            (Some(_), Some(b' ')) => ds[..1].to_vec(),
            (Some(_), Some(_)) => ds.clone(),
            _ => unreachable!(),
        }
    };

    let dk = Matrix::from(" ^A\n<v>");
    let dks = "<v>^A";
    let kk2ks: HashMap<(u8, u8), Vec<Vec<u8>>> = iproduct!(dks.bytes(), dks.bytes())
        .map(|ab| (ab, get_directions(ab, &dk)))
        .collect();

    let nk = Matrix::from("789\n456\n123\n 0A");
    Some(
        input
            .lines()
            .map(|line| {
                line[..line.len() - 1].parse::<u32>().unwrap() * {
                    let mut keys: Vec<Vec<u8>> = iter::once(b'A')
                        .chain(line.bytes())
                        .tuple_windows()
                        .map(|ab| get_directions(ab, &nk))
                        .multi_cartesian_product()
                        .map(|v| v.into_iter().flatten().collect())
                        .collect();
                    for _ in 0..NUM_INTERMEDIATE_ROBOTS {
                        keys = keys
                            .iter()
                            .map(|ks| {
                                iter::once(b'A')
                                    .chain(ks.clone())
                                    .tuple_windows()
                                    .flat_map(|ab| kk2ks.get(&ab).unwrap()[0].clone())
                                    .collect()
                                // This turns out working when NUM_INTERMEDIATE_ROBOTS <= 2
                            })
                            .collect();
                    }
                    keys.iter().map(|ks| ks.len() as u32).min().unwrap()
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
