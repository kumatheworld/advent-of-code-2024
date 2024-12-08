use advent_of_code::Matrix;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut mat = Matrix::from(input);
    let (mut i, mut j) = mat.find(b'^').unwrap();

    let di = [-1, 0, 1, 0];
    let dj = [0, 1, 0, -1];
    let mut d = 0;

    mat[(i, j)] = b'X';
    let mut sum: u32 = 1;
    loop {
        i += di[d];
        j += dj[d];
        match mat.get(i, j) {
            Some(b'.') => {
                sum += 1;
                mat[(i, j)] = b'X';
            }
            Some(b'#') => {
                i -= di[d];
                j -= dj[d];
                d = (d + 1) % 4;
            }
            Some(b'X') => continue,
            Some(_) => panic!(),
            None => break,
        }
    }
    Some(sum)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
