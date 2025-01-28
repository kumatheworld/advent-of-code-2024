use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(24);

fn parse_wires_and_gates(
    input: &str,
) -> (HashMap<&str, Option<bool>>, Vec<(&str, &str, &str, &str)>) {
    let (xys, gates) = input.split_once("\n\n").unwrap();

    let mut wires = HashMap::<&str, Option<bool>>::new();
    for xy in xys.lines() {
        let (key, value) = xy.split_once(": ").unwrap();
        let value = match value {
            "0" => false,
            "1" => true,
            _ => unreachable!(),
        };
        wires.insert(key, Some(value));
    }

    let gates = gates
        .lines()
        .map(|line| {
            let (w0, op, w1, _, w2) = line.split(' ').collect_tuple().unwrap();
            wires.entry(w0).or_insert(None);
            wires.entry(w1).or_insert(None);
            wires.entry(w2).or_insert(None);
            (w0, op, w1, w2)
        })
        .collect_vec();

    (wires, gates)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut wires, mut gates) = parse_wires_and_gates(input);

    while !gates.is_empty() {
        gates.retain(|&(w0, op, w1, w2)| {
            if let (Some(b0), Some(b1)) = (*wires.get(&w0).unwrap(), *wires.get(&w1).unwrap()) {
                *wires.get_mut(&w2).unwrap() = Some(match op {
                    "AND" => b0 && b1,
                    "XOR" => b0 != b1,
                    "OR" => b0 || b1,
                    _ => unreachable!(),
                });
                false
            } else {
                true
            }
        })
    }

    Some(
        wires
            .iter()
            .filter(|(w, _)| w.as_bytes()[0] == b'z')
            .sorted()
            .enumerate()
            .fold(
                0,
                |acc, (i, (_, &b))| if b.unwrap() { (1 << i) | acc } else { acc },
            ),
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
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2024));
    }
}
