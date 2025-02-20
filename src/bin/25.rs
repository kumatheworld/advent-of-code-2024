advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u32> {
    // Basically copy the great solution found below:
    // https://www.reddit.com/r/adventofcode/comments/1hlu4ht/comment/m3tb8y0/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
    let (keys, locks): (Vec<u64>, Vec<u64>) = input
        .split("\n\n")
        .map(|schematic| {
            schematic
                .bytes()
                .fold(0, |acc, b| acc << 1 | (b == b'#') as u64)
        })
        .partition(|x| x & 1 == 1);
    Some(
        keys.into_iter()
            .map(|key| locks.iter().filter(|&lock| key & lock == 0).count() as u32)
            .sum(),
    )
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
