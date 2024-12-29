use itertools::Itertools;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
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
    let mut b = b.parse::<u32>().unwrap();
    let mut c = c.parse::<u32>().unwrap();

    let program = program
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .tuples()
        .collect_vec();

    let mut pointer = 0;
    let executor = std::iter::from_fn(|| {
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
    });

    Some(executor.map(|n| n.to_string()).join(","))
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
