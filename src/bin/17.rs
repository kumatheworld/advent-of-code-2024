use itertools::EitherOrBoth::Both;
use itertools::Itertools;

advent_of_code::solution!(17);

fn common(
    input: &str,
) -> (
    Box<dyn FnMut(Vec<(u8, u8)>, Option<u32>) -> Box<dyn Iterator<Item = u8>>>,
    Vec<(u8, u8)>,
) {
    let re = regex::Regex::new(
        r"^Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: (.*)$",
    )
    .unwrap();
    let (a, b, c, program) = re
        .captures(input.trim())
        .unwrap()
        .iter()
        .skip(1)
        .map(|x| x.unwrap().as_str())
        .collect_tuple()
        .unwrap();
    let mut a = a.parse::<u32>().unwrap();
    let b0 = b.parse::<u32>().unwrap();
    let c0 = c.parse::<u32>().unwrap();

    let executor = Box::new(move |program: Vec<(u8, u8)>, a0: Option<u32>| {
        if let Some(a0) = a0 {
            a = a0;
        }
        let mut b = b0;
        let mut c = c0;
        let mut pointer = 0;
        Box::new(std::iter::from_fn(move || {
            while pointer < program.len() {
                let (opcode, literal) = program[pointer];
                pointer += 1;
                let combo = match literal {
                    n if n < 4 => n as u32,
                    4 => a,
                    5 => b,
                    6 => c,
                    _ => unreachable!(),
                };
                match opcode {
                    0 => a >>= combo,
                    1 => b ^= literal as u32,
                    2 => b = combo % 8,
                    3 => {
                        if a != 0 {
                            pointer = (literal >> 1) as usize;
                            continue;
                        }
                    }
                    4 => b ^= c,
                    5 => return Some((combo % 8) as u8),
                    6 => b = a >> combo,
                    7 => c = a >> combo,
                    _ => unreachable!(),
                };
            }
            None
        })) as Box<dyn Iterator<Item = u8>>
    });

    let program = program
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .tuples()
        .collect_vec();

    (executor, program)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut executor, program) = common(input);
    Some(executor(program, None).map(|n| n.to_string()).join(","))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut executor, program) = common(input);
    let sequence = program.iter().map(|&(i, j)| [i, j]).flatten().collect_vec();

    if let &[2, 4, 1, n1, 7, 5, 1, n3, 4, 1, 5, 5, 0, 3, 3, 0] = sequence.as_slice() {
        // It turns out that the program repeatedly outputs (n1 ^ n2 ^ (a % 8) ^ (a >> (n1 ^ (a % 8)))) % 8,
        // after each of which a >>= 3 is performed and the program exits if a is zero.
        // We search for a0 from the top to the bottom, dividing it into chunks of 3 bits.
        fn search(a: u64, n1: u8, n3: u8, seq: &[u8]) -> Option<u64> {
            match seq.split_last() {
                None => Some(a),
                Some((&last, rest)) => {
                    let a8 = a << 3;
                    (0..8).find_map(|m| {
                        let a8m = a8 + m as u64;
                        if (n1 ^ n3 ^ m ^ ((a8m >> (n1 ^ m)) & 7) as u8) == last {
                            search(a8m as u64, n1, n3, rest)
                        } else {
                            None
                        }
                    })
                }
            }
        }
        return search(0, n1, n3, &sequence);
    }

    (0..)
        .filter_map(|a0| {
            executor(program.clone(), Some(a0)) // Can the cloning be avoided?
                .zip_longest(&sequence)
                .all(|p| matches!(p, Both(m,n) if m == *n))
                .then_some(a0 as u64)
        })
        .next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some("0,1,2".to_string()));
    }

    #[test]
    fn test_part_one_3() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some("4,2,5,6,7,7,7,7,3,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(117440));
    }
}
