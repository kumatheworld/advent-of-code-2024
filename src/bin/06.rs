use advent_of_code::Matrix;
use itertools::iproduct;

advent_of_code::solution!(6);

fn patrol(mat: &mut Matrix<u8>, i0: i32, j0: i32) -> Option<u32> {
    const DI: [i32; 4] = [-1, 0, 1, 0];
    const DJ: [i32; 4] = [0, 1, 0, -1];
    let mut d = 0;
    let mut i = i0;
    let mut j = j0;

    mat[(i, j)] = b'X';
    let mut sum = 1;
    // mat.rows * mat.cols times should be enough to see if there's a loop
    for _ in 0..mat.rows * mat.cols {
        i += DI[d];
        j += DJ[d];
        match mat.get(i, j) {
            Some(b'.') => {
                sum += 1;
                mat[(i, j)] = b'X';
            }
            Some(b'#') => {
                i -= DI[d];
                j -= DJ[d];
                d = (d + 1) % 4;
            }
            Some(b'X') => continue,
            Some(_) => unreachable!(),
            None => return Some(sum),
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut mat = Matrix::from(input);
    let (i0, j0) = mat.position(b'^').unwrap();
    patrol(&mut mat, i0, j0)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mat0 = Matrix::from(input);
    let (i0, j0) = mat0.position(b'^').unwrap();
    let mut mat1 = mat0.clone();
    patrol(&mut mat1, i0, j0);

    Some(
        iproduct!(0..mat0.rows as i32, 0..mat0.cols as i32)
            .filter(|&(ii, jj)| {
                mat1[(ii, jj)] == b'X' && (ii != i0 || jj != j0) && {
                    let mut mat = mat0.clone();
                    mat[(ii, jj)] = b'#';
                    patrol(&mut mat, i0, j0).is_none()
                }
            })
            .count() as u32,
    )
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
